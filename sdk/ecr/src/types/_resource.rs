// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about the resource involved in a finding.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Resource {
    /// <p>An object that contains details about the resource involved in a finding.</p>
    #[doc(hidden)]
    pub details: ::std::option::Option<crate::types::ResourceDetails>,
    /// <p>The ID of the resource.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The tags attached to the resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>The type of resource.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
}
impl Resource {
    /// <p>An object that contains details about the resource involved in a finding.</p>
    pub fn details(&self) -> ::std::option::Option<&crate::types::ResourceDetails> {
        self.details.as_ref()
    }
    /// <p>The ID of the resource.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The tags attached to the resource.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>The type of resource.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
}
impl Resource {
    /// Creates a new builder-style object to manufacture [`Resource`](crate::types::Resource).
    pub fn builder() -> crate::types::builders::ResourceBuilder {
        crate::types::builders::ResourceBuilder::default()
    }
}

/// A builder for [`Resource`](crate::types::Resource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceBuilder {
    pub(crate) details: ::std::option::Option<crate::types::ResourceDetails>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
}
impl ResourceBuilder {
    /// <p>An object that contains details about the resource involved in a finding.</p>
    pub fn details(mut self, input: crate::types::ResourceDetails) -> Self {
        self.details = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that contains details about the resource involved in a finding.</p>
    pub fn set_details(
        mut self,
        input: ::std::option::Option<crate::types::ResourceDetails>,
    ) -> Self {
        self.details = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags attached to the resource.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags attached to the resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The type of resource.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of resource.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`Resource`](crate::types::Resource).
    pub fn build(self) -> crate::types::Resource {
        crate::types::Resource {
            details: self.details,
            id: self.id,
            tags: self.tags,
            r#type: self.r#type,
        }
    }
}
