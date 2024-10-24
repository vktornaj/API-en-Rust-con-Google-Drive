use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use super::{state::AppState, utils::responses::JsonResponse};
use crate::{
    application::usecases,
    domain::value_objects::{file_info::FileInfo, id::Id},
};

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

#[derive(Deserialize)]
pub struct ListFilesQuery {
    folder_id: String,
}

pub async fn handler_get_list_files(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Query(params): Query<ListFilesQuery>,
) -> JsonResponse<Vec<FileInfo>> {
    let user_id = if let Ok(user_id) = Id::try_from(user_id) {
        user_id
    } else {
        return JsonResponse::new_int_ser_err("Internal Server Error".to_string());
    };
    let payload = usecases::list_files::Payload {
        path: params.folder_id.to_string(),
        user_id,
    };
    match usecases::list_files::execute(
        &state.user_repository,
        &state.google_drive_service,
        payload,
    )
    .await
    {
        Ok(file_ids) => JsonResponse::<Vec<FileInfo>>::new_ok(file_ids),
        Err(err) => {
            return JsonResponse::new_int_ser_err(err.to_string());
        }
    }
}

#[derive(Deserialize)]
pub struct DownloadPDFQuery {
    file_id: String,
}

pub async fn handler_download_pdf(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Query(params): Query<DownloadPDFQuery>,
) -> Result<Response, (StatusCode, String)> {
    let user_id = if let Ok(user_id) = Id::try_from(user_id) {
        user_id
    } else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        ));
    };
    let payload = usecases::download_pdf::Payload {
        file_id: params.file_id,
        user_id,
    };
    let file_path = match usecases::download_pdf::execute(
        &state.user_repository,
        &state.google_drive_service,
        payload,
    )
    .await
    {
        Ok(file_path) => file_path,
        Err(err) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
        }
    };

    let file = File::open(&file_path).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error opening PDF file: {}", err),
        )
    })?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=\"example.pdf\"",
            ),
        ],
        body,
    )
        .into_response())
}
