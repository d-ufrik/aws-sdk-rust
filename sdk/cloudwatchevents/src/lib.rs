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
//! <p>Amazon EventBridge helps you to respond to state changes in your Amazon Web Services resources. When your
//! resources change state, they automatically send events to an event stream. You can create
//! rules that match selected events in the stream and route them to targets to take action. You
//! can also use rules to take action on a predetermined schedule. For example, you can configure
//! rules to:</p>
//! <ul>
//! <li>
//! <p>Automatically invoke an Lambda function to update DNS entries when an event
//! notifies you that Amazon EC2 instance enters the running state.</p>
//! </li>
//! <li>
//! <p>Direct specific API records from CloudTrail to an Amazon Kinesis data stream for
//! detailed analysis of potential security or availability risks.</p>
//! </li>
//! <li>
//! <p>Periodically invoke a built-in target to create a snapshot of an Amazon EBS
//! volume.</p>
//! </li>
//! </ul>
//! <p>For more information about the features of Amazon EventBridge, see the <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide">Amazon EventBridge User
//! Guide</a>.</p>
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
    aws_http::user_agent::ApiMetadata::new("cloudwatchevents", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
