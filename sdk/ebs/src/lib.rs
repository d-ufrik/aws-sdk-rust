#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <p>You can use the Amazon Elastic Block Store (Amazon EBS) direct APIs to create Amazon EBS snapshots, write data directly to
//! your snapshots, read data on your snapshots, and identify the differences or changes between
//! two snapshots. If you’re an independent software vendor (ISV) who offers backup services for
//! Amazon EBS, the EBS direct APIs make it more efficient and cost-effective to track incremental changes on
//! your Amazon EBS volumes through snapshots. This can be done without having to create new volumes
//! from snapshots, and then use Amazon Elastic Compute Cloud (Amazon EC2) instances to compare the differences.</p>
//! <p>You can create incremental snapshots directly from data on-premises into volumes and the
//! cloud to use for quick disaster recovery. With the ability to write and read snapshots, you can
//! write your on-premises data to an snapshot during a disaster. Then after recovery, you can
//! restore it back to Amazon Web Services or on-premises from the snapshot. You no longer need to build and
//! maintain complex mechanisms to copy data to and from Amazon EBS.</p>
//! <p>This API reference provides detailed information about the actions, data types,
//! parameters, and errors of the EBS direct APIs. For more information about the elements that
//! make up the EBS direct APIs, and examples of how to use them effectively, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html">Accessing the Contents of an Amazon EBS Snapshot</a> in the <i>Amazon Elastic Compute Cloud User
//! Guide</i>. For more information about the supported Amazon Web Services Regions, endpoints,
//! and service quotas for the EBS direct APIs, see <a href="https://docs.aws.amazon.com/general/latest/gr/ebs-service.html">Amazon Elastic Block Store Endpoints and Quotas</a> in
//! the <i>Amazon Web Services General Reference</i>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
mod idempotency_token;
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
    aws_http::user_agent::ApiMetadata::new("ebs", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;
