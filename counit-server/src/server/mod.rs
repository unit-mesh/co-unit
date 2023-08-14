use std::borrow::Cow;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

pub mod embedding_api;
pub mod translate_api;
pub mod archguard_api;
pub mod semantic_api;



pub(crate) fn json<'a, T>(val: T) -> Json<Response<'a>>
    where
        Response<'a>: From<T>,
{
    Json(Response::from(val))
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error {
    status: StatusCode,
    body: Json<Response<'static>>,
}

impl Error {
    fn new(kind: ErrorKind, message: impl Into<Cow<'static, str>>) -> Error {
        let status = match kind {
            ErrorKind::Configuration
            | ErrorKind::Unknown
            | ErrorKind::UpstreamService
            | ErrorKind::Internal
            | ErrorKind::Custom => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::User => StatusCode::BAD_REQUEST,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
        };

        let body = Json(Response::from(EndpointError {
            kind,
            message: message.into(),
        }));

        Error { status, body }
    }

    fn with_status(mut self, status_code: StatusCode) -> Self {
        self.status = status_code;
        self
    }

    fn internal<S: std::fmt::Display>(message: S) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: Json(Response::from(EndpointError {
                kind: ErrorKind::Internal,
                message: message.to_string().into(),
            })),
        }
    }

    fn user<S: std::fmt::Display>(message: S) -> Self {
        Error {
            status: StatusCode::BAD_REQUEST,
            body: Json(Response::from(EndpointError {
                kind: ErrorKind::User,
                message: message.to_string().into(),
            })),
        }
    }

    fn message(&self) -> &str {
        match &self.body {
            Json(Response::Error(EndpointError { message, .. })) => message.as_ref(),
            _ => "",
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::internal(value.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.body).into_response()
    }
}

/// The response upon encountering an error
#[derive(serde::Serialize, PartialEq, Eq, Debug)]
pub struct EndpointError<'a> {
    /// The kind of this error
    kind: ErrorKind,

    /// A context aware message describing the error
    message: Cow<'a, str>,
}

/// The kind of an error
#[allow(unused)]
#[derive(serde::Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ErrorKind {
    User,
    Unknown,
    NotFound,
    Configuration,
    UpstreamService,
    Internal,

    // TODO: allow construction of detailed custom kinds
    #[doc(hidden)]
    Custom,
}

pub(crate) trait ApiResponse: erased_serde::Serialize {}
erased_serde::serialize_trait_object!(ApiResponse);

/// Every endpoint exposes a Response type
#[derive(serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub(crate) enum Response<'a> {
    Ok(Box<dyn erased_serde::Serialize + Send + Sync + 'static>),
    Error(EndpointError<'a>),
}

impl<T: ApiResponse + Send + Sync + 'static> From<T> for Response<'static> {
    fn from(value: T) -> Self {
        Self::Ok(Box::new(value))
    }
}

impl<'a> From<EndpointError<'a>> for Response<'a> {
    fn from(value: EndpointError<'a>) -> Self {
        Self::Error(value)
    }
}
