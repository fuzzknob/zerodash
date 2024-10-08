use crate::modules::tasks::task_routes;
use lunarus::prelude::*;

pub mod tasks;

pub fn routes() -> Router<AppContext> {
    Router::new().merge(task_routes())
}
