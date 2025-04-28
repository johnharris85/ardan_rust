

async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_test() {
        assert_eq!(2 +2, 4);
    }

    #[tokio::test]
    async fn easy_way() {
        assert_eq!(double(2).await, 4);
    }
}
