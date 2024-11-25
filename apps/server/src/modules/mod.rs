use lunarus::prelude::*;

pub mod auth;
pub mod boards;
pub mod home;
pub mod spaces;
pub mod tasks;
pub mod users;

pub fn routes(context: AppContext, router: Router<AppContext>) -> Router<AppContext> {
    router.merge(home::home_routes()).nest(
        "/v1/api",
        auth::auth_routes()
            .merge(users::user_routes(context.clone()))
            .merge(spaces::space_routes(context.clone()))
            .merge(tasks::task_routes(context.clone()))
            .merge(boards::board_routes(context)),
    )
}
