use anyhow::{Ok, anyhow};
use axum::{Extension, Router};
use axum::{
    Json,
    extract::Path,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::http::ApiCtx;

pub fn router() -> Router {
    Router::new()
        .route("/customers", post(create_customer))
        .route("/customers", get(list_customer))
        .route("/customers/{id}", delete(delete_customer))
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
#[axum::debug_handler]
async fn create_customer(
    ctx: Extension<Arc<ApiCtx>>,
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
    .fetch_one(&ctx.db)
    .await
    .unwrap()
    .try_into_customer()
    .unwrap();
    Json(customer)
}

async fn delete_customer(
    ctx: Extension<Arc<ApiCtx>>,
    Path(customer_id): Path<String>,
) -> axum::Json<Customer> {
    let customer = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            DELETE FROM customers WHERE id == (?)
            RETURNING *
        "#,
        customer_id
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap()
    .try_into_customer()
    .unwrap();

    Json(customer)
}

async fn list_customer(ctx: Extension<Arc<ApiCtx>>) -> axum::Json<Vec<Customer>> {
    let customers: Vec<Customer> = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            SELECT * from customers;
        "#,
    )
    .fetch_all(&ctx.db)
    .await
    .unwrap()
    .iter_mut()
    .map(|t| t.try_into_customer().unwrap())
    .collect();

    Json(customers)
}

// DB util
