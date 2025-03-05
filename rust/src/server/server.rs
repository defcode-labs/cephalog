use axum::{Router, routing::get};
use axum::response::Html;



#[tokio::main]
pub async fn start() {
    //let addr = "0.0.0.0:3000".parse::<SocketAddr>().await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //let addr = listener.local_addr()?;
    let app = Router::new().route("/", get(|| async { Html("<h1>Hello, World!</h1>".to_string()) }));

    axum::serve(listener, app).await.unwrap();
    
}