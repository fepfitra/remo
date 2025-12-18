use crate::store::Store;
use anyhow::Result;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Memo {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl Store {
    pub async fn create_new_memo(&self, content: &str, user_id: &str) -> Result<Memo> {
        let memo = Memo {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.to_string(),
            user_id: user_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            deleted_at: None,
        };

        sqlx::query(
            "
            INSERT INTO memos (id, content, user_id, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&memo.id)
        .bind(&memo.content)
        .bind(&memo.user_id)
        .bind(&memo.created_at)
        .bind(&memo.updated_at)
        .bind(&memo.deleted_at)
        .execute(&self.pool)
        .await?;
        Ok(memo)
    }

    pub async fn get_memo_by_id(&self, memo_id: &str) -> Result<Memo> {
        let memo: Memo = sqlx::query_as(
            "
            SELECT id, content, user_id, created_at, updated_at, deleted_at
            FROM memos
            WHERE id = ?",
        )
        .bind(memo_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(memo)
    }

    pub async fn update_memo(
        &self,
        memo_id: &str,
        content: Option<&str>,
        deleted_at: Option<&str>,
    ) -> Result<Memo> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut memo = self.get_memo_by_id(memo_id).await.unwrap();
        memo.content = content.unwrap_or(&memo.content).to_string();
        memo.deleted_at = deleted_at.map(|s| s.to_string()).or(memo.deleted_at);
        memo.updated_at = now;

        sqlx::query(
            "
            UPDATE memos
            SET content = ?, updated_at = ?, deleted_at = ?
            WHERE id = ?",
        )
        .bind(&memo.content)
        .bind(&memo.updated_at)
        .bind(&memo.deleted_at)
        .bind(&memo.id)
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(memo)
    }

    pub async fn get_memos_by_user_id(&self, user_id: &str) -> Result<Vec<Memo>> {
        let memos: Vec<Memo> = sqlx::query_as(
            "
            SELECT id, content, user_id, created_at, updated_at, deleted_at
            FROM memos
            WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(memos)
    }
}
