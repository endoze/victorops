#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// VictorOps API client implementation.
pub mod client;

/// Error types and result handling for the VictorOps API.
pub mod error;

/// Type definitions for VictorOps API requests and responses.
pub mod types;

/// Main HTTP client for interacting with the VictorOps API.
pub use client::Client;

/// Result type and error types for VictorOps API operations.
pub use error::{ApiResult, Error};

/// All type definitions for VictorOps API data structures.
pub use types::*;
