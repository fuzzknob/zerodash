use lunarus::prelude::*;

pub mod auth;
pub mod boards;
pub mod home;
pub mod spaces;
pub mod tasks;
pub mod users;

pub fn routes(context: AppContext) -> Router<AppContext> {
    Router::new().merge(home::home_routes()).nest(
        "/v1/api",
        tasks::task_routes(context.clone())
            .merge(users::user_routes(context.clone()))
            .merge(auth::auth_routes()),
    )
}
