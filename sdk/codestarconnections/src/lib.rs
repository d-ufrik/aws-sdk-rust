#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <fullname>AWS CodeStar Connections</fullname>
//! <p>This AWS CodeStar Connections API Reference provides descriptions and usage examples of
//! the operations and data types for the AWS CodeStar Connections API. You can use the
//! connections API to work with connections and installations.</p>
//! <p>
//! <i>Connections</i> are configurations that you use to connect AWS
//! resources to external code repositories. Each connection is a resource that can be given to
//! services such as CodePipeline to connect to a third-party repository such as Bitbucket. For
//! example, you can add the connection in CodePipeline so that it triggers your pipeline when a
//! code change is made to your third-party code repository. Each connection is named and
//! associated with a unique ARN that is used to reference the connection.</p>
//! <p>When you create a connection, the console initiates a third-party connection handshake.
//! <i>Installations</i> are the apps that are used to conduct this handshake. For
//! example, the installation for the Bitbucket provider type is the Bitbucket app. When you
//! create a connection, you can choose an existing installation or create one.</p>
//! <p>When you want to create a connection to an installed provider type such as GitHub
//! Enterprise Server, you create a <i>host</i> for your connections.</p>
//! <p>You can work with connections by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>CreateConnection</a>, which creates a uniquely named connection that can be
//! referenced by services such as CodePipeline.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteConnection</a>, which deletes the specified connection.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetConnection</a>, which returns information about the connection, including
//! the connection status.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListConnections</a>, which lists the connections associated with your
//! account.</p>
//! </li>
//! </ul>
//! <p>You can work with hosts by calling:</p>
//! <ul>
//! <li>
//! <p>
//! <a>CreateHost</a>, which creates a host that represents the infrastructure where your provider is installed.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteHost</a>, which deletes the specified host.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetHost</a>, which returns information about the host, including
//! the setup status.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListHosts</a>, which lists the hosts associated with your
//! account.</p>
//! </li>
//! </ul>
//! <p>You can work with tags in AWS CodeStar Connections by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>ListTagsForResource</a>, which gets information about AWS tags for a
//! specified Amazon Resource Name (ARN) in AWS CodeStar Connections.</p>
//! </li>
//! <li>
//! <p>
//! <a>TagResource</a>, which adds or updates tags for a resource in AWS CodeStar
//! Connections.</p>
//! </li>
//! <li>
//! <p>
//! <a>UntagResource</a>, which removes tags for a resource in AWS CodeStar
//! Connections.</p>
//! </li>
//! </ul>
//! <p>For information about how to use AWS CodeStar Connections, see the <a href="https://docs.aws.amazon.com/dtconsole/latest/userguide/welcome-connections.html">Developer Tools User
//! Guide</a>.</p>

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
    aws_http::user_agent::ApiMetadata::new("codestarconnections", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;
