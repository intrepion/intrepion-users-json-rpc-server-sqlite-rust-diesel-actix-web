#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn test_not_logged_in() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
    }
}
