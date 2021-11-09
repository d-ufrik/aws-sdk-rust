// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutObjectOutput {
    /// <p>The SHA256 digest of the object that is persisted.</p>
    pub content_sha256: std::option::Option<std::string::String>,
    /// <p>Unique identifier of the object in the container.</p>
    pub e_tag: std::option::Option<std::string::String>,
    /// <p>The storage class where the object was persisted. The class should be
    /// “Temporal”.</p>
    pub storage_class: std::option::Option<crate::model::StorageClass>,
}
impl PutObjectOutput {
    /// <p>The SHA256 digest of the object that is persisted.</p>
    pub fn content_sha256(&self) -> std::option::Option<&str> {
        self.content_sha256.as_deref()
    }
    /// <p>Unique identifier of the object in the container.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The storage class where the object was persisted. The class should be
    /// “Temporal”.</p>
    pub fn storage_class(&self) -> std::option::Option<&crate::model::StorageClass> {
        self.storage_class.as_ref()
    }
}
impl std::fmt::Debug for PutObjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutObjectOutput");
        formatter.field("content_sha256", &self.content_sha256);
        formatter.field("e_tag", &self.e_tag);
        formatter.field("storage_class", &self.storage_class);
        formatter.finish()
    }
}
/// See [`PutObjectOutput`](crate::output::PutObjectOutput)
pub mod put_object_output {
    /// A builder for [`PutObjectOutput`](crate::output::PutObjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_sha256: std::option::Option<std::string::String>,
        pub(crate) e_tag: std::option::Option<std::string::String>,
        pub(crate) storage_class: std::option::Option<crate::model::StorageClass>,
    }
    impl Builder {
        /// <p>The SHA256 digest of the object that is persisted.</p>
        pub fn content_sha256(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_sha256 = Some(input.into());
            self
        }
        /// <p>The SHA256 digest of the object that is persisted.</p>
        pub fn set_content_sha256(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.content_sha256 = input;
            self
        }
        /// <p>Unique identifier of the object in the container.</p>
        pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.e_tag = Some(input.into());
            self
        }
        /// <p>Unique identifier of the object in the container.</p>
        pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.e_tag = input;
            self
        }
        /// <p>The storage class where the object was persisted. The class should be
        /// “Temporal”.</p>
        pub fn storage_class(mut self, input: crate::model::StorageClass) -> Self {
            self.storage_class = Some(input);
            self
        }
        /// <p>The storage class where the object was persisted. The class should be
        /// “Temporal”.</p>
        pub fn set_storage_class(
            mut self,
            input: std::option::Option<crate::model::StorageClass>,
        ) -> Self {
            self.storage_class = input;
            self
        }
        /// Consumes the builder and constructs a [`PutObjectOutput`](crate::output::PutObjectOutput)
        pub fn build(self) -> crate::output::PutObjectOutput {
            crate::output::PutObjectOutput {
                content_sha256: self.content_sha256,
                e_tag: self.e_tag,
                storage_class: self.storage_class,
            }
        }
    }
}
impl PutObjectOutput {
    /// Creates a new builder-style object to manufacture [`PutObjectOutput`](crate::output::PutObjectOutput)
    pub fn builder() -> crate::output::put_object_output::Builder {
        crate::output::put_object_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListItemsOutput {
    /// <p>The metadata entries for the folders and objects at the requested path.</p>
    pub items: std::option::Option<std::vec::Vec<crate::model::Item>>,
    /// <p>The token that can be used in a request to view the next set of results. For example,
    /// you submit a <code>ListItems</code> request that matches 2,000 items with
    /// <code>MaxResults</code> set at 500. The service returns the first batch of results (up
    /// to 500) and a <code>NextToken</code> value that can be used to fetch the next batch of
    /// results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListItemsOutput {
    /// <p>The metadata entries for the folders and objects at the requested path.</p>
    pub fn items(&self) -> std::option::Option<&[crate::model::Item]> {
        self.items.as_deref()
    }
    /// <p>The token that can be used in a request to view the next set of results. For example,
    /// you submit a <code>ListItems</code> request that matches 2,000 items with
    /// <code>MaxResults</code> set at 500. The service returns the first batch of results (up
    /// to 500) and a <code>NextToken</code> value that can be used to fetch the next batch of
    /// results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListItemsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListItemsOutput");
        formatter.field("items", &self.items);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListItemsOutput`](crate::output::ListItemsOutput)
pub mod list_items_output {
    /// A builder for [`ListItemsOutput`](crate::output::ListItemsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) items: std::option::Option<std::vec::Vec<crate::model::Item>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `items`.
        ///
        /// To override the contents of this collection use [`set_items`](Self::set_items).
        ///
        /// <p>The metadata entries for the folders and objects at the requested path.</p>
        pub fn items(mut self, input: impl Into<crate::model::Item>) -> Self {
            let mut v = self.items.unwrap_or_default();
            v.push(input.into());
            self.items = Some(v);
            self
        }
        /// <p>The metadata entries for the folders and objects at the requested path.</p>
        pub fn set_items(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Item>>,
        ) -> Self {
            self.items = input;
            self
        }
        /// <p>The token that can be used in a request to view the next set of results. For example,
        /// you submit a <code>ListItems</code> request that matches 2,000 items with
        /// <code>MaxResults</code> set at 500. The service returns the first batch of results (up
        /// to 500) and a <code>NextToken</code> value that can be used to fetch the next batch of
        /// results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token that can be used in a request to view the next set of results. For example,
        /// you submit a <code>ListItems</code> request that matches 2,000 items with
        /// <code>MaxResults</code> set at 500. The service returns the first batch of results (up
        /// to 500) and a <code>NextToken</code> value that can be used to fetch the next batch of
        /// results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListItemsOutput`](crate::output::ListItemsOutput)
        pub fn build(self) -> crate::output::ListItemsOutput {
            crate::output::ListItemsOutput {
                items: self.items,
                next_token: self.next_token,
            }
        }
    }
}
impl ListItemsOutput {
    /// Creates a new builder-style object to manufacture [`ListItemsOutput`](crate::output::ListItemsOutput)
    pub fn builder() -> crate::output::list_items_output::Builder {
        crate::output::list_items_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct GetObjectOutput {
    /// <p>The bytes of the object. </p>
    pub body: aws_smithy_http::byte_stream::ByteStream,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
    /// object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    /// <p>Headers with a custom user-defined value are also accepted.</p>
    pub cache_control: std::option::Option<std::string::String>,
    /// <p>The range of bytes to retrieve.</p>
    pub content_range: std::option::Option<std::string::String>,
    /// <p>The length of the object in bytes.</p>
    pub content_length: std::option::Option<i64>,
    /// <p>The content type of the object.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub e_tag: std::option::Option<std::string::String>,
    /// <p>The date and time that the object was last modified.</p>
    pub last_modified: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate
    /// success. All other status codes indicate the type of error that occurred.</p>
    pub status_code: i32,
}
impl GetObjectOutput {
    /// <p>The bytes of the object. </p>
    pub fn body(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.body
    }
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
    /// object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    /// <p>Headers with a custom user-defined value are also accepted.</p>
    pub fn cache_control(&self) -> std::option::Option<&str> {
        self.cache_control.as_deref()
    }
    /// <p>The range of bytes to retrieve.</p>
    pub fn content_range(&self) -> std::option::Option<&str> {
        self.content_range.as_deref()
    }
    /// <p>The length of the object in bytes.</p>
    pub fn content_length(&self) -> std::option::Option<i64> {
        self.content_length
    }
    /// <p>The content type of the object.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The date and time that the object was last modified.</p>
    pub fn last_modified(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_modified.as_ref()
    }
    /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate
    /// success. All other status codes indicate the type of error that occurred.</p>
    pub fn status_code(&self) -> i32 {
        self.status_code
    }
}
impl std::fmt::Debug for GetObjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetObjectOutput");
        formatter.field("body", &self.body);
        formatter.field("cache_control", &self.cache_control);
        formatter.field("content_range", &self.content_range);
        formatter.field("content_length", &self.content_length);
        formatter.field("content_type", &self.content_type);
        formatter.field("e_tag", &self.e_tag);
        formatter.field("last_modified", &self.last_modified);
        formatter.field("status_code", &self.status_code);
        formatter.finish()
    }
}
/// See [`GetObjectOutput`](crate::output::GetObjectOutput)
pub mod get_object_output {
    /// A builder for [`GetObjectOutput`](crate::output::GetObjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) body: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        pub(crate) cache_control: std::option::Option<std::string::String>,
        pub(crate) content_range: std::option::Option<std::string::String>,
        pub(crate) content_length: std::option::Option<i64>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) e_tag: std::option::Option<std::string::String>,
        pub(crate) last_modified: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) status_code: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The bytes of the object. </p>
        pub fn body(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.body = Some(input);
            self
        }
        /// <p>The bytes of the object. </p>
        pub fn set_body(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.body = input;
            self
        }
        /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
        /// object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
        /// <p>Headers with a custom user-defined value are also accepted.</p>
        pub fn cache_control(mut self, input: impl Into<std::string::String>) -> Self {
            self.cache_control = Some(input.into());
            self
        }
        /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
        /// object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
        /// <p>Headers with a custom user-defined value are also accepted.</p>
        pub fn set_cache_control(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.cache_control = input;
            self
        }
        /// <p>The range of bytes to retrieve.</p>
        pub fn content_range(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_range = Some(input.into());
            self
        }
        /// <p>The range of bytes to retrieve.</p>
        pub fn set_content_range(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.content_range = input;
            self
        }
        /// <p>The length of the object in bytes.</p>
        pub fn content_length(mut self, input: i64) -> Self {
            self.content_length = Some(input);
            self
        }
        /// <p>The length of the object in bytes.</p>
        pub fn set_content_length(mut self, input: std::option::Option<i64>) -> Self {
            self.content_length = input;
            self
        }
        /// <p>The content type of the object.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The content type of the object.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>The ETag that represents a unique instance of the object.</p>
        pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.e_tag = Some(input.into());
            self
        }
        /// <p>The ETag that represents a unique instance of the object.</p>
        pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.e_tag = input;
            self
        }
        /// <p>The date and time that the object was last modified.</p>
        pub fn last_modified(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_modified = Some(input);
            self
        }
        /// <p>The date and time that the object was last modified.</p>
        pub fn set_last_modified(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_modified = input;
            self
        }
        /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate
        /// success. All other status codes indicate the type of error that occurred.</p>
        pub fn status_code(mut self, input: i32) -> Self {
            self.status_code = Some(input);
            self
        }
        /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate
        /// success. All other status codes indicate the type of error that occurred.</p>
        pub fn set_status_code(mut self, input: std::option::Option<i32>) -> Self {
            self.status_code = input;
            self
        }
        /// Consumes the builder and constructs a [`GetObjectOutput`](crate::output::GetObjectOutput)
        pub fn build(self) -> crate::output::GetObjectOutput {
            crate::output::GetObjectOutput {
                body: self.body.unwrap_or_default(),
                cache_control: self.cache_control,
                content_range: self.content_range,
                content_length: self.content_length,
                content_type: self.content_type,
                e_tag: self.e_tag,
                last_modified: self.last_modified,
                status_code: self.status_code.unwrap_or_default(),
            }
        }
    }
}
impl GetObjectOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectOutput`](crate::output::GetObjectOutput)
    pub fn builder() -> crate::output::get_object_output::Builder {
        crate::output::get_object_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeObjectOutput {
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub e_tag: std::option::Option<std::string::String>,
    /// <p>The content type of the object.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The length of the object in bytes.</p>
    pub content_length: std::option::Option<i64>,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
    /// object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    /// <p>Headers with a custom user-defined value are also accepted.</p>
    pub cache_control: std::option::Option<std::string::String>,
    /// <p>The date and time that the object was last modified.</p>
    pub last_modified: std::option::Option<aws_smithy_types::Instant>,
}
impl DescribeObjectOutput {
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The content type of the object.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>The length of the object in bytes.</p>
    pub fn content_length(&self) -> std::option::Option<i64> {
        self.content_length
    }
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
    /// object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    /// <p>Headers with a custom user-defined value are also accepted.</p>
    pub fn cache_control(&self) -> std::option::Option<&str> {
        self.cache_control.as_deref()
    }
    /// <p>The date and time that the object was last modified.</p>
    pub fn last_modified(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_modified.as_ref()
    }
}
impl std::fmt::Debug for DescribeObjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeObjectOutput");
        formatter.field("e_tag", &self.e_tag);
        formatter.field("content_type", &self.content_type);
        formatter.field("content_length", &self.content_length);
        formatter.field("cache_control", &self.cache_control);
        formatter.field("last_modified", &self.last_modified);
        formatter.finish()
    }
}
/// See [`DescribeObjectOutput`](crate::output::DescribeObjectOutput)
pub mod describe_object_output {
    /// A builder for [`DescribeObjectOutput`](crate::output::DescribeObjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) e_tag: std::option::Option<std::string::String>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) content_length: std::option::Option<i64>,
        pub(crate) cache_control: std::option::Option<std::string::String>,
        pub(crate) last_modified: std::option::Option<aws_smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The ETag that represents a unique instance of the object.</p>
        pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.e_tag = Some(input.into());
            self
        }
        /// <p>The ETag that represents a unique instance of the object.</p>
        pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.e_tag = input;
            self
        }
        /// <p>The content type of the object.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The content type of the object.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>The length of the object in bytes.</p>
        pub fn content_length(mut self, input: i64) -> Self {
            self.content_length = Some(input);
            self
        }
        /// <p>The length of the object in bytes.</p>
        pub fn set_content_length(mut self, input: std::option::Option<i64>) -> Self {
            self.content_length = input;
            self
        }
        /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
        /// object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
        /// <p>Headers with a custom user-defined value are also accepted.</p>
        pub fn cache_control(mut self, input: impl Into<std::string::String>) -> Self {
            self.cache_control = Some(input.into());
            self
        }
        /// <p>An optional <code>CacheControl</code> header that allows the caller to control the
        /// object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
        /// <p>Headers with a custom user-defined value are also accepted.</p>
        pub fn set_cache_control(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.cache_control = input;
            self
        }
        /// <p>The date and time that the object was last modified.</p>
        pub fn last_modified(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_modified = Some(input);
            self
        }
        /// <p>The date and time that the object was last modified.</p>
        pub fn set_last_modified(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_modified = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeObjectOutput`](crate::output::DescribeObjectOutput)
        pub fn build(self) -> crate::output::DescribeObjectOutput {
            crate::output::DescribeObjectOutput {
                e_tag: self.e_tag,
                content_type: self.content_type,
                content_length: self.content_length,
                cache_control: self.cache_control,
                last_modified: self.last_modified,
            }
        }
    }
}
impl DescribeObjectOutput {
    /// Creates a new builder-style object to manufacture [`DescribeObjectOutput`](crate::output::DescribeObjectOutput)
    pub fn builder() -> crate::output::describe_object_output::Builder {
        crate::output::describe_object_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteObjectOutput {}
impl std::fmt::Debug for DeleteObjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteObjectOutput");
        formatter.finish()
    }
}
/// See [`DeleteObjectOutput`](crate::output::DeleteObjectOutput)
pub mod delete_object_output {
    /// A builder for [`DeleteObjectOutput`](crate::output::DeleteObjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteObjectOutput`](crate::output::DeleteObjectOutput)
        pub fn build(self) -> crate::output::DeleteObjectOutput {
            crate::output::DeleteObjectOutput {}
        }
    }
}
impl DeleteObjectOutput {
    /// Creates a new builder-style object to manufacture [`DeleteObjectOutput`](crate::output::DeleteObjectOutput)
    pub fn builder() -> crate::output::delete_object_output::Builder {
        crate::output::delete_object_output::Builder::default()
    }
}
