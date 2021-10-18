// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AWSAcuitySignalingService`.
///
/// This client allows ergonomic access to a `AWSAcuitySignalingService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn get_ice_server_config(&self) -> fluent_builders::GetIceServerConfig<C, M, R> {
        fluent_builders::GetIceServerConfig::new(self.handle.clone())
    }
    pub fn send_alexa_offer_to_master(&self) -> fluent_builders::SendAlexaOfferToMaster<C, M, R> {
        fluent_builders::SendAlexaOfferToMaster::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct GetIceServerConfig<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_ice_server_config_input::Builder,
    }
    impl<C, M, R> GetIceServerConfig<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetIceServerConfigOutput,
            smithy_http::result::SdkError<crate::error::GetIceServerConfigError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetIceServerConfigInputOperationOutputAlias,
                crate::output::GetIceServerConfigOutput,
                crate::error::GetIceServerConfigError,
                crate::input::GetIceServerConfigInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between
        /// configured peers. </p>
        pub fn channel_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(inp);
            self
        }
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn client_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(inp);
            self
        }
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid
        /// value.</p>
        pub fn service(mut self, inp: crate::model::Service) -> Self {
            self.inner = self.inner.service(inp);
            self
        }
        pub fn set_service(mut self, input: std::option::Option<crate::model::Service>) -> Self {
            self.inner = self.inner.set_service(input);
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn username(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.username(inp);
            self
        }
        pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_username(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct SendAlexaOfferToMaster<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_alexa_offer_to_master_input::Builder,
    }
    impl<C, M, R> SendAlexaOfferToMaster<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendAlexaOfferToMasterOutput,
            smithy_http::result::SdkError<crate::error::SendAlexaOfferToMasterError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendAlexaOfferToMasterInputOperationOutputAlias,
                crate::output::SendAlexaOfferToMasterOutput,
                crate::error::SendAlexaOfferToMasterError,
                crate::input::SendAlexaOfferToMasterInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ARN of the signaling channel by which Alexa and the master peer
        /// communicate.</p>
        pub fn channel_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(inp);
            self
        }
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn sender_client_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sender_client_id(inp);
            self
        }
        pub fn set_sender_client_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sender_client_id(input);
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn message_payload(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_payload(inp);
            self
        }
        pub fn set_message_payload(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_message_payload(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::new(conn).with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::https().with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
