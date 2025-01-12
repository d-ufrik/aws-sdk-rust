// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ActivateNetworkSite`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_site_arn(impl ::std::convert::Into<String>)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::network_site_arn) / [`set_network_site_arn(Option<String>)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::set_network_site_arn): <p>The Amazon Resource Name (ARN) of the network site.</p>
    ///   - [`shipping_address(Address)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::shipping_address) / [`set_shipping_address(Option<Address>)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::set_shipping_address): <p>The shipping address of the network site.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// - On success, responds with [`ActivateNetworkSiteOutput`](crate::operation::activate_network_site::ActivateNetworkSiteOutput) with field(s):
    ///   - [`network_site(Option<NetworkSite>)`](crate::operation::activate_network_site::ActivateNetworkSiteOutput::network_site): <p>Information about the network site.</p>
    /// - On failure, responds with [`SdkError<ActivateNetworkSiteError>`](crate::operation::activate_network_site::ActivateNetworkSiteError)
    pub fn activate_network_site(
        &self,
    ) -> crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder {
        crate::operation::activate_network_site::builders::ActivateNetworkSiteFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
