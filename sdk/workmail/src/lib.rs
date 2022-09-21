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
//! <p>Amazon WorkMail is a secure, managed business email and calendaring service with support for
//! existing desktop and mobile email clients. You can access your email, contacts, and
//! calendars using Microsoft Outlook, your browser, or other native iOS and Android email
//! applications. You can integrate WorkMail with your existing corporate directory and control
//! both the keys that encrypt your data and the location in which your data is
//! stored.</p>
//! <p>The WorkMail API is designed for the following scenarios:</p>
//! <ul>
//! <li>
//! <p>Listing and describing organizations</p>
//! </li>
//! </ul>
//! <ul>
//! <li>
//! <p>Managing users</p>
//! </li>
//! </ul>
//! <ul>
//! <li>
//! <p>Managing groups</p>
//! </li>
//! </ul>
//! <ul>
//! <li>
//! <p>Managing resources</p>
//! </li>
//! </ul>
//! <p>All WorkMail API operations are Amazon-authenticated and certificate-signed. They not
//! only require the use of the AWS SDK, but also allow for the exclusive use of AWS Identity and Access Management
//! users and roles to help facilitate access, trust, and permission policies. By creating a
//! role and allowing an IAM user to access the WorkMail site, the IAM user gains full
//! administrative visibility into the entire WorkMail organization (or as set in the IAM
//! policy). This includes, but is not limited to, the ability to create, update, and delete
//! users, groups, and resources. This allows developers to perform the scenarios listed above,
//! as well as give users the ability to grant access on a selective basis using the IAM
//! model.</p>
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
mod idempotency_token;
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
    aws_http::user_agent::ApiMetadata::new("workmail", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
