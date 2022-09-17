use crate::json_rpc::{JsonRpcRequest, JsonRpcResponse};
use actix_web::{web, HttpResponse};

pub async fn index(request: web::Json<JsonRpcRequest>) -> HttpResponse {
    let response = JsonRpcResponse {
        id: request.id.clone(),
        jsonrpc: request.jsonrpc.clone(),
        result: request.params[0] - request.params[1],
    };
    HttpResponse::Ok().json(response)
}
