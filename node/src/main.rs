use axum::{extract::State, routing::post, Json, Router};
use base64::Engine as _;
use runtime::Runtime;
use serde::Deserialize;
use state::InMemoryState;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize)]
struct DeployReq {
    wasm_base64: String,
    name: String,
}

#[derive(Deserialize)]
struct CallReq {
    contract: String,
    method: String,
    arg: i32,
}

type SharedRuntime = Arc<Runtime<InMemoryState>>;

async fn deploy(State(rt): State<SharedRuntime>, Json(req): Json<DeployReq>) -> Json<String> {
    let wasm = base64::engine::general_purpose::STANDARD
        .decode(&req.wasm_base64)
        .unwrap();
    rt.deploy(&req.name, &wasm).unwrap();
    Json(format!("deployed contract {}", req.name))
}

async fn call(State(rt): State<SharedRuntime>, Json(req): Json<CallReq>) -> Json<i32> {
    let res = rt
        .call_i32(&req.contract, &req.method, req.arg)
        .unwrap();
    Json(res)
}

#[tokio::main]
async fn main() {
    let rt: SharedRuntime = Arc::new(Runtime::new(InMemoryState::new()));

    let app = Router::new()
        .route("/deploy", post(deploy))
        .route("/call", post(call))
        .with_state(rt.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
