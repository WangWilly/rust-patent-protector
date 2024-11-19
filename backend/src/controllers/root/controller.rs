use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::pkgs::errors::handler_404;
use crate::pkgs::repos::test_log::{create::create, list_all::list_all};
use crate::pkgs::time;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    db: Pool<ConnectionManager<PgConnection>>,
    name: String,
}

pub struct Controller {
    m: CtrlState,
}

////////////////////////////////////////////////////////////////////////////////

impl Controller {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        Controller {
            m: CtrlState {
                db,
                name: "root".to_string(),
            },
        }
    }

    pub fn register(&self, router: Router) -> Router {
        let sub_router = Router::new()
            .fallback(handler_404)
            .route("/ruok", get(ruok))
            .route("/testLog", post(create_test_log))
            .route("/testLogs", get(list_all_test_logs))
            .with_state(self.m.clone());

        router.nest("/root", sub_router)
    }
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
