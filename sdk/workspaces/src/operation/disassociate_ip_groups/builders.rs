// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_ip_groups::_disassociate_ip_groups_output::DisassociateIpGroupsOutputBuilder;

pub use crate::operation::disassociate_ip_groups::_disassociate_ip_groups_input::DisassociateIpGroupsInputBuilder;

/// Fluent builder constructing a request to `DisassociateIpGroups`.
///
/// <p>Disassociates the specified IP access control group from the specified directory.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateIpGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_ip_groups::builders::DisassociateIpGroupsInputBuilder,
}
impl DisassociateIpGroupsFluentBuilder {
    /// Creates a new `DisassociateIpGroups`.
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
            crate::operation::disassociate_ip_groups::DisassociateIpGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_ip_groups::DisassociateIpGroupsError,
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
        crate::operation::disassociate_ip_groups::DisassociateIpGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_ip_groups::DisassociateIpGroupsError,
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
        crate::operation::disassociate_ip_groups::DisassociateIpGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_ip_groups::DisassociateIpGroupsError,
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
            crate::operation::disassociate_ip_groups::DisassociateIpGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_ip_groups::DisassociateIpGroupsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// Appends an item to `GroupIds`.
    ///
    /// To override the contents of this collection use [`set_group_ids`](Self::set_group_ids).
    ///
    /// <p>The identifiers of one or more IP access control groups.</p>
    pub fn group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_ids(input.into());
        self
    }
    /// <p>The identifiers of one or more IP access control groups.</p>
    pub fn set_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_group_ids(input);
        self
    }
}
