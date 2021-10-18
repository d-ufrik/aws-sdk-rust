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

/// An ergonomic service client for `PinpointSMSVoice`.
///
/// This client allows ergonomic access to a `PinpointSMSVoice`-shaped service.
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
    pub fn create_configuration_set(&self) -> fluent_builders::CreateConfigurationSet<C, M, R> {
        fluent_builders::CreateConfigurationSet::new(self.handle.clone())
    }
    pub fn create_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::CreateConfigurationSetEventDestination<C, M, R> {
        fluent_builders::CreateConfigurationSetEventDestination::new(self.handle.clone())
    }
    pub fn delete_configuration_set(&self) -> fluent_builders::DeleteConfigurationSet<C, M, R> {
        fluent_builders::DeleteConfigurationSet::new(self.handle.clone())
    }
    pub fn delete_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::DeleteConfigurationSetEventDestination<C, M, R> {
        fluent_builders::DeleteConfigurationSetEventDestination::new(self.handle.clone())
    }
    pub fn get_configuration_set_event_destinations(
        &self,
    ) -> fluent_builders::GetConfigurationSetEventDestinations<C, M, R> {
        fluent_builders::GetConfigurationSetEventDestinations::new(self.handle.clone())
    }
    pub fn list_configuration_sets(&self) -> fluent_builders::ListConfigurationSets<C, M, R> {
        fluent_builders::ListConfigurationSets::new(self.handle.clone())
    }
    pub fn send_voice_message(&self) -> fluent_builders::SendVoiceMessage<C, M, R> {
        fluent_builders::SendVoiceMessage::new(self.handle.clone())
    }
    pub fn update_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::UpdateConfigurationSetEventDestination<C, M, R> {
        fluent_builders::UpdateConfigurationSetEventDestination::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateConfigurationSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_configuration_set_input::Builder,
    }
    impl<C, M, R> CreateConfigurationSet<C, M, R>
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
            crate::output::CreateConfigurationSetOutput,
            smithy_http::result::SdkError<crate::error::CreateConfigurationSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateConfigurationSetInputOperationOutputAlias,
                crate::output::CreateConfigurationSetOutput,
                crate::error::CreateConfigurationSetError,
                crate::input::CreateConfigurationSetInputOperationRetryAlias,
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
        /// The name that you want to give the configuration set.
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct CreateConfigurationSetEventDestination<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> CreateConfigurationSetEventDestination<C, M, R>
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
            crate::output::CreateConfigurationSetEventDestinationOutput,
            smithy_http::result::SdkError<
                crate::error::CreateConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::CreateConfigurationSetEventDestinationOutput,
                crate::error::CreateConfigurationSetEventDestinationError,
                crate::input::CreateConfigurationSetEventDestinationInputOperationRetryAlias,
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
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that defines a single event destination.
        pub fn event_destination(mut self, inp: crate::model::EventDestinationDefinition) -> Self {
            self.inner = self.inner.event_destination(inp);
            self
        }
        pub fn set_event_destination(
            mut self,
            input: std::option::Option<crate::model::EventDestinationDefinition>,
        ) -> Self {
            self.inner = self.inner.set_event_destination(input);
            self
        }
        /// A name that identifies the event destination.
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteConfigurationSet<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_configuration_set_input::Builder,
    }
    impl<C, M, R> DeleteConfigurationSet<C, M, R>
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
            crate::output::DeleteConfigurationSetOutput,
            smithy_http::result::SdkError<crate::error::DeleteConfigurationSetError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteConfigurationSetInputOperationOutputAlias,
                crate::output::DeleteConfigurationSetOutput,
                crate::error::DeleteConfigurationSetError,
                crate::input::DeleteConfigurationSetInputOperationRetryAlias,
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
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteConfigurationSetEventDestination<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> DeleteConfigurationSetEventDestination<C, M, R>
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
            crate::output::DeleteConfigurationSetEventDestinationOutput,
            smithy_http::result::SdkError<
                crate::error::DeleteConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::DeleteConfigurationSetEventDestinationOutput,
                crate::error::DeleteConfigurationSetEventDestinationError,
                crate::input::DeleteConfigurationSetEventDestinationInputOperationRetryAlias,
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
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// EventDestinationName
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetConfigurationSetEventDestinations<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_configuration_set_event_destinations_input::Builder,
    }
    impl<C, M, R> GetConfigurationSetEventDestinations<C, M, R>
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
            crate::output::GetConfigurationSetEventDestinationsOutput,
            smithy_http::result::SdkError<crate::error::GetConfigurationSetEventDestinationsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetConfigurationSetEventDestinationsInputOperationOutputAlias,
                crate::output::GetConfigurationSetEventDestinationsOutput,
                crate::error::GetConfigurationSetEventDestinationsError,
                crate::input::GetConfigurationSetEventDestinationsInputOperationRetryAlias,
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
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListConfigurationSets<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_configuration_sets_input::Builder,
    }
    impl<C, M, R> ListConfigurationSets<C, M, R>
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
            crate::output::ListConfigurationSetsOutput,
            smithy_http::result::SdkError<crate::error::ListConfigurationSetsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListConfigurationSetsInputOperationOutputAlias,
                crate::output::ListConfigurationSetsOutput,
                crate::error::ListConfigurationSetsError,
                crate::input::ListConfigurationSetsInputOperationRetryAlias,
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
        /// A token returned from a previous call to the API that indicates the position in the list of results.
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Used to specify the number of items that should be returned in the response.
        pub fn page_size(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.page_size(inp);
            self
        }
        pub fn set_page_size(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_page_size(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct SendVoiceMessage<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_voice_message_input::Builder,
    }
    impl<C, M, R> SendVoiceMessage<C, M, R>
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
            crate::output::SendVoiceMessageOutput,
            smithy_http::result::SdkError<crate::error::SendVoiceMessageError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendVoiceMessageInputOperationOutputAlias,
                crate::output::SendVoiceMessageOutput,
                crate::error::SendVoiceMessageError,
                crate::input::SendVoiceMessageInputOperationRetryAlias,
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
        /// The phone number that appears on recipients' devices when they receive the message.
        pub fn caller_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.caller_id(inp);
            self
        }
        pub fn set_caller_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_caller_id(input);
            self
        }
        /// The name of the configuration set that you want to use to send the message.
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that contains a voice message and information about the recipient that you want to send it to.
        pub fn content(mut self, inp: crate::model::VoiceMessageContent) -> Self {
            self.inner = self.inner.content(inp);
            self
        }
        pub fn set_content(
            mut self,
            input: std::option::Option<crate::model::VoiceMessageContent>,
        ) -> Self {
            self.inner = self.inner.set_content(input);
            self
        }
        /// The phone number that you want to send the voice message to.
        pub fn destination_phone_number(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_phone_number(inp);
            self
        }
        pub fn set_destination_phone_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_phone_number(input);
            self
        }
        /// The phone number that Amazon Pinpoint should use to send the voice message. This isn't necessarily the phone number that appears on recipients' devices when they receive the message, because you can specify a CallerId parameter in the request.
        pub fn origination_phone_number(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.origination_phone_number(inp);
            self
        }
        pub fn set_origination_phone_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_origination_phone_number(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateConfigurationSetEventDestination<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> UpdateConfigurationSetEventDestination<C, M, R>
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
            crate::output::UpdateConfigurationSetEventDestinationOutput,
            smithy_http::result::SdkError<
                crate::error::UpdateConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::UpdateConfigurationSetEventDestinationOutput,
                crate::error::UpdateConfigurationSetEventDestinationError,
                crate::input::UpdateConfigurationSetEventDestinationInputOperationRetryAlias,
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
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that defines a single event destination.
        pub fn event_destination(mut self, inp: crate::model::EventDestinationDefinition) -> Self {
            self.inner = self.inner.event_destination(inp);
            self
        }
        pub fn set_event_destination(
            mut self,
            input: std::option::Option<crate::model::EventDestinationDefinition>,
        ) -> Self {
            self.inner = self.inner.set_event_destination(input);
            self
        }
        /// EventDestinationName
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
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
