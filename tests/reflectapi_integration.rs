#![cfg(feature = "reflectapi")]

use std::{io::Write as _, time::Duration};

use hotspot::{Coordinate, Hotspot, repr::PixelRepr};
use reflectapi::codegen::rust::Config;
use tokio::time::timeout;

// Include the generated client
#[allow(dead_code, unused_imports, clippy::all)]
mod generated_client {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/generated_client.rs"
    ));
}

/// HTTP client implementation for the generated client
struct TestHttpClient {
    client: reqwest::Client,
}

impl reflectapi::rt::Client for TestHttpClient {
    type Error = reqwest::Error;

    async fn request(
        &self,
        url: reflectapi::rt::Url,
        input: bytes::Bytes,
        _headers: http::HeaderMap,
    ) -> Result<(http::StatusCode, bytes::Bytes), Self::Error> {
        let response = self
            .client
            .post(url.as_str())
            .header("content-type", "application/json")
            .body(input)
            .send()
            .await?;

        let status = response.status();
        let body = response.bytes().await?;
        Ok((status, body))
    }
}

impl Clone for TestHttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

/// Build the reflectapi schema and router for testing
fn build_test_api() -> (reflectapi::Schema, Vec<reflectapi::Router<()>>) {
    async fn echo_coordinate(
        _: (),
        request: Coordinate,
        _headers: reflectapi::Empty,
    ) -> Coordinate {
        request
    }

    async fn echo_hotspot(
        _: (),
        request: Hotspot<PixelRepr>,
        _headers: reflectapi::Empty,
    ) -> Hotspot<PixelRepr> {
        request
    }

    reflectapi::Builder::new()
        .name("Hotspots Test API")
        .description("Test API for validating reflectapi serialization")
        .route(echo_coordinate, |b| {
            b.name("echo_coordinate")
                .description("Echoes back the provided coordinate")
        })
        .route(echo_hotspot, |b| {
            b.name("echo_hotspot")
                .description("Echoes back the provided hotspot")
        })
        .build()
        .expect("Failed to build reflectapi schema")
}

/// Format the provided code using `rustfmt`.
fn rustfmt(code: &str) -> String {
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin
        .write_all(code.as_bytes())
        .expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");

    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}

#[test]
fn test_snapshot_rust_client() {
    let (schema, _router) = build_test_api();

    let config = Config::default();
    let generated = reflectapi::codegen::rust::generate(schema, &config)
        .expect("Failed to generate Rust client");

    // Write the generated client to file for use by other tests
    // Remove inner attributes that don't work with include! macro
    let clean_code = generated
        .replace("#![allow(non_camel_case_types)]", "")
        .replace("#![allow(dead_code)]", "");

    let wrapped_code = format!(
        "#[cfg(feature = \"reflectapi\")]\nmod internal {{\n{}\n}}\n#[cfg(feature = \"reflectapi\")]\n\npub use internal::*;",
        clean_code
    );

    let generated_for_file = rustfmt(&wrapped_code);

    let client_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("generated_client.rs");

    let mut file =
        std::fs::File::create(&client_path).expect("Failed to create generated_client.rs");
    file.write_all(generated_for_file.as_bytes())
        .expect("Failed to write generated client");

    insta::assert_snapshot!("rust_client", generated);
}

#[test]
fn test_snapshot_openapi_spec() {
    let (schema, _router) = build_test_api();

    let openapi_spec = reflectapi::codegen::openapi::Spec::from(&schema);
    let spec_json =
        serde_json::to_string_pretty(&openapi_spec).expect("Failed to serialize OpenAPI spec");

    insta::assert_snapshot!("openapi_spec", spec_json);
}

async fn spawn_test_server() -> (tokio::task::JoinHandle<()>, String) {
    let (_schema, routers) = build_test_api();

    // Convert reflectapi routers to axum router
    let app = reflectapi::axum::into_router((), routers, |_name, router| router);

    // Start the server on a random port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to address");
    let addr = listener.local_addr().expect("Failed to get local address");

    // Spawn the server in the background
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .expect("Server failed to start");
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (server_handle, format!("http://{}", addr))
}

#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(not(feature = "reflectapi"), ignore = "reflectapi feature not enabled")]
async fn test_echo_coordinate() {
    let (server_handle, base_url) = spawn_test_server().await;

    let test_result = timeout(Duration::from_secs(5), async {
        // Create the generated client
        let http_client = TestHttpClient {
            client: reqwest::Client::new(),
        };

        let base_url = reflectapi::rt::Url::parse(&base_url).expect("Failed to parse base URL");
        let client = generated_client::Interface::try_new(http_client, base_url)
            .expect("Failed to create generated client");

        // Test using generated client
        let test_coord = (10, 20);

        let echoed = client
            .echo_coordinate(test_coord, reflectapi::Empty {})
            .await
            .expect("Failed to call echo_coordinate via generated client");

        assert_eq!(echoed.0, 10);
        assert_eq!(echoed.1, 20);
    })
    .await;

    assert!(
        test_result.is_ok(),
        "Test timed out - server may not be responding"
    );

    server_handle.abort();
}

#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(not(feature = "reflectapi"), ignore = "reflectapi feature not enabled")]
async fn test_echo_hotspot() {
    let (server_handle, base_url) = spawn_test_server().await;

    let test_result = timeout(Duration::from_secs(5), async {
        // Create the generated client
        let http_client = TestHttpClient {
            client: reqwest::Client::new(),
        };

        let base_url = reflectapi::rt::Url::parse(&base_url).expect("Failed to parse base URL");
        let client = generated_client::Interface::try_new(http_client, base_url)
            .expect("Failed to create generated client");

        // Test using generated client and generated types
        let test_hotspot = generated_client::types::Hotspot {
            x1: 50,
            y1: 60,
            x2: 150,
            y2: 160,
        };

        let echoed = client
            .echo_hotspot(test_hotspot, reflectapi::Empty {})
            .await
            .expect("Failed to call echo_hotspot via generated client");

        assert_eq!(echoed.x1, 50);
        assert_eq!(echoed.y1, 60);
        assert_eq!(echoed.x2, 150);
        assert_eq!(echoed.y2, 160);
    })
    .await;

    assert!(
        test_result.is_ok(),
        "Test timed out - server may not be responding"
    );

    server_handle.abort();
}

#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(not(feature = "reflectapi"), ignore = "reflectapi feature not enabled")]
async fn test_generated_client_integration() {
    // Start the test server
    let (server_handle, base_url) = spawn_test_server().await;

    let test_result = timeout(Duration::from_secs(10), async {
        // Create the generated client
        let http_client = TestHttpClient {
            client: reqwest::Client::new(),
        };

        let base_url = reflectapi::rt::Url::parse(&base_url).expect("Failed to parse base URL");
        let client = generated_client::Interface::try_new(http_client, base_url)
            .expect("Failed to create generated client");

        // Test 1: Echo hotspot using generated client
        let test_hotspot = generated_client::types::Hotspot {
            x1: 100,
            y1: 200,
            x2: 300,
            y2: 400,
        };

        let echoed = client
            .echo_hotspot(test_hotspot, reflectapi::Empty {})
            .await
            .expect("Failed to call echo_hotspot via generated client");

        assert_eq!(echoed.x1, 100);
        assert_eq!(echoed.y1, 200);
        assert_eq!(echoed.x2, 300);
        assert_eq!(echoed.y2, 400);

        println!("✓ Generated client successfully called echo_hotspot");

        // Test 2: Echo coordinate using generated client
        let test_coord = (150, 250);

        let echoed_coord = client
            .echo_coordinate(test_coord, reflectapi::Empty {})
            .await
            .expect("Failed to call echo_coordinate via generated client");

        assert_eq!(echoed_coord.0, 150);
        assert_eq!(echoed_coord.1, 250);

        println!("✓ Generated client successfully called echo_coordinate");
    })
    .await;

    server_handle.abort();

    assert!(
        test_result.is_ok(),
        "Test timed out - generated client may not be working correctly"
    );
}
