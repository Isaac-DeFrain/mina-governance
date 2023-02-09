pub mod api;

use super::Config;
use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};

use axum_extra::routing::SpaRouter;
use tower_http::services::ServeFile;
pub trait Build {
    fn build_v1(cfg: &Config) -> Router;
}

#[allow(clippy::string_add)]
impl Build for Router {
    fn build_v1(cfg: &Config) -> Router {
        let spa = SpaRouter::new("/assets", format!("{}/assets", &cfg.client_path))
            .index_file("index.html");

        let react_router_fallback =
            get_service(ServeFile::new(format!("{}/index.html", &cfg.client_path))).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Internal Server Error: {error}"),
                    )
                },
            );

        Router::new()
            .merge(spa)
            .route("/api/v1/:keyword", get(api::keyword::keyword_handler))
            .route(
                "/api/v1/:keyword/results",
                get(api::keyword::keyword_results_handler),
            )
            .fallback(react_router_fallback)
    }
}
