// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Whitelist rule
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputWhitelistRule {
    /// The IPv4 CIDR that's whitelisted.
    #[doc(hidden)]
    pub cidr: ::std::option::Option<::std::string::String>,
}
impl InputWhitelistRule {
    /// The IPv4 CIDR that's whitelisted.
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
}
impl InputWhitelistRule {
    /// Creates a new builder-style object to manufacture [`InputWhitelistRule`](crate::types::InputWhitelistRule).
    pub fn builder() -> crate::types::builders::InputWhitelistRuleBuilder {
        crate::types::builders::InputWhitelistRuleBuilder::default()
    }
}

/// A builder for [`InputWhitelistRule`](crate::types::InputWhitelistRule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InputWhitelistRuleBuilder {
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
}
impl InputWhitelistRuleBuilder {
    /// The IPv4 CIDR that's whitelisted.
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// The IPv4 CIDR that's whitelisted.
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// Consumes the builder and constructs a [`InputWhitelistRule`](crate::types::InputWhitelistRule).
    pub fn build(self) -> crate::types::InputWhitelistRule {
        crate::types::InputWhitelistRule { cidr: self.cidr }
    }
}
