use crate::lunar::prelude::*;
use crate::modules::tasks::task_routes;

pub mod tasks;

pub fn routes() -> Router<AppContext> {
    Router::new().merge(task_routes())
}
