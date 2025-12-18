use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::{
    api::{auth::register_auth_routes, user::register_user_routes},
    store::Store,
};

mod api;
mod common;
mod store;

const DB_URL: &str = "sqlite://resources/memos.db";

#[tokio::main]
async fn main() {
    let store = Store::new(DB_URL).await;
    let state = Arc::new(store);
    let app = Router::new()
        .nest("/api/auth", register_auth_routes().await)
        .nest("/api/user", register_user_routes(state.clone()).await)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
