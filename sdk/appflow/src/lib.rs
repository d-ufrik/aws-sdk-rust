#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the Amazon AppFlow API reference. This guide is for developers who need
//! detailed information about the Amazon AppFlow API operations, data types, and errors. </p>
//! <p>Amazon AppFlow is a fully managed integration service that enables you to securely
//! transfer data between software as a service (SaaS) applications like Salesforce, Marketo,
//! Slack, and ServiceNow, and Amazon Web Services like Amazon S3 and Amazon Redshift. </p>
//! <p>Use the following links to get started on the Amazon AppFlow API:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/appflow/1.0/APIReference/API_Operations.html">Actions</a>: An alphabetical list of all Amazon AppFlow API
//! operations.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/appflow/1.0/APIReference/API_Types.html">Data
//! types</a>: An alphabetical list of all Amazon AppFlow data types.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/appflow/1.0/APIReference/CommonParameters.html">Common parameters</a>: Parameters that all Query operations can use.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/appflow/1.0/APIReference/CommonErrors.html">Common
//! errors</a>: Client and server errors that all operations can return.</p>
//! </li>
//! </ul>
//! <p>If you're new to Amazon AppFlow, we recommend that you review the <a href="https://docs.aws.amazon.com/appflow/latest/userguide/what-is-appflow.html">Amazon AppFlow User Guide</a>.</p>
//! <p>Amazon AppFlow API users can use vendor-specific mechanisms for OAuth, and include
//! applicable OAuth attributes (such as <code>auth-code</code> and <code>redirecturi</code>) with
//! the connector-specific <code>ConnectorProfileProperties</code> when creating a new connector
//! profile using Amazon AppFlow API operations. For example, Salesforce users can refer to
//! the <a href="https://help.salesforce.com/articleView?id=remoteaccess_authenticate.htm">
//! <i>Authorize Apps with OAuth</i>
//! </a> documentation.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Wrap operations in a special type allowing for the modification of operations and the requests inside before sending them
pub mod customizable_operation;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
pub use aws_smithy_async::rt::sleep::AsyncSleep;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_smithy_types::timeout::Config as TimeoutConfig;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("appflow", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
