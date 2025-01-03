mod auth_tests;
pub mod quiz_tests;

use crate::common::TestContext;
use crate::common::server::setup_test_server; // Updated import

#[allow(dead_code)]
pub async fn setup() -> String { // Ensure this function is public
    setup_test_server()
        .await
        .expect("Failed to start test server")
}

#[allow(dead_code)]
pub async fn setup_test_context() -> TestContext {
    let ctx = TestContext::new().await;
    // Ensure server is ready
    for _ in 0..5 {
        if ctx
            .api_client
            .get(&format!("{}/health", ctx.base_url))
            .send()
            .await
            .is_ok()
        {
            return ctx;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    panic!("Server failed to start");
}
