use sqlx::{PgPool, Pool, Postgres};

async fn setup_test_db() -> PgPool {
  let database_url = std::env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");

  let pool = PgPool::connect(&database_url).await.expect("Failed to connect to Postgres for testing.");

  sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations for testing.");

  sqlx::query("TRUNCATE TABLE payments RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to clean database before tests");

    pool
}