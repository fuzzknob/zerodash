use lunarus::prelude::*;

pub mod auth;
pub mod home;
pub mod tasks;
pub mod users;

pub fn routes() -> Router<AppContext> {
    Router::new().merge(home::home_routes()).nest(
        "/v1/api",
        tasks::task_routes()
            .merge(users::user_routes())
            .merge(auth::auth_routes()),
    )
}
