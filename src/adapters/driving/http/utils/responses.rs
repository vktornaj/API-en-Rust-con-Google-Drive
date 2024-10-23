use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonError {
    pub message: String,
    pub details: String,
}

#[derive(Serialize)]
pub struct JsonResponse<T> {
    #[serde(skip)]
    pub status: StatusCode,
    pub data: Option<T>,
    pub error: Option<JsonError>,
}

impl<T> JsonResponse<T> {
    pub fn new_ok(data: T) -> Self {
        JsonResponse {
            status: StatusCode::OK,
            data: Some(data),
            error: None,
        }
    }

    pub fn new_err(status: StatusCode, message: &str, details: String) -> Self {
        JsonResponse {
            status,
            data: None,
            error: Some(JsonError {
                message: message.to_string(),
                details,
            }),
        }
    }

    pub fn new_int_ser_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(JsonError {
                message: "Internal Server Error".to_string(),
                details,
            }),
        }
    }

    pub fn new_bad_req_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::BAD_REQUEST,
            data: None,
            error: Some(JsonError {
                message: "Bad request".to_string(),
                details,
            }),
        }
    }

    pub fn new_conflict_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::CONFLICT,
            data: None,
            error: Some(JsonError {
                message: "Conflict".to_string(),
                details,
            }),
        }
    }

    pub fn new_unauthorized_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::UNAUTHORIZED,
            data: None,
            error: Some(JsonError {
                message: "Unauthorized".to_string(),
                details,
            }),
        }
    }

    pub fn new_not_found_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::NOT_FOUND,
            data: None,
            error: Some(JsonError {
                message: "Not found".to_string(),
                details,
            }),
        }
    }

    pub fn new_forbidden_err(details: String) -> Self {
        JsonResponse {
            status: StatusCode::FORBIDDEN,
            data: None,
            error: Some(JsonError {
                message: "Forbidden".to_string(),
                details,
            }),
        }
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            )
            .body(Body::from(serde_json::to_vec(&self).unwrap()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use serde_json::json;

    #[test]
    fn test_response() {
        // let json_response = JsonResponse {
        //     data: Some("data".to_string()),
        //     error: None,
        //     status: StatusCode::OK,
        // };
        // let expected = json!({
        //     "status": 200,
        //     "data": Some("data"),
        //     "error": {
        //         "message": None::<String>,
        //         "details": None::<String>,
        //     },
        // });
        // assert_eq!(serde_json::to_value(json_response).unwrap(), expected);
    }
}
