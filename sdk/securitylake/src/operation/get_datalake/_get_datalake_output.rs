// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDatalakeOutput {
    /// <p>Retrieves the Security Lake configuration object. </p>
    #[doc(hidden)]
    pub configurations: ::std::option::Option<
        ::std::collections::HashMap<crate::types::Region, crate::types::LakeConfigurationResponse>,
    >,
    _request_id: Option<String>,
}
impl GetDatalakeOutput {
    /// <p>Retrieves the Security Lake configuration object. </p>
    pub fn configurations(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<crate::types::Region, crate::types::LakeConfigurationResponse>,
    > {
        self.configurations.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetDatalakeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDatalakeOutput {
    /// Creates a new builder-style object to manufacture [`GetDatalakeOutput`](crate::operation::get_datalake::GetDatalakeOutput).
    pub fn builder() -> crate::operation::get_datalake::builders::GetDatalakeOutputBuilder {
        crate::operation::get_datalake::builders::GetDatalakeOutputBuilder::default()
    }
}

/// A builder for [`GetDatalakeOutput`](crate::operation::get_datalake::GetDatalakeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDatalakeOutputBuilder {
    pub(crate) configurations: ::std::option::Option<
        ::std::collections::HashMap<crate::types::Region, crate::types::LakeConfigurationResponse>,
    >,
    _request_id: Option<String>,
}
impl GetDatalakeOutputBuilder {
    /// Adds a key-value pair to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>Retrieves the Security Lake configuration object. </p>
    pub fn configurations(
        mut self,
        k: crate::types::Region,
        v: crate::types::LakeConfigurationResponse,
    ) -> Self {
        let mut hash_map = self.configurations.unwrap_or_default();
        hash_map.insert(k, v);
        self.configurations = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Retrieves the Security Lake configuration object. </p>
    pub fn set_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::Region,
                crate::types::LakeConfigurationResponse,
            >,
        >,
    ) -> Self {
        self.configurations = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetDatalakeOutput`](crate::operation::get_datalake::GetDatalakeOutput).
    pub fn build(self) -> crate::operation::get_datalake::GetDatalakeOutput {
        crate::operation::get_datalake::GetDatalakeOutput {
            configurations: self.configurations,
            _request_id: self._request_id,
        }
    }
}
