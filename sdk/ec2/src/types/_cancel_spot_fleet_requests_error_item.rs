// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Spot Fleet request that was not successfully canceled.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelSpotFleetRequestsErrorItem {
    /// <p>The error.</p>
    #[doc(hidden)]
    pub error: ::std::option::Option<crate::types::CancelSpotFleetRequestsError>,
    /// <p>The ID of the Spot Fleet request.</p>
    #[doc(hidden)]
    pub spot_fleet_request_id: ::std::option::Option<::std::string::String>,
}
impl CancelSpotFleetRequestsErrorItem {
    /// <p>The error.</p>
    pub fn error(&self) -> ::std::option::Option<&crate::types::CancelSpotFleetRequestsError> {
        self.error.as_ref()
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn spot_fleet_request_id(&self) -> ::std::option::Option<&str> {
        self.spot_fleet_request_id.as_deref()
    }
}
impl CancelSpotFleetRequestsErrorItem {
    /// Creates a new builder-style object to manufacture [`CancelSpotFleetRequestsErrorItem`](crate::types::CancelSpotFleetRequestsErrorItem).
    pub fn builder() -> crate::types::builders::CancelSpotFleetRequestsErrorItemBuilder {
        crate::types::builders::CancelSpotFleetRequestsErrorItemBuilder::default()
    }
}

/// A builder for [`CancelSpotFleetRequestsErrorItem`](crate::types::CancelSpotFleetRequestsErrorItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelSpotFleetRequestsErrorItemBuilder {
    pub(crate) error: ::std::option::Option<crate::types::CancelSpotFleetRequestsError>,
    pub(crate) spot_fleet_request_id: ::std::option::Option<::std::string::String>,
}
impl CancelSpotFleetRequestsErrorItemBuilder {
    /// <p>The error.</p>
    pub fn error(mut self, input: crate::types::CancelSpotFleetRequestsError) -> Self {
        self.error = ::std::option::Option::Some(input);
        self
    }
    /// <p>The error.</p>
    pub fn set_error(
        mut self,
        input: ::std::option::Option<crate::types::CancelSpotFleetRequestsError>,
    ) -> Self {
        self.error = input;
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn spot_fleet_request_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.spot_fleet_request_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn set_spot_fleet_request_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.spot_fleet_request_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotFleetRequestsErrorItem`](crate::types::CancelSpotFleetRequestsErrorItem).
    pub fn build(self) -> crate::types::CancelSpotFleetRequestsErrorItem {
        crate::types::CancelSpotFleetRequestsErrorItem {
            error: self.error,
            spot_fleet_request_id: self.spot_fleet_request_id,
        }
    }
}
