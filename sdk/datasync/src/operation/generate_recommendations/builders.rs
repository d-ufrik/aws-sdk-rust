// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::generate_recommendations::_generate_recommendations_output::GenerateRecommendationsOutputBuilder;

pub use crate::operation::generate_recommendations::_generate_recommendations_input::GenerateRecommendationsInputBuilder;

/// Fluent builder constructing a request to `GenerateRecommendations`.
///
/// <p>Creates recommendations about where to migrate your data to in Amazon Web Services. Recommendations are generated based on information that DataSync Discovery collects about your on-premises storage system's resources. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-understand-recommendations.html">Recommendations provided by DataSync Discovery</a>.</p>
/// <p>Once generated, you can view your recommendations by using the <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_DescribeStorageSystemResources.html">DescribeStorageSystemResources</a> operation.</p> <note>
/// <p>If your <a href="https://docs.aws.amazon.com/datasync/latest/userguide/discovery-job-statuses.html#discovery-job-statuses-table">discovery job completes successfully</a>, you don't need to use this operation. DataSync Discovery generates the recommendations for you automatically.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::generate_recommendations::builders::GenerateRecommendationsInputBuilder,
}
impl GenerateRecommendationsFluentBuilder {
    /// Creates a new `GenerateRecommendations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::generate_recommendations::GenerateRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::generate_recommendations::GenerateRecommendationsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_recommendations::GenerateRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::generate_recommendations::GenerateRecommendationsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
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
    ) -> ::std::result::Result<
        crate::operation::generate_recommendations::GenerateRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::generate_recommendations::GenerateRecommendationsError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::generate_recommendations::GenerateRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::generate_recommendations::GenerateRecommendationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the discovery job that collects information about your on-premises storage system.</p>
    pub fn discovery_job_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.discovery_job_arn(input.into());
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the discovery job that collects information about your on-premises storage system.</p>
    pub fn set_discovery_job_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_discovery_job_arn(input);
        self
    }
    /// Appends an item to `ResourceIds`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>Specifies the universally unique identifiers (UUIDs) of the resources in your storage system that you want recommendations on.</p>
    pub fn resource_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_ids(input.into());
        self
    }
    /// <p>Specifies the universally unique identifiers (UUIDs) of the resources in your storage system that you want recommendations on.</p>
    pub fn set_resource_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_resource_ids(input);
        self
    }
    /// <p>Specifies the type of resource in your storage system that you want recommendations on.</p>
    pub fn resource_type(mut self, input: crate::types::DiscoveryResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>Specifies the type of resource in your storage system that you want recommendations on.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::DiscoveryResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
}
