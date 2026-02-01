use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

type AppState = Arc<RwLock<HashMap<String, String>>>;

#[derive(Serialize, Deserialize)]
struct GetResponse {
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SetRequest {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct SetResponse {
    success: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "3000")]
    port: u16,

    #[arg(short, long, default_value = "data.json")]
    data_file: String,
}

fn load_data(path: &str) -> HashMap<String, String> {
    let path = PathBuf::from(path);
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

async fn get_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Json<GetResponse> {
    let data = state.read().unwrap();
    let value = data.get(&key).cloned();
    Json(GetResponse { value })
}

async fn set_value(
    State(state): State<AppState>,
    Json(req): Json<SetRequest>,
) -> Json<SetResponse> {
    let mut data = state.write().unwrap();
    data.insert(req.key, req.value);
    Json(SetResponse { success: true })
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data = load_data(&args.data_file);
    let state: AppState = Arc::new(RwLock::new(data));

    let app = Router::new()
        .route("/get/:key", get(get_value))
        .route("/set", post(set_value))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}