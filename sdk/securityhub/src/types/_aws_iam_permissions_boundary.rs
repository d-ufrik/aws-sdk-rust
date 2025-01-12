// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsIamPermissionsBoundary {
    /// <p>The ARN of the policy used to set the permissions boundary.</p>
    #[doc(hidden)]
    pub permissions_boundary_arn: ::std::option::Option<::std::string::String>,
    /// <p>The usage type for the permissions boundary.</p>
    #[doc(hidden)]
    pub permissions_boundary_type: ::std::option::Option<::std::string::String>,
}
impl AwsIamPermissionsBoundary {
    /// <p>The ARN of the policy used to set the permissions boundary.</p>
    pub fn permissions_boundary_arn(&self) -> ::std::option::Option<&str> {
        self.permissions_boundary_arn.as_deref()
    }
    /// <p>The usage type for the permissions boundary.</p>
    pub fn permissions_boundary_type(&self) -> ::std::option::Option<&str> {
        self.permissions_boundary_type.as_deref()
    }
}
impl AwsIamPermissionsBoundary {
    /// Creates a new builder-style object to manufacture [`AwsIamPermissionsBoundary`](crate::types::AwsIamPermissionsBoundary).
    pub fn builder() -> crate::types::builders::AwsIamPermissionsBoundaryBuilder {
        crate::types::builders::AwsIamPermissionsBoundaryBuilder::default()
    }
}

/// A builder for [`AwsIamPermissionsBoundary`](crate::types::AwsIamPermissionsBoundary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsIamPermissionsBoundaryBuilder {
    pub(crate) permissions_boundary_arn: ::std::option::Option<::std::string::String>,
    pub(crate) permissions_boundary_type: ::std::option::Option<::std::string::String>,
}
impl AwsIamPermissionsBoundaryBuilder {
    /// <p>The ARN of the policy used to set the permissions boundary.</p>
    pub fn permissions_boundary_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.permissions_boundary_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the policy used to set the permissions boundary.</p>
    pub fn set_permissions_boundary_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.permissions_boundary_arn = input;
        self
    }
    /// <p>The usage type for the permissions boundary.</p>
    pub fn permissions_boundary_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.permissions_boundary_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The usage type for the permissions boundary.</p>
    pub fn set_permissions_boundary_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.permissions_boundary_type = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsIamPermissionsBoundary`](crate::types::AwsIamPermissionsBoundary).
    pub fn build(self) -> crate::types::AwsIamPermissionsBoundary {
        crate::types::AwsIamPermissionsBoundary {
            permissions_boundary_arn: self.permissions_boundary_arn,
            permissions_boundary_type: self.permissions_boundary_type,
        }
    }
}
