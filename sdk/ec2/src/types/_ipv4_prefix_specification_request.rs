// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the IPv4 prefix option for a network interface.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Ipv4PrefixSpecificationRequest {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[doc(hidden)]
    pub ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecificationRequest {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn ipv4_prefix(&self) -> ::std::option::Option<&str> {
        self.ipv4_prefix.as_deref()
    }
}
impl Ipv4PrefixSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`Ipv4PrefixSpecificationRequest`](crate::types::Ipv4PrefixSpecificationRequest).
    pub fn builder() -> crate::types::builders::Ipv4PrefixSpecificationRequestBuilder {
        crate::types::builders::Ipv4PrefixSpecificationRequestBuilder::default()
    }
}

/// A builder for [`Ipv4PrefixSpecificationRequest`](crate::types::Ipv4PrefixSpecificationRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct Ipv4PrefixSpecificationRequestBuilder {
    pub(crate) ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecificationRequestBuilder {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn ipv4_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv4_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_ipv4_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv4_prefix = input;
        self
    }
    /// Consumes the builder and constructs a [`Ipv4PrefixSpecificationRequest`](crate::types::Ipv4PrefixSpecificationRequest).
    pub fn build(self) -> crate::types::Ipv4PrefixSpecificationRequest {
        crate::types::Ipv4PrefixSpecificationRequest {
            ipv4_prefix: self.ipv4_prefix,
        }
    }
}
