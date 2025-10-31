use clap::Parser;
use sqlx::SqlitePool;

use headless::config::Config;
use headless::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();

    let db = SqlitePool::connect(&config.database_url).await?;
    sqlx::migrate!().run(&db).await?;

    http::serve(db, config).await?;

    Ok(())
}

// async fn build_db(config: &Config) {
//     if !Sqlite::database_exists(&config.database_url)
//         .await
//         .unwrap_or(false)
//     {
//         println!("Creating database {}", config.database_url);
//         match Sqlite::create_database(&config.database_url).await {
//             Ok(_) => println!("Create db success"),
//             Err(error) => panic!("error: {}", error),
//         }
//     } else {
//         println!("Database already exists");
//     }
// }
