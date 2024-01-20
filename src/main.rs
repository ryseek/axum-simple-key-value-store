use axum::response::Html;
use axum::{routing::get, Extension, Router};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Default)]
struct KeyValueStore {
    data: RwLock<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let store: Arc<KeyValueStore> = Arc::new(KeyValueStore::default());

    let app = Router::new()
        .route("/", get(|| async { Html("KV store".to_string()) }))
        .route("/get/:key", get(read))
        .route("/set/:key/:value", get(save))
        .route("/del/:key", get(delete))
        .route("/health", get(|| async { Html("OK".to_string()) }))
        .layer(Extension(store));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn read(
    params: axum::extract::Path<String>,
    store: axum::extract::Extension<Arc<KeyValueStore>>,
) -> Html<String> {
    let key = params.0;
    println!("reading key: {}", key);
    let data = store.data.read().await;

    match data.get(&key) {
        Some(value) => Html(value.clone()),
        None => Html("Key not found".to_string()),
    }
}

async fn save(
    params: axum::extract::Path<(String, String)>,
    store: axum::extract::Extension<Arc<KeyValueStore>>,
) -> Html<String> {
    let (key, value) = params.0;
    println!("saving key: {} ", key);
    let mut data = store.data.write().await;
    data.insert(key, value);

    Html("OK".to_string())
}

async fn delete(
    params: axum::extract::Path<String>,
    store: axum::extract::Extension<Arc<KeyValueStore>>,
) -> Html<String> {
    let key = params.0;
    println!("deleting key: {}", key);
    let mut data = store.data.write().await;
    data.remove(&key);

    Html("OK".to_string())
}
