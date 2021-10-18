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

/// An ergonomic service client for `AWSOrigamiServiceGatewayService`.
///
/// This client allows ergonomic access to a `AWSOrigamiServiceGatewayService`-shaped service.
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
    pub fn describe_report_definitions(
        &self,
    ) -> fluent_builders::DescribeReportDefinitions<C, M, R> {
        fluent_builders::DescribeReportDefinitions::new(self.handle.clone())
    }
    pub fn modify_report_definition(&self) -> fluent_builders::ModifyReportDefinition<C, M, R> {
        fluent_builders::ModifyReportDefinition::new(self.handle.clone())
    }
    pub fn put_report_definition(&self) -> fluent_builders::PutReportDefinition<C, M, R> {
        fluent_builders::PutReportDefinition::new(self.handle.clone())
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
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn report_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(inp);
            self
        }
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeReportDefinitions<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_report_definitions_input::Builder,
    }
    impl<C, M, R> DescribeReportDefinitions<C, M, R>
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
            crate::output::DescribeReportDefinitionsOutput,
            smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeReportDefinitionsInputOperationOutputAlias,
                crate::output::DescribeReportDefinitionsOutput,
                crate::error::DescribeReportDefinitionsError,
                crate::input::DescribeReportDefinitionsInputOperationRetryAlias,
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
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>A generic string.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ModifyReportDefinition<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::modify_report_definition_input::Builder,
    }
    impl<C, M, R> ModifyReportDefinition<C, M, R>
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
            crate::output::ModifyReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ModifyReportDefinitionInputOperationOutputAlias,
                crate::output::ModifyReportDefinitionOutput,
                crate::error::ModifyReportDefinitionError,
                crate::input::ModifyReportDefinitionInputOperationRetryAlias,
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
        /// <p>The name of the report that you want to create. The name must be unique,
        /// is case sensitive, and can't include spaces. </p>
        pub fn report_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(inp);
            self
        }
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name,
        /// time unit, report format, compression format, S3 bucket, additional artifacts, and schema
        /// elements in the definition.
        /// </p>
        pub fn report_definition(mut self, inp: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(inp);
            self
        }
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
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
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed
        /// metadata and data file information. </p>
        pub fn report_definition(mut self, inp: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(inp);
            self
        }
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
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
