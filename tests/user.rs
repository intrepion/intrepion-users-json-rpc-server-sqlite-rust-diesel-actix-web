#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use intrepion_users_json_rpc_server_sqlite_rust_diesel_actix_web::routes::index;

    #[actix_web::test]
    async fn test_not_logged_in() {
        let _app = test::init_service(App::new().route("/", web::get().to(index))).await;
    }
}
