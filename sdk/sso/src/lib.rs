#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <p>AWS Single Sign-On Portal is a web service that makes it easy for you to assign user
//! access to AWS SSO resources such as the user portal. Users can get AWS account applications
//! and roles assigned to them and get federated into the application.</p>
//! <p>For general information about AWS SSO, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/what-is.html">What is AWS
//! Single Sign-On?</a> in the <i>AWS SSO User Guide</i>.</p>
//! <p>This API reference guide describes the AWS SSO Portal operations that you can call
//! programatically and includes detailed information on data types and errors.</p>
//! <note>
//! <p>AWS provides SDKs that consist of libraries and sample code for various programming
//! languages and platforms, such as Java, Ruby, .Net, iOS, or Android. The SDKs provide a
//! convenient way to create programmatic access to AWS SSO and other AWS services. For more
//! information about the AWS SDKs, including how to download and install them, see <a href="http://aws.amazon.com/tools/">Tools for Amazon Web Services</a>.</p>
//! </note>

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
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("sso", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;
