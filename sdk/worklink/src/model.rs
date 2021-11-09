// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum IdentityProviderType {
    #[allow(missing_docs)] // documentation missing in model
    Saml,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for IdentityProviderType {
    fn from(s: &str) -> Self {
        match s {
            "SAML" => IdentityProviderType::Saml,
            other => IdentityProviderType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for IdentityProviderType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(IdentityProviderType::from(s))
    }
}
impl IdentityProviderType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            IdentityProviderType::Saml => "SAML",
            IdentityProviderType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["SAML"]
    }
}
impl AsRef<str> for IdentityProviderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The summary of the certificate authority (CA).</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct WebsiteCaSummary {
    /// <p>A unique identifier for the CA.</p>
    pub website_ca_id: std::option::Option<std::string::String>,
    /// <p>The time when the CA was added.</p>
    pub created_time: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The name to display.</p>
    pub display_name: std::option::Option<std::string::String>,
}
impl WebsiteCaSummary {
    /// <p>A unique identifier for the CA.</p>
    pub fn website_ca_id(&self) -> std::option::Option<&str> {
        self.website_ca_id.as_deref()
    }
    /// <p>The time when the CA was added.</p>
    pub fn created_time(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_time.as_ref()
    }
    /// <p>The name to display.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
}
impl std::fmt::Debug for WebsiteCaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("WebsiteCaSummary");
        formatter.field("website_ca_id", &self.website_ca_id);
        formatter.field("created_time", &self.created_time);
        formatter.field("display_name", &self.display_name);
        formatter.finish()
    }
}
/// See [`WebsiteCaSummary`](crate::model::WebsiteCaSummary)
pub mod website_ca_summary {
    /// A builder for [`WebsiteCaSummary`](crate::model::WebsiteCaSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) website_ca_id: std::option::Option<std::string::String>,
        pub(crate) created_time: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) display_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A unique identifier for the CA.</p>
        pub fn website_ca_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.website_ca_id = Some(input.into());
            self
        }
        /// <p>A unique identifier for the CA.</p>
        pub fn set_website_ca_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.website_ca_id = input;
            self
        }
        /// <p>The time when the CA was added.</p>
        pub fn created_time(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_time = Some(input);
            self
        }
        /// <p>The time when the CA was added.</p>
        pub fn set_created_time(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_time = input;
            self
        }
        /// <p>The name to display.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>The name to display.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// Consumes the builder and constructs a [`WebsiteCaSummary`](crate::model::WebsiteCaSummary)
        pub fn build(self) -> crate::model::WebsiteCaSummary {
            crate::model::WebsiteCaSummary {
                website_ca_id: self.website_ca_id,
                created_time: self.created_time,
                display_name: self.display_name,
            }
        }
    }
}
impl WebsiteCaSummary {
    /// Creates a new builder-style object to manufacture [`WebsiteCaSummary`](crate::model::WebsiteCaSummary)
    pub fn builder() -> crate::model::website_ca_summary::Builder {
        crate::model::website_ca_summary::Builder::default()
    }
}

/// <p>The summary of the website authorization provider.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct WebsiteAuthorizationProviderSummary {
    /// <p>A unique identifier for the authorization provider.</p>
    pub authorization_provider_id: std::option::Option<std::string::String>,
    /// <p>The authorization provider type.</p>
    pub authorization_provider_type: std::option::Option<crate::model::AuthorizationProviderType>,
    /// <p>The domain name of the authorization provider. This applies only to SAML-based
    /// authorization providers.</p>
    pub domain_name: std::option::Option<std::string::String>,
    /// <p>The time of creation.</p>
    pub created_time: std::option::Option<aws_smithy_types::Instant>,
}
impl WebsiteAuthorizationProviderSummary {
    /// <p>A unique identifier for the authorization provider.</p>
    pub fn authorization_provider_id(&self) -> std::option::Option<&str> {
        self.authorization_provider_id.as_deref()
    }
    /// <p>The authorization provider type.</p>
    pub fn authorization_provider_type(
        &self,
    ) -> std::option::Option<&crate::model::AuthorizationProviderType> {
        self.authorization_provider_type.as_ref()
    }
    /// <p>The domain name of the authorization provider. This applies only to SAML-based
    /// authorization providers.</p>
    pub fn domain_name(&self) -> std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>The time of creation.</p>
    pub fn created_time(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_time.as_ref()
    }
}
impl std::fmt::Debug for WebsiteAuthorizationProviderSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("WebsiteAuthorizationProviderSummary");
        formatter.field("authorization_provider_id", &self.authorization_provider_id);
        formatter.field(
            "authorization_provider_type",
            &self.authorization_provider_type,
        );
        formatter.field("domain_name", &self.domain_name);
        formatter.field("created_time", &self.created_time);
        formatter.finish()
    }
}
/// See [`WebsiteAuthorizationProviderSummary`](crate::model::WebsiteAuthorizationProviderSummary)
pub mod website_authorization_provider_summary {
    /// A builder for [`WebsiteAuthorizationProviderSummary`](crate::model::WebsiteAuthorizationProviderSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) authorization_provider_id: std::option::Option<std::string::String>,
        pub(crate) authorization_provider_type:
            std::option::Option<crate::model::AuthorizationProviderType>,
        pub(crate) domain_name: std::option::Option<std::string::String>,
        pub(crate) created_time: std::option::Option<aws_smithy_types::Instant>,
    }
    impl Builder {
        /// <p>A unique identifier for the authorization provider.</p>
        pub fn authorization_provider_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.authorization_provider_id = Some(input.into());
            self
        }
        /// <p>A unique identifier for the authorization provider.</p>
        pub fn set_authorization_provider_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.authorization_provider_id = input;
            self
        }
        /// <p>The authorization provider type.</p>
        pub fn authorization_provider_type(
            mut self,
            input: crate::model::AuthorizationProviderType,
        ) -> Self {
            self.authorization_provider_type = Some(input);
            self
        }
        /// <p>The authorization provider type.</p>
        pub fn set_authorization_provider_type(
            mut self,
            input: std::option::Option<crate::model::AuthorizationProviderType>,
        ) -> Self {
            self.authorization_provider_type = input;
            self
        }
        /// <p>The domain name of the authorization provider. This applies only to SAML-based
        /// authorization providers.</p>
        pub fn domain_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.domain_name = Some(input.into());
            self
        }
        /// <p>The domain name of the authorization provider. This applies only to SAML-based
        /// authorization providers.</p>
        pub fn set_domain_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.domain_name = input;
            self
        }
        /// <p>The time of creation.</p>
        pub fn created_time(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_time = Some(input);
            self
        }
        /// <p>The time of creation.</p>
        pub fn set_created_time(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_time = input;
            self
        }
        /// Consumes the builder and constructs a [`WebsiteAuthorizationProviderSummary`](crate::model::WebsiteAuthorizationProviderSummary)
        pub fn build(self) -> crate::model::WebsiteAuthorizationProviderSummary {
            crate::model::WebsiteAuthorizationProviderSummary {
                authorization_provider_id: self.authorization_provider_id,
                authorization_provider_type: self.authorization_provider_type,
                domain_name: self.domain_name,
                created_time: self.created_time,
            }
        }
    }
}
impl WebsiteAuthorizationProviderSummary {
    /// Creates a new builder-style object to manufacture [`WebsiteAuthorizationProviderSummary`](crate::model::WebsiteAuthorizationProviderSummary)
    pub fn builder() -> crate::model::website_authorization_provider_summary::Builder {
        crate::model::website_authorization_provider_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum AuthorizationProviderType {
    #[allow(missing_docs)] // documentation missing in model
    Saml,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for AuthorizationProviderType {
    fn from(s: &str) -> Self {
        match s {
            "SAML" => AuthorizationProviderType::Saml,
            other => AuthorizationProviderType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for AuthorizationProviderType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(AuthorizationProviderType::from(s))
    }
}
impl AuthorizationProviderType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            AuthorizationProviderType::Saml => "SAML",
            AuthorizationProviderType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["SAML"]
    }
}
impl AsRef<str> for AuthorizationProviderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The summary of the fleet.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct FleetSummary {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    pub fleet_arn: std::option::Option<std::string::String>,
    /// <p>The time when the fleet was created.</p>
    pub created_time: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The time when the fleet was last updated.</p>
    pub last_updated_time: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The name of the fleet.</p>
    pub fleet_name: std::option::Option<std::string::String>,
    /// <p>The name of the fleet to display.</p>
    pub display_name: std::option::Option<std::string::String>,
    /// <p>The identifier used by users to sign into the Amazon WorkLink app.</p>
    pub company_code: std::option::Option<std::string::String>,
    /// <p>The status of the fleet.</p>
    pub fleet_status: std::option::Option<crate::model::FleetStatus>,
    /// <p>The tags attached to the resource. A tag is a key-value pair.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl FleetSummary {
    /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
    pub fn fleet_arn(&self) -> std::option::Option<&str> {
        self.fleet_arn.as_deref()
    }
    /// <p>The time when the fleet was created.</p>
    pub fn created_time(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_time.as_ref()
    }
    /// <p>The time when the fleet was last updated.</p>
    pub fn last_updated_time(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_updated_time.as_ref()
    }
    /// <p>The name of the fleet.</p>
    pub fn fleet_name(&self) -> std::option::Option<&str> {
        self.fleet_name.as_deref()
    }
    /// <p>The name of the fleet to display.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The identifier used by users to sign into the Amazon WorkLink app.</p>
    pub fn company_code(&self) -> std::option::Option<&str> {
        self.company_code.as_deref()
    }
    /// <p>The status of the fleet.</p>
    pub fn fleet_status(&self) -> std::option::Option<&crate::model::FleetStatus> {
        self.fleet_status.as_ref()
    }
    /// <p>The tags attached to the resource. A tag is a key-value pair.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for FleetSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("FleetSummary");
        formatter.field("fleet_arn", &self.fleet_arn);
        formatter.field("created_time", &self.created_time);
        formatter.field("last_updated_time", &self.last_updated_time);
        formatter.field("fleet_name", &self.fleet_name);
        formatter.field("display_name", &self.display_name);
        formatter.field("company_code", &self.company_code);
        formatter.field("fleet_status", &self.fleet_status);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`FleetSummary`](crate::model::FleetSummary)
pub mod fleet_summary {
    /// A builder for [`FleetSummary`](crate::model::FleetSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) fleet_arn: std::option::Option<std::string::String>,
        pub(crate) created_time: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) last_updated_time: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) fleet_name: std::option::Option<std::string::String>,
        pub(crate) display_name: std::option::Option<std::string::String>,
        pub(crate) company_code: std::option::Option<std::string::String>,
        pub(crate) fleet_status: std::option::Option<crate::model::FleetStatus>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
        pub fn fleet_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.fleet_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the fleet.</p>
        pub fn set_fleet_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.fleet_arn = input;
            self
        }
        /// <p>The time when the fleet was created.</p>
        pub fn created_time(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_time = Some(input);
            self
        }
        /// <p>The time when the fleet was created.</p>
        pub fn set_created_time(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_time = input;
            self
        }
        /// <p>The time when the fleet was last updated.</p>
        pub fn last_updated_time(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_updated_time = Some(input);
            self
        }
        /// <p>The time when the fleet was last updated.</p>
        pub fn set_last_updated_time(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_updated_time = input;
            self
        }
        /// <p>The name of the fleet.</p>
        pub fn fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.fleet_name = Some(input.into());
            self
        }
        /// <p>The name of the fleet.</p>
        pub fn set_fleet_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.fleet_name = input;
            self
        }
        /// <p>The name of the fleet to display.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>The name of the fleet to display.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// <p>The identifier used by users to sign into the Amazon WorkLink app.</p>
        pub fn company_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.company_code = Some(input.into());
            self
        }
        /// <p>The identifier used by users to sign into the Amazon WorkLink app.</p>
        pub fn set_company_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.company_code = input;
            self
        }
        /// <p>The status of the fleet.</p>
        pub fn fleet_status(mut self, input: crate::model::FleetStatus) -> Self {
            self.fleet_status = Some(input);
            self
        }
        /// <p>The status of the fleet.</p>
        pub fn set_fleet_status(
            mut self,
            input: std::option::Option<crate::model::FleetStatus>,
        ) -> Self {
            self.fleet_status = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags attached to the resource. A tag is a key-value pair.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>The tags attached to the resource. A tag is a key-value pair.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`FleetSummary`](crate::model::FleetSummary)
        pub fn build(self) -> crate::model::FleetSummary {
            crate::model::FleetSummary {
                fleet_arn: self.fleet_arn,
                created_time: self.created_time,
                last_updated_time: self.last_updated_time,
                fleet_name: self.fleet_name,
                display_name: self.display_name,
                company_code: self.company_code,
                fleet_status: self.fleet_status,
                tags: self.tags,
            }
        }
    }
}
impl FleetSummary {
    /// Creates a new builder-style object to manufacture [`FleetSummary`](crate::model::FleetSummary)
    pub fn builder() -> crate::model::fleet_summary::Builder {
        crate::model::fleet_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum FleetStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Creating,
    #[allow(missing_docs)] // documentation missing in model
    Deleted,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    #[allow(missing_docs)] // documentation missing in model
    FailedToCreate,
    #[allow(missing_docs)] // documentation missing in model
    FailedToDelete,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for FleetStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => FleetStatus::Active,
            "CREATING" => FleetStatus::Creating,
            "DELETED" => FleetStatus::Deleted,
            "DELETING" => FleetStatus::Deleting,
            "FAILED_TO_CREATE" => FleetStatus::FailedToCreate,
            "FAILED_TO_DELETE" => FleetStatus::FailedToDelete,
            other => FleetStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for FleetStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(FleetStatus::from(s))
    }
}
impl FleetStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            FleetStatus::Active => "ACTIVE",
            FleetStatus::Creating => "CREATING",
            FleetStatus::Deleted => "DELETED",
            FleetStatus::Deleting => "DELETING",
            FleetStatus::FailedToCreate => "FAILED_TO_CREATE",
            FleetStatus::FailedToDelete => "FAILED_TO_DELETE",
            FleetStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "CREATING",
            "DELETED",
            "DELETING",
            "FAILED_TO_CREATE",
            "FAILED_TO_DELETE",
        ]
    }
}
impl AsRef<str> for FleetStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The summary of the domain.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DomainSummary {
    /// <p>The name of the domain.</p>
    pub domain_name: std::option::Option<std::string::String>,
    /// <p>The name to display.</p>
    pub display_name: std::option::Option<std::string::String>,
    /// <p>The time that the domain was created.</p>
    pub created_time: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The status of the domain.</p>
    pub domain_status: std::option::Option<crate::model::DomainStatus>,
}
impl DomainSummary {
    /// <p>The name of the domain.</p>
    pub fn domain_name(&self) -> std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>The name to display.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The time that the domain was created.</p>
    pub fn created_time(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_time.as_ref()
    }
    /// <p>The status of the domain.</p>
    pub fn domain_status(&self) -> std::option::Option<&crate::model::DomainStatus> {
        self.domain_status.as_ref()
    }
}
impl std::fmt::Debug for DomainSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DomainSummary");
        formatter.field("domain_name", &self.domain_name);
        formatter.field("display_name", &self.display_name);
        formatter.field("created_time", &self.created_time);
        formatter.field("domain_status", &self.domain_status);
        formatter.finish()
    }
}
/// See [`DomainSummary`](crate::model::DomainSummary)
pub mod domain_summary {
    /// A builder for [`DomainSummary`](crate::model::DomainSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) domain_name: std::option::Option<std::string::String>,
        pub(crate) display_name: std::option::Option<std::string::String>,
        pub(crate) created_time: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) domain_status: std::option::Option<crate::model::DomainStatus>,
    }
    impl Builder {
        /// <p>The name of the domain.</p>
        pub fn domain_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.domain_name = Some(input.into());
            self
        }
        /// <p>The name of the domain.</p>
        pub fn set_domain_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.domain_name = input;
            self
        }
        /// <p>The name to display.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>The name to display.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// <p>The time that the domain was created.</p>
        pub fn created_time(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_time = Some(input);
            self
        }
        /// <p>The time that the domain was created.</p>
        pub fn set_created_time(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_time = input;
            self
        }
        /// <p>The status of the domain.</p>
        pub fn domain_status(mut self, input: crate::model::DomainStatus) -> Self {
            self.domain_status = Some(input);
            self
        }
        /// <p>The status of the domain.</p>
        pub fn set_domain_status(
            mut self,
            input: std::option::Option<crate::model::DomainStatus>,
        ) -> Self {
            self.domain_status = input;
            self
        }
        /// Consumes the builder and constructs a [`DomainSummary`](crate::model::DomainSummary)
        pub fn build(self) -> crate::model::DomainSummary {
            crate::model::DomainSummary {
                domain_name: self.domain_name,
                display_name: self.display_name,
                created_time: self.created_time,
                domain_status: self.domain_status,
            }
        }
    }
}
impl DomainSummary {
    /// Creates a new builder-style object to manufacture [`DomainSummary`](crate::model::DomainSummary)
    pub fn builder() -> crate::model::domain_summary::Builder {
        crate::model::domain_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DomainStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Associating,
    #[allow(missing_docs)] // documentation missing in model
    Disassociated,
    #[allow(missing_docs)] // documentation missing in model
    Disassociating,
    #[allow(missing_docs)] // documentation missing in model
    FailedToAssociate,
    #[allow(missing_docs)] // documentation missing in model
    FailedToDisassociate,
    #[allow(missing_docs)] // documentation missing in model
    Inactive,
    #[allow(missing_docs)] // documentation missing in model
    PendingValidation,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DomainStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => DomainStatus::Active,
            "ASSOCIATING" => DomainStatus::Associating,
            "DISASSOCIATED" => DomainStatus::Disassociated,
            "DISASSOCIATING" => DomainStatus::Disassociating,
            "FAILED_TO_ASSOCIATE" => DomainStatus::FailedToAssociate,
            "FAILED_TO_DISASSOCIATE" => DomainStatus::FailedToDisassociate,
            "INACTIVE" => DomainStatus::Inactive,
            "PENDING_VALIDATION" => DomainStatus::PendingValidation,
            other => DomainStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DomainStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DomainStatus::from(s))
    }
}
impl DomainStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DomainStatus::Active => "ACTIVE",
            DomainStatus::Associating => "ASSOCIATING",
            DomainStatus::Disassociated => "DISASSOCIATED",
            DomainStatus::Disassociating => "DISASSOCIATING",
            DomainStatus::FailedToAssociate => "FAILED_TO_ASSOCIATE",
            DomainStatus::FailedToDisassociate => "FAILED_TO_DISASSOCIATE",
            DomainStatus::Inactive => "INACTIVE",
            DomainStatus::PendingValidation => "PENDING_VALIDATION",
            DomainStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "ASSOCIATING",
            "DISASSOCIATED",
            "DISASSOCIATING",
            "FAILED_TO_ASSOCIATE",
            "FAILED_TO_DISASSOCIATE",
            "INACTIVE",
            "PENDING_VALIDATION",
        ]
    }
}
impl AsRef<str> for DomainStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The summary of devices.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeviceSummary {
    /// <p>The ID of the device.</p>
    pub device_id: std::option::Option<std::string::String>,
    /// <p>The status of the device.</p>
    pub device_status: std::option::Option<crate::model::DeviceStatus>,
}
impl DeviceSummary {
    /// <p>The ID of the device.</p>
    pub fn device_id(&self) -> std::option::Option<&str> {
        self.device_id.as_deref()
    }
    /// <p>The status of the device.</p>
    pub fn device_status(&self) -> std::option::Option<&crate::model::DeviceStatus> {
        self.device_status.as_ref()
    }
}
impl std::fmt::Debug for DeviceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeviceSummary");
        formatter.field("device_id", &self.device_id);
        formatter.field("device_status", &self.device_status);
        formatter.finish()
    }
}
/// See [`DeviceSummary`](crate::model::DeviceSummary)
pub mod device_summary {
    /// A builder for [`DeviceSummary`](crate::model::DeviceSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_id: std::option::Option<std::string::String>,
        pub(crate) device_status: std::option::Option<crate::model::DeviceStatus>,
    }
    impl Builder {
        /// <p>The ID of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_id = Some(input.into());
            self
        }
        /// <p>The ID of the device.</p>
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_id = input;
            self
        }
        /// <p>The status of the device.</p>
        pub fn device_status(mut self, input: crate::model::DeviceStatus) -> Self {
            self.device_status = Some(input);
            self
        }
        /// <p>The status of the device.</p>
        pub fn set_device_status(
            mut self,
            input: std::option::Option<crate::model::DeviceStatus>,
        ) -> Self {
            self.device_status = input;
            self
        }
        /// Consumes the builder and constructs a [`DeviceSummary`](crate::model::DeviceSummary)
        pub fn build(self) -> crate::model::DeviceSummary {
            crate::model::DeviceSummary {
                device_id: self.device_id,
                device_status: self.device_status,
            }
        }
    }
}
impl DeviceSummary {
    /// Creates a new builder-style object to manufacture [`DeviceSummary`](crate::model::DeviceSummary)
    pub fn builder() -> crate::model::device_summary::Builder {
        crate::model::device_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DeviceStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    SignedOut,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DeviceStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => DeviceStatus::Active,
            "SIGNED_OUT" => DeviceStatus::SignedOut,
            other => DeviceStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DeviceStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DeviceStatus::from(s))
    }
}
impl DeviceStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DeviceStatus::Active => "ACTIVE",
            DeviceStatus::SignedOut => "SIGNED_OUT",
            DeviceStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ACTIVE", "SIGNED_OUT"]
    }
}
impl AsRef<str> for DeviceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
