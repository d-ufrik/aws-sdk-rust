#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>Cloud9</fullname>
//! <p>Cloud9 is a collection of tools that you can use to code, build, run, test, debug, and
//! release software in the cloud.</p>
//! <p>For more information about Cloud9, see the <a href="https://docs.aws.amazon.com/cloud9/latest/user-guide">Cloud9 User Guide</a>.</p>
//! <p>Cloud9 supports these operations:</p>
//! <ul>
//! <li>
//! <p>
//! <code>CreateEnvironmentEC2</code>: Creates an Cloud9 development environment, launches
//! an Amazon EC2 instance, and then connects from the instance to the environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>CreateEnvironmentMembership</code>: Adds an environment member to an
//! environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>DeleteEnvironment</code>: Deletes an environment. If an Amazon EC2 instance is
//! connected to the environment, also terminates the instance.</p>
//! </li>
//! <li>
//! <p>
//! <code>DeleteEnvironmentMembership</code>: Deletes an environment member from an
//! environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>DescribeEnvironmentMemberships</code>: Gets information about environment
//! members for an environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>DescribeEnvironments</code>: Gets information about environments.</p>
//! </li>
//! <li>
//! <p>
//! <code>DescribeEnvironmentStatus</code>: Gets status information for an
//! environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListEnvironments</code>: Gets a list of environment identifiers.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListTagsForResource</code>: Gets the tags for an environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>TagResource</code>: Adds tags to an environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>UntagResource</code>: Removes tags from an environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateEnvironment</code>: Changes the settings of an existing
//! environment.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateEnvironmentMembership</code>: Changes the settings of an existing
//! environment member for an environment.</p>
//! </li>
//! </ul>

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
    aws_http::user_agent::ApiMetadata::new("cloud9", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;
