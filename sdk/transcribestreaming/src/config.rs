// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub struct Config {
    pub(crate) retry_config: Option<smithy_types::retry::RetryConfig>,
    pub(crate) endpoint_resolver: ::std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>,
    pub(crate) region: Option<aws_types::region::Region>,
    pub(crate) credentials_provider: aws_types::credentials::SharedCredentialsProvider,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut config = f.debug_struct("Config");
        config.finish()
    }
}
impl Config {
    pub fn builder() -> Builder {
        Builder::default()
    }
    pub fn new(config: &aws_types::config::Config) -> Self {
        Builder::from(config).build()
    }
    /// The signature version 4 service signing name to use in the credential scope when signing requests.
    ///
    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
    /// [`SigningService`](aws_types::SigningService) during operation construction
    pub fn signing_service(&self) -> &'static str {
        "transcribe"
    }
    /// Creates a new Event Stream `SignMessage` implementor.
    pub fn new_event_stream_signer(
        &self,
        properties: smithy_http::property_bag::SharedPropertyBag,
    ) -> aws_sig_auth::event_stream::SigV4Signer {
        aws_sig_auth::event_stream::SigV4Signer::new(properties)
    }
}
#[derive(Default)]
pub struct Builder {
    retry_config: Option<smithy_types::retry::RetryConfig>,
    endpoint_resolver: Option<::std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>>,
    region: Option<aws_types::region::Region>,
    credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>,
}
impl Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_sdk_transcribestreaming::config::Config;
    /// use smithy_types::retry::RetryConfig;
    ///
    /// let retry_config = RetryConfig::new().with_max_attempts(5);
    /// let config = Config::builder().retry_config(retry_config).build();
    /// ```
    pub fn retry_config(mut self, retry_config: smithy_types::retry::RetryConfig) -> Self {
        self.set_retry_config(Some(retry_config));
        self
    }

    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_sdk_transcribestreaming::config::{Builder, Config};
    /// use smithy_types::retry::RetryConfig;
    ///
    /// fn disable_retries(builder: &mut Builder) {
    ///     let retry_config = RetryConfig::new().with_max_attempts(1);
    ///     builder.set_retry_config(Some(retry_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// disable_retries(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_retry_config(
        &mut self,
        retry_config: Option<smithy_types::retry::RetryConfig>,
    ) -> &mut Self {
        self.retry_config = retry_config;
        self
    }
    pub fn endpoint_resolver(
        mut self,
        endpoint_resolver: impl aws_endpoint::ResolveAwsEndpoint + 'static,
    ) -> Self {
        self.endpoint_resolver = Some(::std::sync::Arc::new(endpoint_resolver));
        self
    }
    pub fn region(mut self, region: impl Into<Option<aws_types::region::Region>>) -> Self {
        self.region = region.into();
        self
    }
    /// Set the credentials provider for this service
    pub fn credentials_provider(
        mut self,
        credentials_provider: impl aws_types::credentials::ProvideCredentials + 'static,
    ) -> Self {
        self.credentials_provider = Some(aws_types::credentials::SharedCredentialsProvider::new(
            credentials_provider,
        ));
        self
    }

    pub fn set_credentials_provider(
        &mut self,
        credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>,
    ) -> &mut Self {
        self.credentials_provider = credentials_provider;
        self
    }
    pub fn build(self) -> Config {
        Config {
            retry_config: self.retry_config,
            endpoint_resolver: self
                .endpoint_resolver
                .unwrap_or_else(|| ::std::sync::Arc::new(crate::aws_endpoint::endpoint_resolver())),
            region: self.region,
            credentials_provider: self.credentials_provider.unwrap_or_else(|| {
                aws_types::credentials::SharedCredentialsProvider::new(
                    crate::no_credentials::NoCredentials,
                )
            }),
        }
    }
}

impl From<&aws_types::config::Config> for Builder {
    fn from(input: &aws_types::config::Config) -> Self {
        let mut builder = Builder::default();
        builder = builder.region(input.region().cloned());
        builder.set_retry_config(input.retry_config().cloned());
        builder.set_credentials_provider(input.credentials_provider().cloned());
        builder
    }
}

impl From<&aws_types::config::Config> for Config {
    fn from(config: &aws_types::config::Config) -> Self {
        Builder::from(config).build()
    }
}
