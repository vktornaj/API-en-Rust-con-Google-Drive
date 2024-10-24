use axum::{
    extract::{Query, State},
    response::Redirect,
    Extension,
};
use serde::Deserialize;
use uuid::Uuid;

use super::{state::AppState, utils::responses::JsonResponse};
use crate::{application::usecases, domain::value_objects::id::Id};

pub async fn handler_get_google_auth_url(
    State(state): State<AppState>,
) -> Result<Redirect, JsonResponse<String>> {
    match usecases::get_google_auth_url::execute(&state.google_drive_service).await {
        Ok(url) => Ok(Redirect::permanent(&url)),
        Err(_) => {
            return Err(JsonResponse::new_int_ser_err(
                "Internal Server Error".to_string(),
            ));
        }
    }
}

#[derive(Deserialize)]
pub struct GoogleCallbackQuery {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

pub async fn handler_handle_google_callback(
    State(state): State<AppState>,
    Query(params): Query<GoogleCallbackQuery>,
) -> JsonResponse<String> {
    let payload = usecases::handle_google_callback::Payload {
        code: params.code.to_string(),
    };

    match usecases::handle_google_callback::execute(
        &state.user_repository,
        &state.google_drive_service,
        &state.config.secret,
        payload,
    )
    .await
    {
        Ok(auth_token) => JsonResponse::<String>::new_ok(auth_token),
        Err(_) => {
            return JsonResponse::new_int_ser_err("Internal Server Error".to_string());
        }
    }
}

pub async fn handler_get_list_files(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Query(path): Query<String>,
) -> JsonResponse<Vec<std::string::String>> {
    let user_id = if let Ok(user_id) = Id::try_from(user_id) {
        user_id
    } else {
        return JsonResponse::new_int_ser_err("Internal Server Error".to_string());
    };
    let payload = usecases::list_files::Payload {
        path: path.to_string(),
        user_id,
    };
    match usecases::list_files::execute(
        &state.user_repository,
        &state.google_drive_service,
        payload,
    )
    .await
    {
        Ok(file_ids) => JsonResponse::<Vec<String>>::new_ok(file_ids),
        Err(_) => {
            return JsonResponse::new_int_ser_err("Internal Server Error".to_string());
        }
    }
}
