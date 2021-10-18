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

/// An ergonomic service client for `AWSApplicationCostProfiler`.
///
/// This client allows ergonomic access to a `AWSApplicationCostProfiler`-shaped service.
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
    pub fn delete_report_definition(&self) -> fluent_builders::DeleteReportDefinition<C, M, R> {
        fluent_builders::DeleteReportDefinition::new(self.handle.clone())
    }
    pub fn get_report_definition(&self) -> fluent_builders::GetReportDefinition<C, M, R> {
        fluent_builders::GetReportDefinition::new(self.handle.clone())
    }
    pub fn import_application_usage(&self) -> fluent_builders::ImportApplicationUsage<C, M, R> {
        fluent_builders::ImportApplicationUsage::new(self.handle.clone())
    }
    pub fn list_report_definitions(&self) -> fluent_builders::ListReportDefinitions<C, M, R> {
        fluent_builders::ListReportDefinitions::new(self.handle.clone())
    }
    pub fn put_report_definition(&self) -> fluent_builders::PutReportDefinition<C, M, R> {
        fluent_builders::PutReportDefinition::new(self.handle.clone())
    }
    pub fn update_report_definition(&self) -> fluent_builders::UpdateReportDefinition<C, M, R> {
        fluent_builders::UpdateReportDefinition::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DeleteReportDefinition<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_report_definition_input::Builder,
    }
    impl<C, M, R> DeleteReportDefinition<C, M, R>
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
            crate::output::DeleteReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteReportDefinitionInputOperationOutputAlias,
                crate::output::DeleteReportDefinitionOutput,
                crate::error::DeleteReportDefinitionError,
                crate::input::DeleteReportDefinitionInputOperationRetryAlias,
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
        /// <p>Required. ID of the report to delete.</p>
        pub fn report_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(inp);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetReportDefinition<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_report_definition_input::Builder,
    }
    impl<C, M, R> GetReportDefinition<C, M, R>
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
            crate::output::GetReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::GetReportDefinitionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetReportDefinitionInputOperationOutputAlias,
                crate::output::GetReportDefinitionOutput,
                crate::error::GetReportDefinitionError,
                crate::input::GetReportDefinitionInputOperationRetryAlias,
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
        /// <p>ID of the report to retrieve.</p>
        pub fn report_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(inp);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ImportApplicationUsage<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::import_application_usage_input::Builder,
    }
    impl<C, M, R> ImportApplicationUsage<C, M, R>
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
            crate::output::ImportApplicationUsageOutput,
            smithy_http::result::SdkError<crate::error::ImportApplicationUsageError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ImportApplicationUsageInputOperationOutputAlias,
                crate::output::ImportApplicationUsageOutput,
                crate::error::ImportApplicationUsageError,
                crate::input::ImportApplicationUsageInputOperationRetryAlias,
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
        /// <p>Amazon S3 location to import application usage data from.</p>
        pub fn source_s3_location(mut self, inp: crate::model::SourceS3Location) -> Self {
            self.inner = self.inner.source_s3_location(inp);
            self
        }
        pub fn set_source_s3_location(
            mut self,
            input: std::option::Option<crate::model::SourceS3Location>,
        ) -> Self {
            self.inner = self.inner.set_source_s3_location(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListReportDefinitions<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_report_definitions_input::Builder,
    }
    impl<C, M, R> ListReportDefinitions<C, M, R>
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
            crate::output::ListReportDefinitionsOutput,
            smithy_http::result::SdkError<crate::error::ListReportDefinitionsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListReportDefinitionsInputOperationOutputAlias,
                crate::output::ListReportDefinitionsOutput,
                crate::error::ListReportDefinitionsError,
                crate::input::ListReportDefinitionsInputOperationRetryAlias,
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
        /// <p>The token value from a previous call to access the next page of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutReportDefinition<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_report_definition_input::Builder,
    }
    impl<C, M, R> PutReportDefinition<C, M, R>
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
            crate::output::PutReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::PutReportDefinitionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutReportDefinitionInputOperationOutputAlias,
                crate::output::PutReportDefinitionOutput,
                crate::error::PutReportDefinitionError,
                crate::input::PutReportDefinitionInputOperationRetryAlias,
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
        /// <p>Required. ID of the report. You can choose any valid string matching the pattern for the
        /// ID.</p>
        pub fn report_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(inp);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
        /// <p>Required. Description of the report.</p>
        pub fn report_description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_description(inp);
            self
        }
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_report_description(input);
            self
        }
        /// <p>Required. The cadence to generate the report.</p>
        pub fn report_frequency(mut self, inp: crate::model::ReportFrequency) -> Self {
            self.inner = self.inner.report_frequency(inp);
            self
        }
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.inner = self.inner.set_report_frequency(input);
            self
        }
        /// <p>Required. The format to use for the generated report.</p>
        pub fn format(mut self, inp: crate::model::Format) -> Self {
            self.inner = self.inner.format(inp);
            self
        }
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.inner = self.inner.set_format(input);
            self
        }
        /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
        /// report.</p>
        pub fn destination_s3_location(mut self, inp: crate::model::S3Location) -> Self {
            self.inner = self.inner.destination_s3_location(inp);
            self
        }
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_location(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateReportDefinition<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_report_definition_input::Builder,
    }
    impl<C, M, R> UpdateReportDefinition<C, M, R>
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
            crate::output::UpdateReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::UpdateReportDefinitionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateReportDefinitionInputOperationOutputAlias,
                crate::output::UpdateReportDefinitionOutput,
                crate::error::UpdateReportDefinitionError,
                crate::input::UpdateReportDefinitionInputOperationRetryAlias,
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
        /// <p>Required. ID of the report to update.</p>
        pub fn report_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(inp);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
        /// <p>Required. Description of the report.</p>
        pub fn report_description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_description(inp);
            self
        }
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_report_description(input);
            self
        }
        /// <p>Required. The cadence to generate the report.</p>
        pub fn report_frequency(mut self, inp: crate::model::ReportFrequency) -> Self {
            self.inner = self.inner.report_frequency(inp);
            self
        }
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.inner = self.inner.set_report_frequency(input);
            self
        }
        /// <p>Required. The format to use for the generated report.</p>
        pub fn format(mut self, inp: crate::model::Format) -> Self {
            self.inner = self.inner.format(inp);
            self
        }
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.inner = self.inner.set_format(input);
            self
        }
        /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
        /// report.</p>
        pub fn destination_s3_location(mut self, inp: crate::model::S3Location) -> Self {
            self.inner = self.inner.destination_s3_location(inp);
            self
        }
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_location(input);
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
