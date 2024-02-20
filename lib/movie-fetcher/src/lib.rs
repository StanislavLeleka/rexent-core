pub mod fetcher;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::fetcher;

    #[tokio::test]
    async fn empty_test() {
        fetcher::fetch().await;
        assert_eq!(0, 0);
    }
}
