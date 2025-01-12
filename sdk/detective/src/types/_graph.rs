// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A behavior graph in Detective.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Graph {
    /// <p>The ARN of the behavior graph.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The date and time that the behavior graph was created. The value is an ISO8601 formatted string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Graph {
    /// <p>The ARN of the behavior graph.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The date and time that the behavior graph was created. The value is an ISO8601 formatted string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
}
impl Graph {
    /// Creates a new builder-style object to manufacture [`Graph`](crate::types::Graph).
    pub fn builder() -> crate::types::builders::GraphBuilder {
        crate::types::builders::GraphBuilder::default()
    }
}

/// A builder for [`Graph`](crate::types::Graph).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GraphBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl GraphBuilder {
    /// <p>The ARN of the behavior graph.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The date and time that the behavior graph was created. The value is an ISO8601 formatted string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the behavior graph was created. The value is an ISO8601 formatted string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// Consumes the builder and constructs a [`Graph`](crate::types::Graph).
    pub fn build(self) -> crate::types::Graph {
        crate::types::Graph {
            arn: self.arn,
            created_time: self.created_time,
        }
    }
}
