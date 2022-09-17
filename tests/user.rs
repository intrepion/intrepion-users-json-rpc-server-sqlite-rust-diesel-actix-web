#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App};
    use intrepion_users_json_rpc_server_sqlite_rust_diesel_actix_web::{
        json_rpc::JsonRpcRequest, routes::index,
    };

    #[actix_web::test]
    async fn test_index() {
        let app =
            test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
                .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&JsonRpcRequest {
                id: "1".to_string(),
                jsonrpc: "2.0".to_owned(),
                method: "subtract".to_owned(),
                params: vec![42, 23],
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r##"{"id":"1","jsonrpc":"2.0","result":19}"##);
    }
}
