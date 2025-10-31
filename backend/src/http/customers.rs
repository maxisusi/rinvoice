use anyhow::anyhow;
use axum::{Extension, Router};
use axum::{
    Json,
    extract::Path,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::http::{self, ApiCtx};

pub fn router() -> Router {
    Router::new()
        .route("/customers", get(list_customer))
        .route("/customers/{id}", get(get_customer))
        .route("/customers/{id}", delete(delete_customer))
        .route("/customers", post(create_customer))
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
) -> http::Result<axum::Json<Customer>> {
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
    .await?
    .try_into_customer()?;

    Ok(Json(customer))
}

#[axum::debug_handler]
async fn get_customer(
    ctx: Extension<Arc<ApiCtx>>,
    Path(customer_id): Path<String>,
) -> http::Result<axum::Json<Customer>> {
    let customer = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            SELECT * FROM customers WHERE id == (?)
        "#,
        customer_id
    )
    .fetch_one(&ctx.db)
    .await?
    .try_into_customer()?;

    Ok(Json(customer))
}

async fn delete_customer(
    ctx: Extension<Arc<ApiCtx>>,
    Path(customer_id): Path<String>,
) -> http::Result<axum::Json<Customer>> {
    let customer = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            DELETE FROM customers WHERE id == (?)
            RETURNING *
        "#,
        customer_id
    )
    .fetch_one(&ctx.db)
    .await?
    .try_into_customer()?;

    Ok(Json(customer))
}

async fn list_customer(ctx: Extension<Arc<ApiCtx>>) -> http::Result<axum::Json<Vec<Customer>>> {
    let customers = sqlx::query_as!(
        CustomerFromQuery,
        r#"
            SELECT * from customers;
        "#,
    )
    .fetch_all(&ctx.db)
    .await?
    .iter_mut()
    .map(|t| t.try_into_customer())
    .collect::<anyhow::Result<Vec<Customer>>>()?;

    Ok(Json(customers))
}
