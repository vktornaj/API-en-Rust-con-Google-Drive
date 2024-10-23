use axum::{
    extract::{Query, State},
    Extension,
};
use uuid::Uuid;

use super::{state::AppState, utils::responses::JsonResponse};
use crate::{application::usecases, domain::value_objects::id::Id};

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
