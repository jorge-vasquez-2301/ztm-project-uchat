use axum::{async_trait, Json};
use hyper::StatusCode;
use uchat_endpoint::user::{CreateUser, CreateUserOk};

use crate::{error::ApiResult, extractor::DbConnection, AppState};

use super::PublicApiRequest;

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(self.password)?;
        let user_id = uchat_query::user::new(&mut conn, password_hash, &self.username)?;

        tracing::info!(username = self.username.as_ref(), "new user created");

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username,
            }),
        ))
    }
}
