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

/// An ergonomic service client for `IotLaserThingJobManagerExternalService`.
///
/// This client allows ergonomic access to a `IotLaserThingJobManagerExternalService`-shaped service.
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
    pub fn describe_job_execution(&self) -> fluent_builders::DescribeJobExecution<C, M, R> {
        fluent_builders::DescribeJobExecution::new(self.handle.clone())
    }
    pub fn get_pending_job_executions(&self) -> fluent_builders::GetPendingJobExecutions<C, M, R> {
        fluent_builders::GetPendingJobExecutions::new(self.handle.clone())
    }
    pub fn start_next_pending_job_execution(
        &self,
    ) -> fluent_builders::StartNextPendingJobExecution<C, M, R> {
        fluent_builders::StartNextPendingJobExecution::new(self.handle.clone())
    }
    pub fn update_job_execution(&self) -> fluent_builders::UpdateJobExecution<C, M, R> {
        fluent_builders::UpdateJobExecution::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DescribeJobExecution<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_job_execution_input::Builder,
    }
    impl<C, M, R> DescribeJobExecution<C, M, R>
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
            crate::output::DescribeJobExecutionOutput,
            smithy_http::result::SdkError<crate::error::DescribeJobExecutionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeJobExecutionInputOperationOutputAlias,
                crate::output::DescribeJobExecutionOutput,
                crate::error::DescribeJobExecutionError,
                crate::input::DescribeJobExecutionInputOperationRetryAlias,
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
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn job_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(inp);
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The thing name associated with the device the job execution is running on.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn include_job_document(mut self, inp: bool) -> Self {
            self.inner = self.inner.include_job_document(inp);
            self
        }
        pub fn set_include_job_document(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_document(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device. If not specified,
        /// the latest job execution is returned.</p>
        pub fn execution_number(mut self, inp: i64) -> Self {
            self.inner = self.inner.execution_number(inp);
            self
        }
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_execution_number(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetPendingJobExecutions<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_pending_job_executions_input::Builder,
    }
    impl<C, M, R> GetPendingJobExecutions<C, M, R>
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
            crate::output::GetPendingJobExecutionsOutput,
            smithy_http::result::SdkError<crate::error::GetPendingJobExecutionsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetPendingJobExecutionsInputOperationOutputAlias,
                crate::output::GetPendingJobExecutionsOutput,
                crate::error::GetPendingJobExecutionsError,
                crate::input::GetPendingJobExecutionsInputOperationRetryAlias,
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
        /// <p>The name of the thing that is executing the job.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartNextPendingJobExecution<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_next_pending_job_execution_input::Builder,
    }
    impl<C, M, R> StartNextPendingJobExecution<C, M, R>
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
            crate::output::StartNextPendingJobExecutionOutput,
            smithy_http::result::SdkError<crate::error::StartNextPendingJobExecutionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartNextPendingJobExecutionInputOperationOutputAlias,
                crate::output::StartNextPendingJobExecutionOutput,
                crate::error::StartNextPendingJobExecutionError,
                crate::input::StartNextPendingJobExecutionInputOperationRetryAlias,
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
        /// <p>The name of the thing associated with the device.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// Adds a key-value pair to `statusDetails`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        /// <p>A collection of name/value pairs that describe the status of the job execution. If not specified, the
        /// statusDetails are unchanged.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.status_details(k, v);
            self
        }
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_status_details(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job
        /// execution status is not set to a terminal state before this timer expires, or before the
        /// timer is reset (by calling <code>UpdateJobExecution</code>, setting the status to
        /// <code>IN_PROGRESS</code> and specifying a new timeout value in field <code>stepTimeoutInMinutes</code>)
        /// the job execution status will be automatically set to <code>TIMED_OUT</code>.  Note that setting
        /// this timeout has no effect on that job execution timeout which may have been specified when
        /// the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn step_timeout_in_minutes(mut self, inp: i64) -> Self {
            self.inner = self.inner.step_timeout_in_minutes(inp);
            self
        }
        pub fn set_step_timeout_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_step_timeout_in_minutes(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateJobExecution<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_job_execution_input::Builder,
    }
    impl<C, M, R> UpdateJobExecution<C, M, R>
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
            crate::output::UpdateJobExecutionOutput,
            smithy_http::result::SdkError<crate::error::UpdateJobExecutionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateJobExecutionInputOperationOutputAlias,
                crate::output::UpdateJobExecutionOutput,
                crate::error::UpdateJobExecutionError,
                crate::input::UpdateJobExecutionInputOperationRetryAlias,
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
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn job_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(inp);
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The name of the thing associated with the device.</p>
        pub fn thing_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(inp);
            self
        }
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED). This must be specified
        /// on every update.</p>
        pub fn status(mut self, inp: crate::model::JobExecutionStatus) -> Self {
            self.inner = self.inner.status(inp);
            self
        }
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::JobExecutionStatus>,
        ) -> Self {
            self.inner = self.inner.set_status(input);
            self
        }
        /// Adds a key-value pair to `statusDetails`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        /// <p> Optional. A collection of name/value pairs that describe the status of the job execution. If not
        /// specified, the statusDetails are unchanged.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.status_details(k, v);
            self
        }
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_status_details(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job
        /// execution status is not set to a terminal state before this timer expires, or before the
        /// timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to
        /// <code>IN_PROGRESS</code> and specifying a new timeout value in this field) the job execution
        /// status will be automatically set to <code>TIMED_OUT</code>.  Note that setting or resetting
        /// this timeout has no effect on that job execution timeout which may have been specified when
        /// the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn step_timeout_in_minutes(mut self, inp: i64) -> Self {
            self.inner = self.inner.step_timeout_in_minutes(inp);
            self
        }
        pub fn set_step_timeout_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_step_timeout_in_minutes(input);
            self
        }
        /// <p>Optional. The expected current version of the job execution. Each time you update the job execution, its
        /// version is incremented. If the version of the job execution stored in Jobs does not match, the update is
        /// rejected with a VersionMismatch error, and an ErrorResponse that contains the current job execution status data
        /// is returned. (This makes it unnecessary to perform a separate DescribeJobExecution request in order to obtain
        /// the job execution status data.)</p>
        pub fn expected_version(mut self, inp: i64) -> Self {
            self.inner = self.inner.expected_version(inp);
            self
        }
        pub fn set_expected_version(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_expected_version(input);
            self
        }
        /// <p>Optional. When included and set to true, the response contains the JobExecutionState data. The default is
        /// false.</p>
        pub fn include_job_execution_state(mut self, inp: bool) -> Self {
            self.inner = self.inner.include_job_execution_state(inp);
            self
        }
        pub fn set_include_job_execution_state(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_execution_state(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn include_job_document(mut self, inp: bool) -> Self {
            self.inner = self.inner.include_job_document(inp);
            self
        }
        pub fn set_include_job_document(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_document(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device.</p>
        pub fn execution_number(mut self, inp: i64) -> Self {
            self.inner = self.inner.execution_number(inp);
            self
        }
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_execution_number(input);
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
