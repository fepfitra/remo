use crate::store::Store;
use anyhow::Result;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(FromRow, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub github_name: Option<String>,
    pub wx_open_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Store {
    pub async fn create_new_user(
        &self,
        username: &str,
        password: &str,
        github_name: &str,
        wx_open_id: &str,
    ) -> Result<User> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: password.to_string(),
            github_name: Some(github_name.to_string()),
            wx_open_id: Some(wx_open_id.to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        sqlx::query("
            INSERT INTO users (id, username, password, github_name, wx_open_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.github_name)
        .bind(&user.wx_open_id)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        username: Option<&str>,
        password: Option<&str>,
        github_name: Option<&str>,
        wx_open_id: Option<&str>,
    ) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut user = self.get_user_by_id(user_id).await?;

        user.username = username.unwrap_or(&user.username).to_string();
        user.password = password.unwrap_or(&user.password).to_string();
        user.github_name = github_name.map(|s| s.to_string()).or(user.github_name);
        user.wx_open_id = wx_open_id.map(|s| s.to_string()).or(user.wx_open_id);
        user.updated_at = now;

        sqlx::query(
            "
            UPDATE users
            SET username = ?, password = ?, github_name = ?, wx_open_id = ?, updated_at = ?
            WHERE id = ?
        ",
        )
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.github_name)
        .bind(&user.wx_open_id)
        .bind(&user.updated_at)
        .bind(&user.id)
        .execute(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        let user: User = sqlx::query_as(
            "
            SELECT id, username, password, github_name, wx_open_id, created_at, updated_at
            FROM users
            WHERE id = ?",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_username_and_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User> {
        let user: User = sqlx::query_as(
            "
            SELECT id, username, password, github_name, wx_open_id, created_at, updated_at
            FROM users
            WHERE username = ? AND password = ?",
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_github_name(&self, github_name: &str) -> Result<User> {
        let user: User = sqlx::query_as(
            "
            SELECT id, username, password, github_name, wx_open_id, created_at, updated_at
            FROM users
            WHERE github_name = ?",
        )
        .bind(github_name)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_wx_open_id(&self, wx_open_id: &str) -> Result<User> {
        let user: User = sqlx::query_as(
            "
            SELECT id, username, password, github_name, wx_open_id, created_at, updated_at
            FROM users
            WHERE wx_open_id = ?",
        )
        .bind(wx_open_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
