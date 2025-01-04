mod integration;

#[cfg(test)]
mod tests {
    use super::integration::quiz_tests;
    
    #[tokio::test]
    async fn integration_quiz_test() -> anyhow::Result<()> {
        quiz_tests::test_quiz_crud_operations().await
    }
}
