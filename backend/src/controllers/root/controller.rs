use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};

#[derive(Clone)]
struct CtrlState {
    name: String,
}

pub struct Controller{
    m: CtrlState,
}

impl Controller {
    pub fn new() -> Self {
        Controller{
            m: CtrlState{
                name: "root".to_string(),
            }
        }
    }

    pub fn register(&self, router: Router) -> Router {
        let sub_router = Router::new()
            .route("/ruok", get(ruok))
            .with_state(self.m.clone());

        router.nest("/root", sub_router)
    }
}

async fn ruok(State(state): State<CtrlState>) -> impl IntoResponse {
    format!("I am ok, my name is {}", state.name)
}
