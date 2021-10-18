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

/// An ergonomic service client for `AWSSSOOIDCService`.
///
/// This client allows ergonomic access to a `AWSSSOOIDCService`-shaped service.
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
    pub fn create_token(&self) -> fluent_builders::CreateToken<C, M, R> {
        fluent_builders::CreateToken::new(self.handle.clone())
    }
    pub fn register_client(&self) -> fluent_builders::RegisterClient<C, M, R> {
        fluent_builders::RegisterClient::new(self.handle.clone())
    }
    pub fn start_device_authorization(&self) -> fluent_builders::StartDeviceAuthorization<C, M, R> {
        fluent_builders::StartDeviceAuthorization::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateToken<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_token_input::Builder,
    }
    impl<C, M, R> CreateToken<C, M, R>
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
            crate::output::CreateTokenOutput,
            smithy_http::result::SdkError<crate::error::CreateTokenError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateTokenInputOperationOutputAlias,
                crate::output::CreateTokenOutput,
                crate::error::CreateTokenError,
                crate::input::CreateTokenInputOperationRetryAlias,
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
        /// <p>The unique identifier string for each client. This value should come from the persisted result of the <a>RegisterClient</a> API.</p>
        pub fn client_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(inp);
            self
        }
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>A secret string generated for the client. This value should come from the persisted result of the <a>RegisterClient</a> API.</p>
        pub fn client_secret(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_secret(inp);
            self
        }
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_secret(input);
            self
        }
        /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
        pub fn grant_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.grant_type(inp);
            self
        }
        pub fn set_grant_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_grant_type(input);
            self
        }
        /// <p>Used only when calling this API for the device code grant type. This short-term code is
        /// used to identify this authentication attempt. This should come from an in-memory reference to
        /// the result of the <a>StartDeviceAuthorization</a> API.</p>
        pub fn device_code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_code(inp);
            self
        }
        pub fn set_device_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_code(input);
            self
        }
        /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
        pub fn code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.code(inp);
            self
        }
        pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_code(input);
            self
        }
        /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
        pub fn refresh_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.refresh_token(inp);
            self
        }
        pub fn set_refresh_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_refresh_token(input);
            self
        }
        /// Appends an item to `scope`.
        ///
        /// To override the contents of this collection use [`set_scope`](Self::set_scope).
        /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to
        /// restrict permissions when granting an access token.</p>
        pub fn scope(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.scope(inp);
            self
        }
        pub fn set_scope(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_scope(input);
            self
        }
        /// <p>The location of the application that will receive the authorization code. Users authorize
        /// the service to send the request to this location.</p>
        pub fn redirect_uri(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.redirect_uri(inp);
            self
        }
        pub fn set_redirect_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_redirect_uri(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct RegisterClient<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::register_client_input::Builder,
    }
    impl<C, M, R> RegisterClient<C, M, R>
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
            crate::output::RegisterClientOutput,
            smithy_http::result::SdkError<crate::error::RegisterClientError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::RegisterClientInputOperationOutputAlias,
                crate::output::RegisterClientOutput,
                crate::error::RegisterClientError,
                crate::input::RegisterClientInputOperationRetryAlias,
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
        /// <p>The friendly name of the client.</p>
        pub fn client_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_name(inp);
            self
        }
        pub fn set_client_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_name(input);
            self
        }
        /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
        pub fn client_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_type(inp);
            self
        }
        pub fn set_client_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_type(input);
            self
        }
        /// Appends an item to `scopes`.
        ///
        /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
        /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used
        /// to restrict permissions when granting an access token.</p>
        pub fn scopes(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.scopes(inp);
            self
        }
        pub fn set_scopes(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_scopes(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartDeviceAuthorization<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_device_authorization_input::Builder,
    }
    impl<C, M, R> StartDeviceAuthorization<C, M, R>
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
            crate::output::StartDeviceAuthorizationOutput,
            smithy_http::result::SdkError<crate::error::StartDeviceAuthorizationError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartDeviceAuthorizationInputOperationOutputAlias,
                crate::output::StartDeviceAuthorizationOutput,
                crate::error::StartDeviceAuthorizationError,
                crate::input::StartDeviceAuthorizationInputOperationRetryAlias,
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
        /// <p>The unique identifier string for the client that is registered with AWS SSO. This value
        /// should come from the persisted result of the <a>RegisterClient</a> API
        /// operation.</p>
        pub fn client_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(inp);
            self
        }
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>A secret string that is generated for the client. This value should come from the
        /// persisted result of the <a>RegisterClient</a> API operation.</p>
        pub fn client_secret(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_secret(inp);
            self
        }
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_secret(input);
            self
        }
        /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using
        /// the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
        pub fn start_url(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.start_url(inp);
            self
        }
        pub fn set_start_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_start_url(input);
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
