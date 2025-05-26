use thiserror::Error;

/// Result type for VictorOps API operations.
///
/// This is a convenience type alias that uses the [`Error`] type as the error variant.
pub type ApiResult<T> = std::result::Result<T, Error>;

/// Error types that can occur when using the VictorOps API client.
#[derive(Error, Debug)]
pub enum Error {
  /// HTTP request failed.
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  /// JSON serialization or deserialization failed.
  #[error("JSON serialization/deserialization failed: {0}")]
  Json(#[from] serde_json::Error),

  /// URL parsing failed.
  #[error("URL parsing failed: {0}")]
  UrlParse(#[from] url::ParseError),

  /// Invalid HTTP header value.
  #[error("Invalid header value: {0}")]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

  /// API returned an error response.
  #[error("API error: {status} - {message}")]
  Api { 
    /// The HTTP status code returned by the API.
    status: u16, 
    /// The error message returned by the API.
    message: String 
  },

  /// Authentication failed.
  #[error("Authentication failed")]
  Authentication,

  /// Requested resource was not found.
  #[error("Resource not found")]
  NotFound,

  /// Invalid input provided to the API.
  #[error("Invalid input: {0}")]
  InvalidInput(String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let api_error = Error::Api {
      status: 404,
      message: "Not found".to_string(),
    };
    assert_eq!(format!("{}", api_error), "API error: 404 - Not found");

    let auth_error = Error::Authentication;
    assert_eq!(format!("{}", auth_error), "Authentication failed");

    let not_found_error = Error::NotFound;
    assert_eq!(format!("{}", not_found_error), "Resource not found");

    let invalid_input_error = Error::InvalidInput("Bad data".to_string());
    assert_eq!(format!("{}", invalid_input_error), "Invalid input: Bad data");
  }

  #[test]
  fn test_error_debug() {
    let api_error = Error::Api {
      status: 500,
      message: "Internal error".to_string(),
    };
    let debug_str = format!("{:?}", api_error);
    assert!(debug_str.contains("Api"));
    assert!(debug_str.contains("500"));
  }
}
