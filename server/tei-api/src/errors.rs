use std::{
    collections::BTreeMap,
    collections::HashSet,
    fmt::{Debug, Display},
};

use axum::{http::StatusCode, response::IntoResponse, Json};
use deadpool_postgres::PoolError;
use lazy_static::lazy_static;
use serde::Serialize;
use tei_data::tags;
use tracing::warn;
use unicase::UniCase;

// ------- Problem details
lazy_static! {
    static ref RESERVED_NAMES: HashSet<UniCase<&'static str>> = {
        let mut keys = HashSet::new();

        keys.insert(UniCase::new("type"));
        keys.insert(UniCase::new("status"));
        keys.insert(UniCase::new("title"));
        keys.insert(UniCase::new("detail"));
        keys.insert(UniCase::new("instance"));
        keys.insert(UniCase::new("additional_fields"));

        keys
    };
}

#[derive(Debug, Serialize, Default)]
pub struct ProblemDetails {
    pub r#type: Option<String>,

    pub title: Option<String>,

    pub details: Option<String>,

    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

impl ProblemDetails {
    #[must_use]
    pub fn with_type<T: Into<String>>(mut self, value: T) -> Self {
        self.r#type = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_title<T: Into<String>>(mut self, value: T) -> Self {
        self.title = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_details<T: Into<String>>(mut self, value: T) -> Self {
        self.details = Some(value.into());
        self
    }

    pub fn with_extension<K: Into<String>, V: Serialize>(
        mut self,
        key: K,
        value: V,
    ) -> Result<Self, ApiError> {
        let key: String = key.into();
        let key_uni = UniCase::new(key.as_str());

        if RESERVED_NAMES.contains(&key_uni) {
            return Err(ApiError::internal(format!(
                "'{key}' can't be used as an extension key"
            )))?;
        }

        let value = serde_json::to_value(value).map_err(ApiError::internal_err)?;
        self.additional_fields.insert(key, value);

        Ok(self)
    }
}

// ------- Api Error

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    problem_details: Box<ProblemDetails>,
    _message: Option<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("tst")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        warn!("error: {:?}", self);

        (self.status, Json(self.problem_details)).into_response()
    }
}

impl ApiError {
    #[must_use]
    pub fn new(status: StatusCode, details: ProblemDetails) -> Self {
        Self {
            status,
            problem_details: details.into(),
            _message: None,
        }
    }

    pub fn with_msg<M: Into<String>>(
        status: StatusCode,
        details: ProblemDetails,
        message: M,
    ) -> Self {
        Self {
            status,
            problem_details: details.into(),
            _message: Some(message.into()),
        }
    }
}

// CONVERSIONS

impl From<PoolError> for ApiError {
    fn from(e: PoolError) -> Self {
        ApiError::internal(e.to_string())
    }
}

impl From<tags::Error> for ApiError {
    fn from(value: tags::Error) -> Self {
        match value {
            tags::Error::DataStore(err) => ApiError::internal_err(err),
        }
    }
}

impl From<tei_filter::tags::Error> for ApiError {
    fn from(value: tei_filter::tags::Error) -> Self {
        match value {
            tei_filter::tags::Error::Meilisearch(err) => ApiError::internal_err(err),
        }
    }
}

// ERRORS

impl ApiError {
    pub fn internal<M: Into<String>>(msg: M) -> Self {
        ApiError::with_msg(
            StatusCode::INTERNAL_SERVER_ERROR,
            ProblemDetails::internal(),
            msg,
        )
    }

    pub fn internal_err<E: Debug + Display>(err: E) -> Self {
        ApiError::internal(err.to_string())
    }

    pub fn not_found<T: Into<String>>(id: T) -> Self {
        let details = ProblemDetails::not_found()
            .with_extension("id", id.into())
            .unwrap();
        ApiError::with_msg(
            StatusCode::NOT_FOUND,
            details,
            "hey! it's me, a log line for a developer".to_string(),
        )
    }
}

impl ProblemDetails {
    #[must_use]
    pub fn internal() -> Self {
        ProblemDetails::default()
            .with_type("urn:error:internal")
            .with_title("Internal server error")
            .with_details("An internal server error occurred while processing the request. Please try again later.")
    }

    #[must_use]
    pub fn not_found() -> Self {
        ProblemDetails::default()
            .with_type("urn:error:not-found")
            .with_title("Requested document not found")
    }
}
