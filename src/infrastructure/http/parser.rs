use actix_web::{
    error::{Error, InternalError, JsonPayloadError, PathError, QueryPayloadError},
    http::StatusCode,
    HttpRequest, HttpResponse,
};

use crate::infrastructure::http::response::GenericApiResponse;

pub fn json_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let detail = err.to_string();

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(
            GenericApiResponse::from_error(&detail, StatusCode::UNSUPPORTED_MEDIA_TYPE),
        ),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(GenericApiResponse::from_error(
                &detail,
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        }
        _ => HttpResponse::BadRequest().json(GenericApiResponse::from_error(
            &detail,
            StatusCode::BAD_REQUEST,
        )),
    };

    InternalError::from_response(err, resp).into()
}

pub fn query_handler(err: QueryPayloadError, _req: &HttpRequest) -> Error {
    let detail = err.to_string();

    let resp = match &err {
        QueryPayloadError::Deserialize(..) => HttpResponse::UnprocessableEntity().json(
            GenericApiResponse::from_error(&detail, StatusCode::UNPROCESSABLE_ENTITY),
        ),
        _ => HttpResponse::BadRequest().json(GenericApiResponse::from_error(
            &detail,
            StatusCode::BAD_REQUEST,
        )),
    };

    InternalError::from_response(err, resp).into()
}

pub fn path_handler(err: PathError, _req: &HttpRequest) -> Error {
    let detail = err.to_string();

    let resp = match &err {
        PathError::Deserialize(..) => HttpResponse::UnprocessableEntity().json(
            GenericApiResponse::from_error(&detail, StatusCode::UNPROCESSABLE_ENTITY),
        ),
        _ => HttpResponse::BadRequest().json(GenericApiResponse::from_error(
            &detail,
            StatusCode::BAD_REQUEST,
        )),
    };

    InternalError::from_response(err, resp).into()
}
