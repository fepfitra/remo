use sqlx::{AnyPool, Sqlite, any::AnyPoolOptions, migrate::MigrateDatabase};
pub mod memo;
pub mod test;
pub mod user;
pub struct Store {
    pool: AnyPool,
}

impl Store {
    pub async fn new(database_url: &str) -> Self {
        sqlx::any::install_default_drivers();

        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            print!("Database {} does not exist. Creating...\n", database_url);
            match Sqlite::create_database(database_url).await {
                Ok(_) => print!("Database created successfully.\n"),
                Err(e) => print!("Failed to create database: {}\n", e),
            }
        }

        let pool = AnyPoolOptions::new()
            .connect(database_url)
            .await
            .expect("Failed to create database pool");

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./resources");

        let migrator = sqlx::migrate::Migrator::new(migrations).await.unwrap();
        println!("Found {} migrations", migrator.migrations.len());

        let migration_results = migrator.run(&pool).await;

        match migration_results {
            Ok(_) => print!("Migrations applied successfully.\n"),
            Err(e) => panic!("Failed to apply migrations: {}\n", e),
        }

        sqlx::query("SELECT * FROM sqlite_master")
            .fetch_all(&pool)
            .await
            .expect("Failed to verify database connection");

        Store { pool }
    }
}
