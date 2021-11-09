// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>An arbitary key/value pair used to add searchable metadata to secure tunnel
/// resources.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The value of the tag.</p>
    pub value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>The key of the tag.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The value of the tag.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The key of the tag.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>The key of the tag.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The value of the tag.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The value of the tag.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>Tunnel timeout configuration.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TimeoutConfig {
    /// <p>The maximum amount of time (in minutes) a tunnel can remain open. If not specified,
    /// maxLifetimeTimeoutMinutes defaults to 720 minutes. Valid values are from 1 minute to 12
    /// hours (720 minutes) </p>
    pub max_lifetime_timeout_minutes: std::option::Option<i32>,
}
impl TimeoutConfig {
    /// <p>The maximum amount of time (in minutes) a tunnel can remain open. If not specified,
    /// maxLifetimeTimeoutMinutes defaults to 720 minutes. Valid values are from 1 minute to 12
    /// hours (720 minutes) </p>
    pub fn max_lifetime_timeout_minutes(&self) -> std::option::Option<i32> {
        self.max_lifetime_timeout_minutes
    }
}
impl std::fmt::Debug for TimeoutConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TimeoutConfig");
        formatter.field(
            "max_lifetime_timeout_minutes",
            &self.max_lifetime_timeout_minutes,
        );
        formatter.finish()
    }
}
/// See [`TimeoutConfig`](crate::model::TimeoutConfig)
pub mod timeout_config {
    /// A builder for [`TimeoutConfig`](crate::model::TimeoutConfig)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) max_lifetime_timeout_minutes: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The maximum amount of time (in minutes) a tunnel can remain open. If not specified,
        /// maxLifetimeTimeoutMinutes defaults to 720 minutes. Valid values are from 1 minute to 12
        /// hours (720 minutes) </p>
        pub fn max_lifetime_timeout_minutes(mut self, input: i32) -> Self {
            self.max_lifetime_timeout_minutes = Some(input);
            self
        }
        /// <p>The maximum amount of time (in minutes) a tunnel can remain open. If not specified,
        /// maxLifetimeTimeoutMinutes defaults to 720 minutes. Valid values are from 1 minute to 12
        /// hours (720 minutes) </p>
        pub fn set_max_lifetime_timeout_minutes(mut self, input: std::option::Option<i32>) -> Self {
            self.max_lifetime_timeout_minutes = input;
            self
        }
        /// Consumes the builder and constructs a [`TimeoutConfig`](crate::model::TimeoutConfig)
        pub fn build(self) -> crate::model::TimeoutConfig {
            crate::model::TimeoutConfig {
                max_lifetime_timeout_minutes: self.max_lifetime_timeout_minutes,
            }
        }
    }
}
impl TimeoutConfig {
    /// Creates a new builder-style object to manufacture [`TimeoutConfig`](crate::model::TimeoutConfig)
    pub fn builder() -> crate::model::timeout_config::Builder {
        crate::model::timeout_config::Builder::default()
    }
}

/// <p>The destination configuration.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DestinationConfig {
    /// <p>The name of the IoT thing to which you want to connect.</p>
    pub thing_name: std::option::Option<std::string::String>,
    /// <p>A list of service names that identity the target application. The AWS IoT client running on the destination device reads
    /// this value and uses it to look up a port or an IP address and a port. The AWS IoT client
    /// instantiates the local proxy which uses this information to connect to the destination
    /// application.</p>
    pub services: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DestinationConfig {
    /// <p>The name of the IoT thing to which you want to connect.</p>
    pub fn thing_name(&self) -> std::option::Option<&str> {
        self.thing_name.as_deref()
    }
    /// <p>A list of service names that identity the target application. The AWS IoT client running on the destination device reads
    /// this value and uses it to look up a port or an IP address and a port. The AWS IoT client
    /// instantiates the local proxy which uses this information to connect to the destination
    /// application.</p>
    pub fn services(&self) -> std::option::Option<&[std::string::String]> {
        self.services.as_deref()
    }
}
impl std::fmt::Debug for DestinationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DestinationConfig");
        formatter.field("thing_name", &self.thing_name);
        formatter.field("services", &self.services);
        formatter.finish()
    }
}
/// See [`DestinationConfig`](crate::model::DestinationConfig)
pub mod destination_config {
    /// A builder for [`DestinationConfig`](crate::model::DestinationConfig)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) thing_name: std::option::Option<std::string::String>,
        pub(crate) services: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>The name of the IoT thing to which you want to connect.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.thing_name = Some(input.into());
            self
        }
        /// <p>The name of the IoT thing to which you want to connect.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.thing_name = input;
            self
        }
        /// Appends an item to `services`.
        ///
        /// To override the contents of this collection use [`set_services`](Self::set_services).
        ///
        /// <p>A list of service names that identity the target application. The AWS IoT client running on the destination device reads
        /// this value and uses it to look up a port or an IP address and a port. The AWS IoT client
        /// instantiates the local proxy which uses this information to connect to the destination
        /// application.</p>
        pub fn services(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.services.unwrap_or_default();
            v.push(input.into());
            self.services = Some(v);
            self
        }
        /// <p>A list of service names that identity the target application. The AWS IoT client running on the destination device reads
        /// this value and uses it to look up a port or an IP address and a port. The AWS IoT client
        /// instantiates the local proxy which uses this information to connect to the destination
        /// application.</p>
        pub fn set_services(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.services = input;
            self
        }
        /// Consumes the builder and constructs a [`DestinationConfig`](crate::model::DestinationConfig)
        pub fn build(self) -> crate::model::DestinationConfig {
            crate::model::DestinationConfig {
                thing_name: self.thing_name,
                services: self.services,
            }
        }
    }
}
impl DestinationConfig {
    /// Creates a new builder-style object to manufacture [`DestinationConfig`](crate::model::DestinationConfig)
    pub fn builder() -> crate::model::destination_config::Builder {
        crate::model::destination_config::Builder::default()
    }
}

/// <p>Information about the tunnel.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TunnelSummary {
    /// <p>The unique alpha-numeric identifier for the tunnel.</p>
    pub tunnel_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name of the tunnel. The tunnel ARN format is
    /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
    /// </p>
    pub tunnel_arn: std::option::Option<std::string::String>,
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    pub status: std::option::Option<crate::model::TunnelStatus>,
    /// <p>A description of the tunnel.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>The time the tunnel was created.</p>
    pub created_at: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The time the tunnel was last updated.</p>
    pub last_updated_at: std::option::Option<aws_smithy_types::Instant>,
}
impl TunnelSummary {
    /// <p>The unique alpha-numeric identifier for the tunnel.</p>
    pub fn tunnel_id(&self) -> std::option::Option<&str> {
        self.tunnel_id.as_deref()
    }
    /// <p>The Amazon Resource Name of the tunnel. The tunnel ARN format is
    /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
    /// </p>
    pub fn tunnel_arn(&self) -> std::option::Option<&str> {
        self.tunnel_arn.as_deref()
    }
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::TunnelStatus> {
        self.status.as_ref()
    }
    /// <p>A description of the tunnel.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The time the tunnel was created.</p>
    pub fn created_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_at.as_ref()
    }
    /// <p>The time the tunnel was last updated.</p>
    pub fn last_updated_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_updated_at.as_ref()
    }
}
impl std::fmt::Debug for TunnelSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TunnelSummary");
        formatter.field("tunnel_id", &self.tunnel_id);
        formatter.field("tunnel_arn", &self.tunnel_arn);
        formatter.field("status", &self.status);
        formatter.field("description", &self.description);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.finish()
    }
}
/// See [`TunnelSummary`](crate::model::TunnelSummary)
pub mod tunnel_summary {
    /// A builder for [`TunnelSummary`](crate::model::TunnelSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tunnel_id: std::option::Option<std::string::String>,
        pub(crate) tunnel_arn: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::TunnelStatus>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) last_updated_at: std::option::Option<aws_smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The unique alpha-numeric identifier for the tunnel.</p>
        pub fn tunnel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_id = Some(input.into());
            self
        }
        /// <p>The unique alpha-numeric identifier for the tunnel.</p>
        pub fn set_tunnel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_id = input;
            self
        }
        /// <p>The Amazon Resource Name of the tunnel. The tunnel ARN format is
        /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
        /// </p>
        pub fn tunnel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name of the tunnel. The tunnel ARN format is
        /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
        /// </p>
        pub fn set_tunnel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_arn = input;
            self
        }
        /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
        pub fn status(mut self, input: crate::model::TunnelStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::TunnelStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>A description of the tunnel.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>A description of the tunnel.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The time the tunnel was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>The time the tunnel was created.</p>
        pub fn set_created_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_at = input;
            self
        }
        /// <p>The time the tunnel was last updated.</p>
        pub fn last_updated_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>The time the tunnel was last updated.</p>
        pub fn set_last_updated_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_updated_at = input;
            self
        }
        /// Consumes the builder and constructs a [`TunnelSummary`](crate::model::TunnelSummary)
        pub fn build(self) -> crate::model::TunnelSummary {
            crate::model::TunnelSummary {
                tunnel_id: self.tunnel_id,
                tunnel_arn: self.tunnel_arn,
                status: self.status,
                description: self.description,
                created_at: self.created_at,
                last_updated_at: self.last_updated_at,
            }
        }
    }
}
impl TunnelSummary {
    /// Creates a new builder-style object to manufacture [`TunnelSummary`](crate::model::TunnelSummary)
    pub fn builder() -> crate::model::tunnel_summary::Builder {
        crate::model::tunnel_summary::Builder::default()
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
pub enum TunnelStatus {
    #[allow(missing_docs)] // documentation missing in model
    Closed,
    #[allow(missing_docs)] // documentation missing in model
    Open,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for TunnelStatus {
    fn from(s: &str) -> Self {
        match s {
            "CLOSED" => TunnelStatus::Closed,
            "OPEN" => TunnelStatus::Open,
            other => TunnelStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for TunnelStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TunnelStatus::from(s))
    }
}
impl TunnelStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TunnelStatus::Closed => "CLOSED",
            TunnelStatus::Open => "OPEN",
            TunnelStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CLOSED", "OPEN"]
    }
}
impl AsRef<str> for TunnelStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A connection between a source computer and a destination device.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tunnel {
    /// <p>A unique alpha-numeric ID that identifies a tunnel.</p>
    pub tunnel_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of a tunnel. The tunnel ARN format is
    /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
    /// </p>
    pub tunnel_arn: std::option::Option<std::string::String>,
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    pub status: std::option::Option<crate::model::TunnelStatus>,
    /// <p>The connection state of the source application.</p>
    pub source_connection_state: std::option::Option<crate::model::ConnectionState>,
    /// <p>The connection state of the destination application.</p>
    pub destination_connection_state: std::option::Option<crate::model::ConnectionState>,
    /// <p>A description of the tunnel.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>The destination configuration that specifies the thing name of the destination
    /// device and a service name that the local proxy uses to connect to the destination
    /// application.</p>
    pub destination_config: std::option::Option<crate::model::DestinationConfig>,
    /// <p>Timeout configuration for the tunnel.</p>
    pub timeout_config: std::option::Option<crate::model::TimeoutConfig>,
    /// <p>A list of tag metadata associated with the secure tunnel.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>The time when the tunnel was created.</p>
    pub created_at: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The last time the tunnel was updated.</p>
    pub last_updated_at: std::option::Option<aws_smithy_types::Instant>,
}
impl Tunnel {
    /// <p>A unique alpha-numeric ID that identifies a tunnel.</p>
    pub fn tunnel_id(&self) -> std::option::Option<&str> {
        self.tunnel_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of a tunnel. The tunnel ARN format is
    /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
    /// </p>
    pub fn tunnel_arn(&self) -> std::option::Option<&str> {
        self.tunnel_arn.as_deref()
    }
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::TunnelStatus> {
        self.status.as_ref()
    }
    /// <p>The connection state of the source application.</p>
    pub fn source_connection_state(&self) -> std::option::Option<&crate::model::ConnectionState> {
        self.source_connection_state.as_ref()
    }
    /// <p>The connection state of the destination application.</p>
    pub fn destination_connection_state(
        &self,
    ) -> std::option::Option<&crate::model::ConnectionState> {
        self.destination_connection_state.as_ref()
    }
    /// <p>A description of the tunnel.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The destination configuration that specifies the thing name of the destination
    /// device and a service name that the local proxy uses to connect to the destination
    /// application.</p>
    pub fn destination_config(&self) -> std::option::Option<&crate::model::DestinationConfig> {
        self.destination_config.as_ref()
    }
    /// <p>Timeout configuration for the tunnel.</p>
    pub fn timeout_config(&self) -> std::option::Option<&crate::model::TimeoutConfig> {
        self.timeout_config.as_ref()
    }
    /// <p>A list of tag metadata associated with the secure tunnel.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The time when the tunnel was created.</p>
    pub fn created_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_at.as_ref()
    }
    /// <p>The last time the tunnel was updated.</p>
    pub fn last_updated_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_updated_at.as_ref()
    }
}
impl std::fmt::Debug for Tunnel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tunnel");
        formatter.field("tunnel_id", &self.tunnel_id);
        formatter.field("tunnel_arn", &self.tunnel_arn);
        formatter.field("status", &self.status);
        formatter.field("source_connection_state", &self.source_connection_state);
        formatter.field(
            "destination_connection_state",
            &self.destination_connection_state,
        );
        formatter.field("description", &self.description);
        formatter.field("destination_config", &self.destination_config);
        formatter.field("timeout_config", &self.timeout_config);
        formatter.field("tags", &self.tags);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.finish()
    }
}
/// See [`Tunnel`](crate::model::Tunnel)
pub mod tunnel {
    /// A builder for [`Tunnel`](crate::model::Tunnel)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tunnel_id: std::option::Option<std::string::String>,
        pub(crate) tunnel_arn: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::TunnelStatus>,
        pub(crate) source_connection_state: std::option::Option<crate::model::ConnectionState>,
        pub(crate) destination_connection_state: std::option::Option<crate::model::ConnectionState>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) destination_config: std::option::Option<crate::model::DestinationConfig>,
        pub(crate) timeout_config: std::option::Option<crate::model::TimeoutConfig>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) last_updated_at: std::option::Option<aws_smithy_types::Instant>,
    }
    impl Builder {
        /// <p>A unique alpha-numeric ID that identifies a tunnel.</p>
        pub fn tunnel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_id = Some(input.into());
            self
        }
        /// <p>A unique alpha-numeric ID that identifies a tunnel.</p>
        pub fn set_tunnel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_id = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a tunnel. The tunnel ARN format is
        /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
        /// </p>
        pub fn tunnel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a tunnel. The tunnel ARN format is
        /// <code>arn:aws:tunnel:<region>:<account-id>:tunnel/<tunnel-id></code>
        /// </p>
        pub fn set_tunnel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_arn = input;
            self
        }
        /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
        pub fn status(mut self, input: crate::model::TunnelStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::TunnelStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>The connection state of the source application.</p>
        pub fn source_connection_state(mut self, input: crate::model::ConnectionState) -> Self {
            self.source_connection_state = Some(input);
            self
        }
        /// <p>The connection state of the source application.</p>
        pub fn set_source_connection_state(
            mut self,
            input: std::option::Option<crate::model::ConnectionState>,
        ) -> Self {
            self.source_connection_state = input;
            self
        }
        /// <p>The connection state of the destination application.</p>
        pub fn destination_connection_state(
            mut self,
            input: crate::model::ConnectionState,
        ) -> Self {
            self.destination_connection_state = Some(input);
            self
        }
        /// <p>The connection state of the destination application.</p>
        pub fn set_destination_connection_state(
            mut self,
            input: std::option::Option<crate::model::ConnectionState>,
        ) -> Self {
            self.destination_connection_state = input;
            self
        }
        /// <p>A description of the tunnel.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>A description of the tunnel.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The destination configuration that specifies the thing name of the destination
        /// device and a service name that the local proxy uses to connect to the destination
        /// application.</p>
        pub fn destination_config(mut self, input: crate::model::DestinationConfig) -> Self {
            self.destination_config = Some(input);
            self
        }
        /// <p>The destination configuration that specifies the thing name of the destination
        /// device and a service name that the local proxy uses to connect to the destination
        /// application.</p>
        pub fn set_destination_config(
            mut self,
            input: std::option::Option<crate::model::DestinationConfig>,
        ) -> Self {
            self.destination_config = input;
            self
        }
        /// <p>Timeout configuration for the tunnel.</p>
        pub fn timeout_config(mut self, input: crate::model::TimeoutConfig) -> Self {
            self.timeout_config = Some(input);
            self
        }
        /// <p>Timeout configuration for the tunnel.</p>
        pub fn set_timeout_config(
            mut self,
            input: std::option::Option<crate::model::TimeoutConfig>,
        ) -> Self {
            self.timeout_config = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>A list of tag metadata associated with the secure tunnel.</p>
        pub fn tags(mut self, input: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input.into());
            self.tags = Some(v);
            self
        }
        /// <p>A list of tag metadata associated with the secure tunnel.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// <p>The time when the tunnel was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>The time when the tunnel was created.</p>
        pub fn set_created_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_at = input;
            self
        }
        /// <p>The last time the tunnel was updated.</p>
        pub fn last_updated_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>The last time the tunnel was updated.</p>
        pub fn set_last_updated_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_updated_at = input;
            self
        }
        /// Consumes the builder and constructs a [`Tunnel`](crate::model::Tunnel)
        pub fn build(self) -> crate::model::Tunnel {
            crate::model::Tunnel {
                tunnel_id: self.tunnel_id,
                tunnel_arn: self.tunnel_arn,
                status: self.status,
                source_connection_state: self.source_connection_state,
                destination_connection_state: self.destination_connection_state,
                description: self.description,
                destination_config: self.destination_config,
                timeout_config: self.timeout_config,
                tags: self.tags,
                created_at: self.created_at,
                last_updated_at: self.last_updated_at,
            }
        }
    }
}
impl Tunnel {
    /// Creates a new builder-style object to manufacture [`Tunnel`](crate::model::Tunnel)
    pub fn builder() -> crate::model::tunnel::Builder {
        crate::model::tunnel::Builder::default()
    }
}

/// <p>The state of a connection.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ConnectionState {
    /// <p>The connection status of the tunnel. Valid values are <code>CONNECTED</code> and
    /// <code>DISCONNECTED</code>.</p>
    pub status: std::option::Option<crate::model::ConnectionStatus>,
    /// <p>The last time the connection status was updated.</p>
    pub last_updated_at: std::option::Option<aws_smithy_types::Instant>,
}
impl ConnectionState {
    /// <p>The connection status of the tunnel. Valid values are <code>CONNECTED</code> and
    /// <code>DISCONNECTED</code>.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::ConnectionStatus> {
        self.status.as_ref()
    }
    /// <p>The last time the connection status was updated.</p>
    pub fn last_updated_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.last_updated_at.as_ref()
    }
}
impl std::fmt::Debug for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConnectionState");
        formatter.field("status", &self.status);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.finish()
    }
}
/// See [`ConnectionState`](crate::model::ConnectionState)
pub mod connection_state {
    /// A builder for [`ConnectionState`](crate::model::ConnectionState)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::ConnectionStatus>,
        pub(crate) last_updated_at: std::option::Option<aws_smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The connection status of the tunnel. Valid values are <code>CONNECTED</code> and
        /// <code>DISCONNECTED</code>.</p>
        pub fn status(mut self, input: crate::model::ConnectionStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The connection status of the tunnel. Valid values are <code>CONNECTED</code> and
        /// <code>DISCONNECTED</code>.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::ConnectionStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>The last time the connection status was updated.</p>
        pub fn last_updated_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>The last time the connection status was updated.</p>
        pub fn set_last_updated_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.last_updated_at = input;
            self
        }
        /// Consumes the builder and constructs a [`ConnectionState`](crate::model::ConnectionState)
        pub fn build(self) -> crate::model::ConnectionState {
            crate::model::ConnectionState {
                status: self.status,
                last_updated_at: self.last_updated_at,
            }
        }
    }
}
impl ConnectionState {
    /// Creates a new builder-style object to manufacture [`ConnectionState`](crate::model::ConnectionState)
    pub fn builder() -> crate::model::connection_state::Builder {
        crate::model::connection_state::Builder::default()
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
pub enum ConnectionStatus {
    #[allow(missing_docs)] // documentation missing in model
    Connected,
    #[allow(missing_docs)] // documentation missing in model
    Disconnected,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ConnectionStatus {
    fn from(s: &str) -> Self {
        match s {
            "CONNECTED" => ConnectionStatus::Connected,
            "DISCONNECTED" => ConnectionStatus::Disconnected,
            other => ConnectionStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ConnectionStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ConnectionStatus::from(s))
    }
}
impl ConnectionStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionStatus::Connected => "CONNECTED",
            ConnectionStatus::Disconnected => "DISCONNECTED",
            ConnectionStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CONNECTED", "DISCONNECTED"]
    }
}
impl AsRef<str> for ConnectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
