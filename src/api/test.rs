#[cfg(test)]
mod tests {
    use axum_extra::extract::cookie::Cookie;
    use axum_test::TestServer;
    use std::sync::Arc;

    use crate::{
        api::{auth::register_auth_routes, user::register_user_routes},
        store::Store,
    };

    #[tokio::test]
    async fn test_auth_routes() {
        const DB_URL: &str = "sqlite:target/memory";
        let store = Arc::new(Store::new(DB_URL).await);
        let app = register_auth_routes().await.with_state(store.clone());
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/signup")
            .json(&serde_json::json!({
                "username": "testuser",
                "password": "password"
            }))
            .await;

        assert_eq!(response.status_code(), 201, "User sign up");

        let response = server
            .post("/signin")
            .json(&serde_json::json!({
                "username": "testuser",
                "password": "passworda"
            }))
            .await;
        assert_eq!(
            response.status_code(),
            401,
            "User sign in with invalid credentials"
        );

        let response = server
            .post("/signin")
            .json(&serde_json::json!({
                "username": "testuser",
                "password": "password"
            }))
            .await;
        assert_eq!(
            response.status_code(),
            200,
            "User sign in with valid credentials"
        );

        let response = server.post("/signout").await;
        assert_eq!(response.status_code(), 200, "User sign out");
    }

    #[tokio::test]
    async fn test_user_routes() {
        const DB_URL: &str = "sqlite:target/memory";
        let store = Arc::new(Store::new(DB_URL).await);
        let app = register_user_routes(store.clone())
            .await
            .with_state(store.clone());
        let server = TestServer::new(app).unwrap();

        let user = store
            .create_new_user("testuser", "password", "", "")
            .await
            .unwrap();

        let response = server
            .get("/me")
            .add_cookie(Cookie::new("user_id", user.id.clone()))
            .await;
        assert_eq!(response.status_code(), 200, "Get my user info");

        let response = server
            .patch("/me")
            .add_cookie(Cookie::new("user_id", user.id.clone()))
            .json(&serde_json::json!({
                "github_name": "new_github"
            }))
            .await;
        assert_eq!(response.status_code(), 200, "Update my user info");
    }
}
