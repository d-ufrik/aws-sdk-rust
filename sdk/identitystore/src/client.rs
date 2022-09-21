// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWS SSO Identity Store
///
/// Client for invoking operations on AWS SSO Identity Store. Each operation on AWS SSO Identity Store is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_identitystore::Client::new(&shared_config);
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
/// let config = aws_sdk_identitystore::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_identitystore::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`DescribeGroup`](crate::client::fluent_builders::DescribeGroup) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::client::fluent_builders::DescribeGroup::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::client::fluent_builders::DescribeGroup::set_identity_store_id): <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
    ///   - [`group_id(impl Into<String>)`](crate::client::fluent_builders::DescribeGroup::group_id) / [`set_group_id(Option<String>)`](crate::client::fluent_builders::DescribeGroup::set_group_id): <p>The identifier for a group in the identity store.</p>
    /// - On success, responds with [`DescribeGroupOutput`](crate::output::DescribeGroupOutput) with field(s):
    ///   - [`group_id(Option<String>)`](crate::output::DescribeGroupOutput::group_id): <p>The identifier for a group in the identity store.</p>
    ///   - [`display_name(Option<String>)`](crate::output::DescribeGroupOutput::display_name): <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time that the group is created and stored as an attribute of the group object in the identity store.</p>
    /// - On failure, responds with [`SdkError<DescribeGroupError>`](crate::error::DescribeGroupError)
    pub fn describe_group(&self) -> fluent_builders::DescribeGroup {
        fluent_builders::DescribeGroup::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeUser`](crate::client::fluent_builders::DescribeUser) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUser::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::client::fluent_builders::DescribeUser::set_identity_store_id): <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUser::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::DescribeUser::set_user_id): <p>The identifier for a user in the identity store.</p>
    /// - On success, responds with [`DescribeUserOutput`](crate::output::DescribeUserOutput) with field(s):
    ///   - [`user_name(Option<String>)`](crate::output::DescribeUserOutput::user_name): <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    ///   - [`user_id(Option<String>)`](crate::output::DescribeUserOutput::user_id): <p>The identifier for a user in the identity store.</p>
    /// - On failure, responds with [`SdkError<DescribeUserError>`](crate::error::DescribeUserError)
    pub fn describe_user(&self) -> fluent_builders::DescribeUser {
        fluent_builders::DescribeUser::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListGroups`](crate::client::fluent_builders::ListGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListGroups::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::client::fluent_builders::ListGroups::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::client::fluent_builders::ListGroups::set_identity_store_id): <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListGroups::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListGroups::set_max_results): <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListGroups::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListGroups::set_next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListGroups::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListGroups::set_filters): <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
    /// - On success, responds with [`ListGroupsOutput`](crate::output::ListGroupsOutput) with field(s):
    ///   - [`groups(Option<Vec<Group>>)`](crate::output::ListGroupsOutput::groups): <p>A list of <code>Group</code> objects in the identity store.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListGroupsOutput::next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it1 is used in the API request to search for the next page.</p>
    /// - On failure, responds with [`SdkError<ListGroupsError>`](crate::error::ListGroupsError)
    pub fn list_groups(&self) -> fluent_builders::ListGroups {
        fluent_builders::ListGroups::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListUsers`](crate::client::fluent_builders::ListUsers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUsers::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::client::fluent_builders::ListUsers::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::client::fluent_builders::ListUsers::set_identity_store_id): <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListUsers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListUsers::set_max_results): <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListUsers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListUsers::set_next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListUsers::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListUsers::set_filters): <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
    /// - On success, responds with [`ListUsersOutput`](crate::output::ListUsersOutput) with field(s):
    ///   - [`users(Option<Vec<User>>)`](crate::output::ListUsersOutput::users): <p>A list of <code>User</code> objects in the identity store.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListUsersOutput::next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    /// - On failure, responds with [`SdkError<ListUsersError>`](crate::error::ListUsersError)
    pub fn list_users(&self) -> fluent_builders::ListUsers {
        fluent_builders::ListUsers::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DescribeGroup`.
    ///
    /// <p>Retrieves the group metadata and attributes from <code>GroupId</code> in an identity store.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeGroup {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_group_input::Builder,
    }
    impl DescribeGroup {
        /// Creates a new `DescribeGroup`.
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
                crate::operation::DescribeGroup,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::DescribeGroupError>,
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
            crate::output::DescribeGroupOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeGroupError>,
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
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.group_id(input.into());
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_group_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeUser`.
    ///
    /// <p>Retrieves the user metadata and attributes from <code>UserId</code> in an identity store.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeUser {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_user_input::Builder,
    }
    impl DescribeUser {
        /// Creates a new `DescribeUser`.
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
                crate::operation::DescribeUser,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::DescribeUserError>,
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
            crate::output::DescribeUserOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeUserError>,
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
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListGroups`.
    ///
    /// <p>Lists the attribute name and value of the group that you specified in the search. We only support <code>DisplayName</code> as a valid filter attribute path currently, and filter is required. This API returns minimum attributes, including <code>GroupId</code> and group <code>DisplayName</code> in the response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListGroups {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_groups_input::Builder,
    }
    impl ListGroups {
        /// Creates a new `ListGroups`.
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
                crate::operation::ListGroups,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListGroupsError>,
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
            crate::output::ListGroupsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListGroupsError>,
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
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListGroupsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListGroupsPaginator {
            crate::paginator::ListGroupsPaginator::new(self.handle, self.inner)
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListUsers`.
    ///
    /// <p>Lists the attribute name and value of the user that you specified in the search. We only support <code>UserName</code> as a valid filter attribute path currently, and filter is required. This API returns minimum attributes, including <code>UserId</code> and <code>UserName</code> in the response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListUsers {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_users_input::Builder,
    }
    impl ListUsers {
        /// Creates a new `ListUsers`.
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
                crate::operation::ListUsers,
                aws_http::retry::AwsErrorRetryPolicy,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListUsersError>,
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
            crate::output::ListUsersOutput,
            aws_smithy_http::result::SdkError<crate::error::ListUsersError>,
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
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListUsersPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListUsersPaginator {
            crate::paginator::ListUsersPaginator::new(self.handle, self.inner)
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
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
