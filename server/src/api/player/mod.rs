use crate::app_state::AppState;
use axum::http::request;
use axum::routing::get;
use axum::{middleware, Router};
use characters::player_get_character;
use whoami::player_whoami;

mod characters;
mod whoami;

pub fn player_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/player/whoami",
            get(player_whoami).layer(middleware::from_fn(super::auth::authorize)),
        )
        .route(
            "/player/:world_code/:character_code",
            get(player_get_character).layer(middleware::from_fn(|req, next| {
                super::auth::authorize(req, next)
            })),
        )

    /* .route(
        "/player/:world_code/:character_code",
        get(player_get_character).layer(middleware::from_fn(authorization("game.world.manage"))),

    )*/
}
