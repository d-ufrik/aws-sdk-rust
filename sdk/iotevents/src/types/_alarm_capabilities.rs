// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the configuration information of alarm state changes.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AlarmCapabilities {
    /// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
    #[doc(hidden)]
    pub initialization_configuration:
        ::std::option::Option<crate::types::InitializationConfiguration>,
    /// <p>Specifies whether to get notified for alarm state changes.</p>
    #[doc(hidden)]
    pub acknowledge_flow: ::std::option::Option<crate::types::AcknowledgeFlow>,
}
impl AlarmCapabilities {
    /// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
    pub fn initialization_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::InitializationConfiguration> {
        self.initialization_configuration.as_ref()
    }
    /// <p>Specifies whether to get notified for alarm state changes.</p>
    pub fn acknowledge_flow(&self) -> ::std::option::Option<&crate::types::AcknowledgeFlow> {
        self.acknowledge_flow.as_ref()
    }
}
impl AlarmCapabilities {
    /// Creates a new builder-style object to manufacture [`AlarmCapabilities`](crate::types::AlarmCapabilities).
    pub fn builder() -> crate::types::builders::AlarmCapabilitiesBuilder {
        crate::types::builders::AlarmCapabilitiesBuilder::default()
    }
}

/// A builder for [`AlarmCapabilities`](crate::types::AlarmCapabilities).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AlarmCapabilitiesBuilder {
    pub(crate) initialization_configuration:
        ::std::option::Option<crate::types::InitializationConfiguration>,
    pub(crate) acknowledge_flow: ::std::option::Option<crate::types::AcknowledgeFlow>,
}
impl AlarmCapabilitiesBuilder {
    /// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
    pub fn initialization_configuration(
        mut self,
        input: crate::types::InitializationConfiguration,
    ) -> Self {
        self.initialization_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
    pub fn set_initialization_configuration(
        mut self,
        input: ::std::option::Option<crate::types::InitializationConfiguration>,
    ) -> Self {
        self.initialization_configuration = input;
        self
    }
    /// <p>Specifies whether to get notified for alarm state changes.</p>
    pub fn acknowledge_flow(mut self, input: crate::types::AcknowledgeFlow) -> Self {
        self.acknowledge_flow = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether to get notified for alarm state changes.</p>
    pub fn set_acknowledge_flow(
        mut self,
        input: ::std::option::Option<crate::types::AcknowledgeFlow>,
    ) -> Self {
        self.acknowledge_flow = input;
        self
    }
    /// Consumes the builder and constructs a [`AlarmCapabilities`](crate::types::AlarmCapabilities).
    pub fn build(self) -> crate::types::AlarmCapabilities {
        crate::types::AlarmCapabilities {
            initialization_configuration: self.initialization_configuration,
            acknowledge_flow: self.acknowledge_flow,
        }
    }
}
