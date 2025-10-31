use anyhow::anyhow;
use axum::{
    Json, Router,
    extract::State,
    routing::{delete, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, SqlitePool, migrate::MigrateDatabase, sqlite::Sqlite};
use std::sync::Arc;

const PORT: &str = "3000";
const DB_URL: &str = "sqlite://sqlite.db";

struct AppState {
    db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_db().await;

    let db = SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!().run(&db).await?;

    let app_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/customers", post(c_customer))
        .route("/customers/{id}", delete(c_customer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct Customer {
    id: i64,
    name: String,
    surname: String,
    email: Option<String>,
    price_per_hour: f64,
}

#[derive(Deserialize)]
struct CustomerFromQuery {
    id: Option<i64>,
    name: String,
    surname: String,
    email: Option<String>,
    price_per_hour: f64,
}

impl CustomerFromQuery {
    fn try_into_customer(&self) -> anyhow::Result<Customer> {
        if let Some(id) = self.id {
            Ok(Customer {
                id,
                name: self.name.clone(),
                email: self.email.clone(),
                price_per_hour: self.price_per_hour,
                surname: self.surname.clone(),
            })
        } else {
            Err(anyhow!("Couldn't get the ID"))
        }
    }
}

// Controllers
async fn c_customer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CustomerFromQuery>,
) -> axum::Json<Customer> {
    let customer = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            INSERT INTO customers (name, surname, email, price_per_hour)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        payload.name,
        payload.surname,
        payload.email,
        payload.price_per_hour
    )
    .fetch_one(&state.db)
    .await
    .unwrap()
    .try_into_customer()
    .unwrap();
    Json(customer)
}

// DB util
async fn run_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}
