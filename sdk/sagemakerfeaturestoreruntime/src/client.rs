// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon SageMaker Feature Store Runtime
///
/// Client for invoking operations on Amazon SageMaker Feature Store Runtime. Each operation on Amazon SageMaker Feature Store Runtime is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_sagemakerfeaturestoreruntime::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_sagemakerfeaturestoreruntime::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_sagemakerfeaturestoreruntime::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`BatchGetRecord`](crate::client::fluent_builders::BatchGetRecord) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifiers(Vec<BatchGetRecordIdentifier>)`](crate::client::fluent_builders::BatchGetRecord::identifiers) / [`set_identifiers(Option<Vec<BatchGetRecordIdentifier>>)`](crate::client::fluent_builders::BatchGetRecord::set_identifiers): <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name that have been requested to be retrieved in batch.</p>
    /// - On success, responds with [`BatchGetRecordOutput`](crate::output::BatchGetRecordOutput) with field(s):
    ///   - [`records(Option<Vec<BatchGetRecordResultDetail>>)`](crate::output::BatchGetRecordOutput::records): <p>A list of Records you requested to be retrieved in batch.</p>
    ///   - [`errors(Option<Vec<BatchGetRecordError>>)`](crate::output::BatchGetRecordOutput::errors): <p>A list of errors that have occured when retrieving a batch of Records.</p>
    ///   - [`unprocessed_identifiers(Option<Vec<BatchGetRecordIdentifier>>)`](crate::output::BatchGetRecordOutput::unprocessed_identifiers): <p>A unprocessed list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name.</p>
    /// - On failure, responds with [`SdkError<BatchGetRecordError>`](crate::error::BatchGetRecordError)
    pub fn batch_get_record(&self) -> fluent_builders::BatchGetRecord {
        fluent_builders::BatchGetRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DeleteRecord`](crate::client::fluent_builders::DeleteRecord) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`feature_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::feature_group_name) / [`set_feature_group_name(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_feature_group_name): <p>The name of the feature group to delete the record from. </p>
    ///   - [`record_identifier_value_as_string(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::record_identifier_value_as_string) / [`set_record_identifier_value_as_string(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_record_identifier_value_as_string): <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in string format. </p>
    ///   - [`event_time(impl Into<String>)`](crate::client::fluent_builders::DeleteRecord::event_time) / [`set_event_time(Option<String>)`](crate::client::fluent_builders::DeleteRecord::set_event_time): <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be used to query data at a certain point in time.</p>
    /// - On success, responds with [`DeleteRecordOutput`](crate::output::DeleteRecordOutput)

    /// - On failure, responds with [`SdkError<DeleteRecordError>`](crate::error::DeleteRecordError)
    pub fn delete_record(&self) -> fluent_builders::DeleteRecord {
        fluent_builders::DeleteRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetRecord`](crate::client::fluent_builders::GetRecord) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`feature_group_name(impl Into<String>)`](crate::client::fluent_builders::GetRecord::feature_group_name) / [`set_feature_group_name(Option<String>)`](crate::client::fluent_builders::GetRecord::set_feature_group_name): <p>The name of the feature group in which you want to put the records.</p>
    ///   - [`record_identifier_value_as_string(impl Into<String>)`](crate::client::fluent_builders::GetRecord::record_identifier_value_as_string) / [`set_record_identifier_value_as_string(Option<String>)`](crate::client::fluent_builders::GetRecord::set_record_identifier_value_as_string): <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies the record in the <code>FeatureGroup</code>. </p>
    ///   - [`feature_names(Vec<String>)`](crate::client::fluent_builders::GetRecord::feature_names) / [`set_feature_names(Option<Vec<String>>)`](crate::client::fluent_builders::GetRecord::set_feature_names): <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
    /// - On success, responds with [`GetRecordOutput`](crate::output::GetRecordOutput) with field(s):
    ///   - [`record(Option<Vec<FeatureValue>>)`](crate::output::GetRecordOutput::record): <p>The record you requested. A list of <code>FeatureValues</code>.</p>
    /// - On failure, responds with [`SdkError<GetRecordError>`](crate::error::GetRecordError)
    pub fn get_record(&self) -> fluent_builders::GetRecord {
        fluent_builders::GetRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutRecord`](crate::client::fluent_builders::PutRecord) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`feature_group_name(impl Into<String>)`](crate::client::fluent_builders::PutRecord::feature_group_name) / [`set_feature_group_name(Option<String>)`](crate::client::fluent_builders::PutRecord::set_feature_group_name): <p>The name of the feature group that you want to insert the record into.</p>
    ///   - [`record(Vec<FeatureValue>)`](crate::client::fluent_builders::PutRecord::record) / [`set_record(Option<Vec<FeatureValue>>)`](crate::client::fluent_builders::PutRecord::set_record): <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>  <ul>   <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>   <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>   <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>  </ul>
    /// - On success, responds with [`PutRecordOutput`](crate::output::PutRecordOutput)

    /// - On failure, responds with [`SdkError<PutRecordError>`](crate::error::PutRecordError)
    pub fn put_record(&self) -> fluent_builders::PutRecord {
        fluent_builders::PutRecord::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `BatchGetRecord`.
    ///
    /// <p>Retrieves a batch of <code>Records</code> from a <code>FeatureGroup</code>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct BatchGetRecord {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::batch_get_record_input::Builder,
    }
    impl BatchGetRecord {
        /// Creates a new `BatchGetRecord`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::customizable_operation::CustomizableOperation<
                crate::operation::BatchGetRecord,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::BatchGetRecordError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::customizable_operation::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::BatchGetRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::BatchGetRecordError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Appends an item to `Identifiers`.
        ///
        /// To override the contents of this collection use [`set_identifiers`](Self::set_identifiers).
        ///
        /// <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name that have been requested to be retrieved in batch.</p>
        pub fn identifiers(mut self, input: crate::model::BatchGetRecordIdentifier) -> Self {
            self.inner = self.inner.identifiers(input);
            self
        }
        /// <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name that have been requested to be retrieved in batch.</p>
        pub fn set_identifiers(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
        ) -> Self {
            self.inner = self.inner.set_identifiers(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteRecord`.
    ///
    /// <p>Deletes a <code>Record</code> from a <code>FeatureGroup</code>. A new record will show up in the <code>OfflineStore</code> when the <code>DeleteRecord</code> API is called. This record will have a value of <code>True</code> in the <code>is_deleted</code> column.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteRecord {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::delete_record_input::Builder,
    }
    impl DeleteRecord {
        /// Creates a new `DeleteRecord`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::customizable_operation::CustomizableOperation<
                crate::operation::DeleteRecord,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::DeleteRecordError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::customizable_operation::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteRecordError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the feature group to delete the record from. </p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(input.into());
            self
        }
        /// <p>The name of the feature group to delete the record from. </p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in string format. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.record_identifier_value_as_string(input.into());
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in string format. </p>
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_record_identifier_value_as_string(input);
            self
        }
        /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be used to query data at a certain point in time.</p>
        pub fn event_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_time(input.into());
            self
        }
        /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be used to query data at a certain point in time.</p>
        pub fn set_event_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_event_time(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetRecord`.
    ///
    /// <p>Use for <code>OnlineStore</code> serving from a <code>FeatureStore</code>. Only the latest records stored in the <code>OnlineStore</code> can be retrieved. If no Record with <code>RecordIdentifierValue</code> is found, then an empty result is returned. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRecord {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_record_input::Builder,
    }
    impl GetRecord {
        /// Creates a new `GetRecord`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::customizable_operation::CustomizableOperation<
                crate::operation::GetRecord,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetRecordError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::customizable_operation::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRecordError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the feature group in which you want to put the records.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(input.into());
            self
        }
        /// <p>The name of the feature group in which you want to put the records.</p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies the record in the <code>FeatureGroup</code>. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.record_identifier_value_as_string(input.into());
            self
        }
        /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies the record in the <code>FeatureGroup</code>. </p>
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_record_identifier_value_as_string(input);
            self
        }
        /// Appends an item to `FeatureNames`.
        ///
        /// To override the contents of this collection use [`set_feature_names`](Self::set_feature_names).
        ///
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
        pub fn feature_names(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_names(input.into());
            self
        }
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
        pub fn set_feature_names(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_feature_names(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutRecord`.
    ///
    /// <p>Used for data ingestion into the <code>FeatureStore</code>. The <code>PutRecord</code> API writes to both the <code>OnlineStore</code> and <code>OfflineStore</code>. If the record is the latest record for the <code>recordIdentifier</code>, the record is written to both the <code>OnlineStore</code> and <code>OfflineStore</code>. If the record is a historic record, it is written only to the <code>OfflineStore</code>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutRecord {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::put_record_input::Builder,
    }
    impl PutRecord {
        /// Creates a new `PutRecord`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::customizable_operation::CustomizableOperation<
                crate::operation::PutRecord,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::PutRecordError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::customizable_operation::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::PutRecordError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the feature group that you want to insert the record into.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(input.into());
            self
        }
        /// <p>The name of the feature group that you want to insert the record into.</p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// Appends an item to `Record`.
        ///
        /// To override the contents of this collection use [`set_record`](Self::set_record).
        ///
        /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
        /// <ul>
        /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
        /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
        /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
        /// </ul>
        pub fn record(mut self, input: crate::model::FeatureValue) -> Self {
            self.inner = self.inner.record(input);
            self
        }
        /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
        /// <ul>
        /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
        /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
        /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
        /// </ul>
        pub fn set_record(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
        ) -> Self {
            self.inner = self.inner.set_record(input);
            self
        }
    }
}

impl Client {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
    where
        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
        E: Into<aws_smithy_http::result::ConnectorError>,
    {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf.timeout_config().cloned().unwrap_or_default();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(aws_smithy_client::erase::DynConnector::new(conn))
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ));
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = conf.sleep_impl() {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf.timeout_config().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
        }
        let mut builder = aws_smithy_client::Builder::dyn_https().middleware(
            aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ),
        );
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
