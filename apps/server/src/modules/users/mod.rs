use lunarus::prelude::*;
use middleware::from_fn_with_state;

use super::auth::auth_middleware::auth_middleware;

pub mod model;
pub mod user_controller;

pub fn user_routes(context: AppContext) -> Router<AppContext> {
    Router::new()
        .route("/users/me", get(user_controller::me))
        .layer(from_fn_with_state(context, auth_middleware))
}
