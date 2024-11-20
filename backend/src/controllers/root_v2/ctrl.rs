use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::pkgs::repos::test_log::{create::create, list_all::list_all};
use crate::pkgs::time;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    db: Pool<ConnectionManager<PgConnection>>,
    name: String,
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Router {
    let s = CtrlState {
        db: db.clone(),
        name: "root_v2".to_string(),
    };

    Router::new()
        .route("/root/v2/ruok", get(ruok))
        .route("/root/v2/testLog", post(create_test_log))
        .route("/root/v2/testLogs", get(list_all_test_logs))
        .with_state(s)
}

////////////////////////////////////////////////////////////////////////////////

async fn ruok(State(state): State<CtrlState>) -> impl IntoResponse {
    format!("I am ok, my name is {}", state.name)
}

async fn create_test_log(State(state): State<CtrlState>) -> impl IntoResponse {
    match create(&state.db) {
        Ok(test_log) => {
            format!("Created test log: {}", test_log.id)
        }
        Err(e) => {
            format!("Failed to create test log: {}", e)
        }
    }
}

async fn list_all_test_logs(State(state): State<CtrlState>) -> impl IntoResponse {
    match list_all(&state.db) {
        Ok(test_logs) => {
            let mut res = String::new();
            for test_log in test_logs {
                res.push_str(&format!(
                    "Test log: {} created at: {}\n",
                    test_log.id,
                    test_log
                        .created_at
                        .map(|t| time::iso8601(&t))
                        .unwrap_or("None".to_string())
                ));
            }

            res
        }
        Err(e) => {
            format!("Failed to list test logs: {}", e)
        }
    }
}
