// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An environment variable to pass to the container.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
    /// <p>The name of the environment variable.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The value of the environment variable.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
    /// <p>The name of the environment variable.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The value of the environment variable.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
    /// Creates a new builder-style object to manufacture [`AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails).
    pub fn builder(
    ) -> crate::types::builders::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetailsBuilder
    {
        crate::types::builders::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetailsBuilder::default()
    }
}

/// A builder for [`AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetailsBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetailsBuilder {
    /// <p>The name of the environment variable.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the environment variable.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The value of the environment variable.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the environment variable.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails).
    pub fn build(self) -> crate::types::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
        crate::types::AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
            name: self.name,
            value: self.value,
        }
    }
}
