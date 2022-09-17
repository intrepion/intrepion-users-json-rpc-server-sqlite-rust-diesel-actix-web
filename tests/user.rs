#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use intrepion_users_json_rpc_server_sqlite_rust_diesel_actix_web::routes::index;

    #[actix_web::test]
    async fn test_sign_up() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let request = test::TestRequest::post().uri("/").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_client_error());
    }
}
