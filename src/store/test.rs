#[cfg(test)]
mod tests {

    use crate::store::Store;
    use sqlx::{Sqlite, migrate::MigrateDatabase};

    #[tokio::test]
    async fn test_user() {
        const DB_URL: &str = "sqlite:target/memory1";
        let store = Store::new(DB_URL).await;
        let user = store
            .create_new_user("testuser", "password", "githubuser", "wxopenid")
            .await
            .unwrap();
        assert_eq!(user.username, "testuser", "create_new_user");

        let fetched_user = store.get_user_by_id(&user.id).await.unwrap();
        assert_eq!(fetched_user.id, user.id, "get_user_by_id");

        let fetched_user2 = store
            .get_user_by_username_and_password("testuser", "password")
            .await
            .unwrap();
        assert_eq!(
            fetched_user2.id, user.id,
            "get_user_by_username_and_password"
        );

        let fetched_user3 = store.get_user_by_github_name("githubuser").await.unwrap();
        assert_eq!(fetched_user3.id, user.id, "get_user_by_github_name");

        let fetched_user4 = store.get_user_by_wx_open_id("wxopenid").await.unwrap();
        assert_eq!(fetched_user4.id, user.id, "get_user_by_wx_open_id");

        let updated_user = store
            .update_user(
                &user.id,
                Some("updateduser"),
                Some("newpassword"),
                Some("newgithub"),
                Some("newwxid"),
            )
            .await
            .unwrap();
        assert_eq!(updated_user.username, "updateduser", "update_user");

        Sqlite::drop_database(DB_URL).await.unwrap();
    }

    #[tokio::test]
    async fn test_memo() {
        const DB_URL: &str = "sqlite:target/memory";
        let store = Store::new(DB_URL).await;
        let user = store
            .create_new_user("testuser", "password", "githubuser", "wxopenid")
            .await
            .unwrap();

        let memo = store
            .create_new_memo("This is a test memo", &user.id)
            .await
            .unwrap();
        assert_eq!(memo.content, "This is a test memo", "create_new_memo");

        let fetched_memo = store.get_memo_by_id(&memo.id).await.unwrap();
        assert_eq!(fetched_memo.id, memo.id, "get_memo_by_id");

        let updated_memo = store
            .update_memo(&memo.id, Some("Updated content"), None)
            .await
            .unwrap();
        assert_eq!(updated_memo.content, "Updated content", "update_memo");

        let deleted_memo = store
            .update_memo(&memo.id, None, Some(&chrono::Utc::now().to_rfc3339()))
            .await
            .unwrap();
        assert!(deleted_memo.deleted_at.is_some(), "delete_memo");

        let get_memos_by_user = store.get_memos_by_user_id(&user.id).await.unwrap();
        assert_eq!(get_memos_by_user.len(), 1, "get_memos_by_user_id");
    }
}
