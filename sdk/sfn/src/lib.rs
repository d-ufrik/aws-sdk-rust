#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>AWS Step Functions</fullname>
//! <p>AWS Step Functions is a service that lets you coordinate the components of distributed applications
//! and microservices using visual workflows.</p>
//! <p>You can use Step Functions to build applications from individual components, each of which performs
//! a discrete function, or <i>task</i>, allowing you to scale and change
//! applications quickly. Step Functions provides a console that helps visualize the components of your
//! application as a series of steps. Step Functions automatically triggers and tracks each step, and
//! retries steps when there are errors, so your application executes predictably and in the right
//! order every time. Step Functions logs the state of each step, so you can quickly diagnose and debug any
//! issues.</p>
//! <p>Step Functions manages operations and underlying infrastructure to ensure your application is
//! available at any scale. You can run tasks on AWS, your own servers, or any system that has
//! access to AWS. You can access and use Step Functions using the console, the AWS SDKs, or an HTTP API.
//! For more information about Step Functions, see the <i>
//! <a href="https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html">AWS Step Functions Developer Guide</a>
//! </i>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("sfn", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;
