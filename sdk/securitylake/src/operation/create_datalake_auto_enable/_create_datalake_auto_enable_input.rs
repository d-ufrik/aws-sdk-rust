// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDatalakeAutoEnableInput {
    /// <p>Enable Security Lake with the specified configuration settings to begin collecting security data for new accounts in your organization. </p>
    #[doc(hidden)]
    pub configuration_for_new_accounts:
        ::std::option::Option<::std::vec::Vec<crate::types::AutoEnableNewRegionConfiguration>>,
}
impl CreateDatalakeAutoEnableInput {
    /// <p>Enable Security Lake with the specified configuration settings to begin collecting security data for new accounts in your organization. </p>
    pub fn configuration_for_new_accounts(
        &self,
    ) -> ::std::option::Option<&[crate::types::AutoEnableNewRegionConfiguration]> {
        self.configuration_for_new_accounts.as_deref()
    }
}
impl CreateDatalakeAutoEnableInput {
    /// Creates a new builder-style object to manufacture [`CreateDatalakeAutoEnableInput`](crate::operation::create_datalake_auto_enable::CreateDatalakeAutoEnableInput).
    pub fn builder(
    ) -> crate::operation::create_datalake_auto_enable::builders::CreateDatalakeAutoEnableInputBuilder
    {
        crate::operation::create_datalake_auto_enable::builders::CreateDatalakeAutoEnableInputBuilder::default()
    }
}

/// A builder for [`CreateDatalakeAutoEnableInput`](crate::operation::create_datalake_auto_enable::CreateDatalakeAutoEnableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDatalakeAutoEnableInputBuilder {
    pub(crate) configuration_for_new_accounts:
        ::std::option::Option<::std::vec::Vec<crate::types::AutoEnableNewRegionConfiguration>>,
}
impl CreateDatalakeAutoEnableInputBuilder {
    /// Appends an item to `configuration_for_new_accounts`.
    ///
    /// To override the contents of this collection use [`set_configuration_for_new_accounts`](Self::set_configuration_for_new_accounts).
    ///
    /// <p>Enable Security Lake with the specified configuration settings to begin collecting security data for new accounts in your organization. </p>
    pub fn configuration_for_new_accounts(
        mut self,
        input: crate::types::AutoEnableNewRegionConfiguration,
    ) -> Self {
        let mut v = self.configuration_for_new_accounts.unwrap_or_default();
        v.push(input);
        self.configuration_for_new_accounts = ::std::option::Option::Some(v);
        self
    }
    /// <p>Enable Security Lake with the specified configuration settings to begin collecting security data for new accounts in your organization. </p>
    pub fn set_configuration_for_new_accounts(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AutoEnableNewRegionConfiguration>,
        >,
    ) -> Self {
        self.configuration_for_new_accounts = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateDatalakeAutoEnableInput`](crate::operation::create_datalake_auto_enable::CreateDatalakeAutoEnableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_datalake_auto_enable::CreateDatalakeAutoEnableInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_datalake_auto_enable::CreateDatalakeAutoEnableInput {
                configuration_for_new_accounts: self.configuration_for_new_accounts,
            },
        )
    }
}
