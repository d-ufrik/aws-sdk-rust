// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Access Control List. You can authenticate users with Access Contol Lists. ACLs enable you to control cluster access by grouping users. These Access control lists are designed as a way to organize access to clusters.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Acl {
    /// <p>The name of the Access Control List</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Indicates ACL status. Can be "creating", "active", "modifying", "deleting".</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The list of user names that belong to the ACL.</p>
    #[doc(hidden)]
    pub user_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The minimum engine version supported for the ACL</p>
    #[doc(hidden)]
    pub minimum_engine_version: ::std::option::Option<::std::string::String>,
    /// <p>A list of updates being applied to the ACL.</p>
    #[doc(hidden)]
    pub pending_changes: ::std::option::Option<crate::types::AclPendingChanges>,
    /// <p>A list of clusters associated with the ACL.</p>
    #[doc(hidden)]
    pub clusters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The Amazon Resource Name (ARN) of the ACL</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl Acl {
    /// <p>The name of the Access Control List</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Indicates ACL status. Can be "creating", "active", "modifying", "deleting".</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The list of user names that belong to the ACL.</p>
    pub fn user_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.user_names.as_deref()
    }
    /// <p>The minimum engine version supported for the ACL</p>
    pub fn minimum_engine_version(&self) -> ::std::option::Option<&str> {
        self.minimum_engine_version.as_deref()
    }
    /// <p>A list of updates being applied to the ACL.</p>
    pub fn pending_changes(&self) -> ::std::option::Option<&crate::types::AclPendingChanges> {
        self.pending_changes.as_ref()
    }
    /// <p>A list of clusters associated with the ACL.</p>
    pub fn clusters(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.clusters.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the ACL</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl Acl {
    /// Creates a new builder-style object to manufacture [`Acl`](crate::types::Acl).
    pub fn builder() -> crate::types::builders::AclBuilder {
        crate::types::builders::AclBuilder::default()
    }
}

/// A builder for [`Acl`](crate::types::Acl).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AclBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) user_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) minimum_engine_version: ::std::option::Option<::std::string::String>,
    pub(crate) pending_changes: ::std::option::Option<crate::types::AclPendingChanges>,
    pub(crate) clusters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl AclBuilder {
    /// <p>The name of the Access Control List</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Access Control List</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Indicates ACL status. Can be "creating", "active", "modifying", "deleting".</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates ACL status. Can be "creating", "active", "modifying", "deleting".</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `user_names`.
    ///
    /// To override the contents of this collection use [`set_user_names`](Self::set_user_names).
    ///
    /// <p>The list of user names that belong to the ACL.</p>
    pub fn user_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.user_names.unwrap_or_default();
        v.push(input.into());
        self.user_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of user names that belong to the ACL.</p>
    pub fn set_user_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.user_names = input;
        self
    }
    /// <p>The minimum engine version supported for the ACL</p>
    pub fn minimum_engine_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.minimum_engine_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The minimum engine version supported for the ACL</p>
    pub fn set_minimum_engine_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.minimum_engine_version = input;
        self
    }
    /// <p>A list of updates being applied to the ACL.</p>
    pub fn pending_changes(mut self, input: crate::types::AclPendingChanges) -> Self {
        self.pending_changes = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of updates being applied to the ACL.</p>
    pub fn set_pending_changes(
        mut self,
        input: ::std::option::Option<crate::types::AclPendingChanges>,
    ) -> Self {
        self.pending_changes = input;
        self
    }
    /// Appends an item to `clusters`.
    ///
    /// To override the contents of this collection use [`set_clusters`](Self::set_clusters).
    ///
    /// <p>A list of clusters associated with the ACL.</p>
    pub fn clusters(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.clusters.unwrap_or_default();
        v.push(input.into());
        self.clusters = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of clusters associated with the ACL.</p>
    pub fn set_clusters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.clusters = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the ACL</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the ACL</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`Acl`](crate::types::Acl).
    pub fn build(self) -> crate::types::Acl {
        crate::types::Acl {
            name: self.name,
            status: self.status,
            user_names: self.user_names,
            minimum_engine_version: self.minimum_engine_version,
            pending_changes: self.pending_changes,
            clusters: self.clusters,
            arn: self.arn,
        }
    }
}
