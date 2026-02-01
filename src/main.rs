use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

const DATA_FILE: &str = "data.json";

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

fn load_data() -> HashMap<String, String> {
    let path = PathBuf::from(DATA_FILE);
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_data(data: &HashMap<String, String>) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(DATA_FILE, json);
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
    save_data(&data);
    Json(SetResponse { success: true })
}

#[tokio::main]
async fn main() {
    let data = load_data();
    let state: AppState = Arc::new(RwLock::new(data));

    let app = Router::new()
        .route("/get/:key", get(get_value))
        .route("/set", post(set_value))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
