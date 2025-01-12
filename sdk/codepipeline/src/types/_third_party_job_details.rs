// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a job sent in response to a <code>GetThirdPartyJobDetails</code> request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThirdPartyJobDetails {
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The data to be returned by the third party job worker.</p>
    #[doc(hidden)]
    pub data: ::std::option::Option<crate::types::ThirdPartyJobData>,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <code>AcknowledgeThirdPartyJob</code> request.</p>
    #[doc(hidden)]
    pub nonce: ::std::option::Option<::std::string::String>,
}
impl ThirdPartyJobDetails {
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The data to be returned by the third party job worker.</p>
    pub fn data(&self) -> ::std::option::Option<&crate::types::ThirdPartyJobData> {
        self.data.as_ref()
    }
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <code>AcknowledgeThirdPartyJob</code> request.</p>
    pub fn nonce(&self) -> ::std::option::Option<&str> {
        self.nonce.as_deref()
    }
}
impl ThirdPartyJobDetails {
    /// Creates a new builder-style object to manufacture [`ThirdPartyJobDetails`](crate::types::ThirdPartyJobDetails).
    pub fn builder() -> crate::types::builders::ThirdPartyJobDetailsBuilder {
        crate::types::builders::ThirdPartyJobDetailsBuilder::default()
    }
}

/// A builder for [`ThirdPartyJobDetails`](crate::types::ThirdPartyJobDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ThirdPartyJobDetailsBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) data: ::std::option::Option<crate::types::ThirdPartyJobData>,
    pub(crate) nonce: ::std::option::Option<::std::string::String>,
}
impl ThirdPartyJobDetailsBuilder {
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The data to be returned by the third party job worker.</p>
    pub fn data(mut self, input: crate::types::ThirdPartyJobData) -> Self {
        self.data = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data to be returned by the third party job worker.</p>
    pub fn set_data(
        mut self,
        input: ::std::option::Option<crate::types::ThirdPartyJobData>,
    ) -> Self {
        self.data = input;
        self
    }
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <code>AcknowledgeThirdPartyJob</code> request.</p>
    pub fn nonce(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.nonce = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <code>AcknowledgeThirdPartyJob</code> request.</p>
    pub fn set_nonce(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.nonce = input;
        self
    }
    /// Consumes the builder and constructs a [`ThirdPartyJobDetails`](crate::types::ThirdPartyJobDetails).
    pub fn build(self) -> crate::types::ThirdPartyJobDetails {
        crate::types::ThirdPartyJobDetails {
            id: self.id,
            data: self.data,
            nonce: self.nonce,
        }
    }
}
