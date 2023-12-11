mod commands;
mod resources;

use axum::{routing::get, Router};
use resources::home;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower_http::services::ServeDir;

type SharedState = Arc<AppState>;

pub struct AppState {
    pub pool: Pool<Postgres>,
    pub teloxide_token: String,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, teloxide_token: String) -> AppState {
        AppState {
            pool,
            teloxide_token,
        }
    }
}

pub struct BackgroundServices {
    pub state: SharedState,
    pub router: Router,
}

impl BackgroundServices {
    pub fn new(state: SharedState, router: Router) -> BackgroundServices {
        BackgroundServices { state, router }
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BackgroundServices {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = axum::Server::bind(&addr).serve(self.router.into_make_service());

        tokio::select!(
            _ = router => (),
            _ =  commands::run(self.state) => ()
        );

        Ok(())
    }
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(home::jobs))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state)
}
