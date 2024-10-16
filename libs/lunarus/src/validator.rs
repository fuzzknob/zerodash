pub use garde::Validate;

// TODO: make extractor work
// use crate::prelude::{AppContext, Error};
// use serde::de::DeserializeOwned;
// use axum::{
//     async_trait,
//     extract::{rejection::JsonRejection, FromRef, FromRequest, Request},
//     Json,
// };
// #[derive(Debug, Clone, Copy, Default)]
// pub struct JsonValidator<T>(pub T);

// #[async_trait]
// impl<T, S> FromRequest<S> for JsonValidator<T>
// where
//     T: DeserializeOwned + Validate<Context = AppContext>,
//     S: Send + Sync,
//     Json<T>: FromRequest<S, Rejection = JsonRejection>,
//     AppContext: FromRef<S>,
// {
//     type Rejection = Error;
//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(req, &state).await?;
//         let ctx = AppContext::from_ref(state);
//         value.validate_with(&ctx.clone())?;
//         Ok(JsonValidator(value))
//     }
// }
