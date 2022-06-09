// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.connect_peer_id {
        object.key("ConnectPeerId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_id {
        object.key("DeviceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.link_id {
        object.key("LinkId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_customer_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateCustomerGatewayInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.customer_gateway_arn {
        object.key("CustomerGatewayArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.device_id {
        object.key("DeviceId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.link_id {
        object.key("LinkId").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateLinkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.device_id {
        object.key("DeviceId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.link_id {
        object.key("LinkId").string(var_8.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_transit_gateway_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTransitGatewayConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.device_id {
        object.key("DeviceId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.link_id {
        object.key("LinkId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.transit_gateway_connect_peer_arn {
        object
            .key("TransitGatewayConnectPeerArn")
            .string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connect_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectAttachmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.client_token {
        object.key("ClientToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.edge_location {
        object.key("EdgeLocation").string(var_14.as_str());
    }
    if let Some(var_15) = &input.options {
        let mut object_16 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_connect_attachment_options(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut array_18 = object.key("Tags").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.transport_attachment_id {
        object.key("TransportAttachmentId").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.connected_device_id {
        object.key("ConnectedDeviceId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_23.as_str());
    }
    if let Some(var_24) = &input.description {
        object.key("Description").string(var_24.as_str());
    }
    if let Some(var_25) = &input.device_id {
        object.key("DeviceId").string(var_25.as_str());
    }
    if let Some(var_26) = &input.link_id {
        object.key("LinkId").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("Tags").start_array();
        for item_29 in var_27 {
            {
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.bgp_options {
        let mut object_32 = object.key("BgpOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_bgp_options(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.client_token {
        object.key("ClientToken").string(var_33.as_str());
    }
    if let Some(var_34) = &input.connect_attachment_id {
        object.key("ConnectAttachmentId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.core_network_address {
        object.key("CoreNetworkAddress").string(var_35.as_str());
    }
    if let Some(var_36) = &input.inside_cidr_blocks {
        let mut array_37 = object.key("InsideCidrBlocks").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.peer_address {
        object.key("PeerAddress").string(var_39.as_str());
    }
    if let Some(var_40) = &input.tags {
        let mut array_41 = object.key("Tags").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_core_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCoreNetworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.client_token {
        object.key("ClientToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.description {
        object.key("Description").string(var_45.as_str());
    }
    if let Some(var_46) = &input.global_network_id {
        object.key("GlobalNetworkId").string(var_46.as_str());
    }
    if let Some(var_47) = &input.policy_document {
        object.key("PolicyDocument").string(var_47.as_str());
    }
    if let Some(var_48) = &input.tags {
        let mut array_49 = object.key("Tags").start_array();
        for item_50 in var_48 {
            {
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_51, item_50)?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.aws_location {
        let mut object_53 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_53, var_52)?;
        object_53.finish();
    }
    if let Some(var_54) = &input.description {
        object.key("Description").string(var_54.as_str());
    }
    if let Some(var_55) = &input.location {
        let mut object_56 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_56, var_55)?;
        object_56.finish();
    }
    if let Some(var_57) = &input.model {
        object.key("Model").string(var_57.as_str());
    }
    if let Some(var_58) = &input.serial_number {
        object.key("SerialNumber").string(var_58.as_str());
    }
    if let Some(var_59) = &input.site_id {
        object.key("SiteId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.tags {
        let mut array_61 = object.key("Tags").start_array();
        for item_62 in var_60 {
            {
                let mut object_63 = array_61.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_63, item_62)?;
                object_63.finish();
            }
        }
        array_61.finish();
    }
    if let Some(var_64) = &input.r#type {
        object.key("Type").string(var_64.as_str());
    }
    if let Some(var_65) = &input.vendor {
        object.key("Vendor").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGlobalNetworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.description {
        object.key("Description").string(var_66.as_str());
    }
    if let Some(var_67) = &input.tags {
        let mut array_68 = object.key("Tags").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_70, item_69)?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLinkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.bandwidth {
        let mut object_72 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_72, var_71)?;
        object_72.finish();
    }
    if let Some(var_73) = &input.description {
        object.key("Description").string(var_73.as_str());
    }
    if let Some(var_74) = &input.provider {
        object.key("Provider").string(var_74.as_str());
    }
    if let Some(var_75) = &input.site_id {
        object.key("SiteId").string(var_75.as_str());
    }
    if let Some(var_76) = &input.tags {
        let mut array_77 = object.key("Tags").start_array();
        for item_78 in var_76 {
            {
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_79, item_78)?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    if let Some(var_80) = &input.r#type {
        object.key("Type").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSiteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.description {
        object.key("Description").string(var_81.as_str());
    }
    if let Some(var_82) = &input.location {
        let mut object_83 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_83, var_82)?;
        object_83.finish();
    }
    if let Some(var_84) = &input.tags {
        let mut array_85 = object.key("Tags").start_array();
        for item_86 in var_84 {
            {
                let mut object_87 = array_85.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_87, item_86)?;
                object_87.finish();
            }
        }
        array_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_site_to_site_vpn_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSiteToSiteVpnAttachmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.client_token {
        object.key("ClientToken").string(var_88.as_str());
    }
    if let Some(var_89) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_89.as_str());
    }
    if let Some(var_90) = &input.tags {
        let mut array_91 = object.key("Tags").start_array();
        for item_92 in var_90 {
            {
                let mut object_93 = array_91.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_93, item_92)?;
                object_93.finish();
            }
        }
        array_91.finish();
    }
    if let Some(var_94) = &input.vpn_connection_arn {
        object.key("VpnConnectionArn").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vpc_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVpcAttachmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.client_token {
        object.key("ClientToken").string(var_95.as_str());
    }
    if let Some(var_96) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_96.as_str());
    }
    if let Some(var_97) = &input.options {
        let mut object_98 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_98, var_97)?;
        object_98.finish();
    }
    if let Some(var_99) = &input.subnet_arns {
        let mut array_100 = object.key("SubnetArns").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101.as_str());
            }
        }
        array_100.finish();
    }
    if let Some(var_102) = &input.tags {
        let mut array_103 = object.key("Tags").start_array();
        for item_104 in var_102 {
            {
                let mut object_105 = array_103.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_105, item_104)?;
                object_105.finish();
            }
        }
        array_103.finish();
    }
    if let Some(var_106) = &input.vpc_arn {
        object.key("VpcArn").string(var_106.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_network_routes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetNetworkRoutesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.destination_filters {
        let mut object_108 = object.key("DestinationFilters").start_object();
        for (key_109, value_110) in var_107 {
            {
                let mut array_111 = object_108.key(key_109).start_array();
                for item_112 in value_110 {
                    {
                        array_111.value().string(item_112.as_str());
                    }
                }
                array_111.finish();
            }
        }
        object_108.finish();
    }
    if let Some(var_113) = &input.exact_cidr_matches {
        let mut array_114 = object.key("ExactCidrMatches").start_array();
        for item_115 in var_113 {
            {
                array_114.value().string(item_115.as_str());
            }
        }
        array_114.finish();
    }
    if let Some(var_116) = &input.longest_prefix_matches {
        let mut array_117 = object.key("LongestPrefixMatches").start_array();
        for item_118 in var_116 {
            {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    if let Some(var_119) = &input.prefix_list_ids {
        let mut array_120 = object.key("PrefixListIds").start_array();
        for item_121 in var_119 {
            {
                array_120.value().string(item_121.as_str());
            }
        }
        array_120.finish();
    }
    if let Some(var_122) = &input.route_table_identifier {
        let mut object_123 = object.key("RouteTableIdentifier").start_object();
        crate::json_ser::serialize_structure_crate_model_route_table_identifier(
            &mut object_123,
            var_122,
        )?;
        object_123.finish();
    }
    if let Some(var_124) = &input.states {
        let mut array_125 = object.key("States").start_array();
        for item_126 in var_124 {
            {
                array_125.value().string(item_126.as_str());
            }
        }
        array_125.finish();
    }
    if let Some(var_127) = &input.subnet_of_matches {
        let mut array_128 = object.key("SubnetOfMatches").start_array();
        for item_129 in var_127 {
            {
                array_128.value().string(item_129.as_str());
            }
        }
        array_128.finish();
    }
    if let Some(var_130) = &input.supernet_of_matches {
        let mut array_131 = object.key("SupernetOfMatches").start_array();
        for item_132 in var_130 {
            {
                array_131.value().string(item_132.as_str());
            }
        }
        array_131.finish();
    }
    if let Some(var_133) = &input.types {
        let mut array_134 = object.key("Types").start_array();
        for item_135 in var_133 {
            {
                array_134.value().string(item_135.as_str());
            }
        }
        array_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_core_network_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutCoreNetworkPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.client_token {
        object.key("ClientToken").string(var_136.as_str());
    }
    if let Some(var_137) = &input.description {
        object.key("Description").string(var_137.as_str());
    }
    if let Some(var_138) = &input.latest_version_id {
        object.key("LatestVersionId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_138).into()),
        );
    }
    if let Some(var_139) = &input.policy_document {
        object.key("PolicyDocument").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.policy_document {
        object.key("PolicyDocument").string(var_140.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_transit_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterTransitGatewayInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.transit_gateway_arn {
        object.key("TransitGatewayArn").string(var_141.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_organization_service_access_update_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartOrganizationServiceAccessUpdateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_142) = &input.action {
        object.key("Action").string(var_142.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_route_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartRouteAnalysisInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_143) = &input.destination {
        let mut object_144 = object.key("Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_144, var_143)?;
        object_144.finish();
    }
    if input.include_return_path {
        object
            .key("IncludeReturnPath")
            .boolean(input.include_return_path);
    }
    if let Some(var_145) = &input.source {
        let mut object_146 = object.key("Source").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_146, var_145)?;
        object_146.finish();
    }
    if input.use_middleboxes {
        object.key("UseMiddleboxes").boolean(input.use_middleboxes);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.tags {
        let mut array_148 = object.key("Tags").start_array();
        for item_149 in var_147 {
            {
                let mut object_150 = array_148.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_150, item_149)?;
                object_150.finish();
            }
        }
        array_148.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConnectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_151.as_str());
    }
    if let Some(var_152) = &input.description {
        object.key("Description").string(var_152.as_str());
    }
    if let Some(var_153) = &input.link_id {
        object.key("LinkId").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_core_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCoreNetworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_154) = &input.description {
        object.key("Description").string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_155) = &input.aws_location {
        let mut object_156 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_156, var_155)?;
        object_156.finish();
    }
    if let Some(var_157) = &input.description {
        object.key("Description").string(var_157.as_str());
    }
    if let Some(var_158) = &input.location {
        let mut object_159 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_159, var_158)?;
        object_159.finish();
    }
    if let Some(var_160) = &input.model {
        object.key("Model").string(var_160.as_str());
    }
    if let Some(var_161) = &input.serial_number {
        object.key("SerialNumber").string(var_161.as_str());
    }
    if let Some(var_162) = &input.site_id {
        object.key("SiteId").string(var_162.as_str());
    }
    if let Some(var_163) = &input.r#type {
        object.key("Type").string(var_163.as_str());
    }
    if let Some(var_164) = &input.vendor {
        object.key("Vendor").string(var_164.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGlobalNetworkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_165) = &input.description {
        object.key("Description").string(var_165.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLinkInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_166) = &input.bandwidth {
        let mut object_167 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_167, var_166)?;
        object_167.finish();
    }
    if let Some(var_168) = &input.description {
        object.key("Description").string(var_168.as_str());
    }
    if let Some(var_169) = &input.provider {
        object.key("Provider").string(var_169.as_str());
    }
    if let Some(var_170) = &input.r#type {
        object.key("Type").string(var_170.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_network_resource_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNetworkResourceMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_171) = &input.metadata {
        let mut object_172 = object.key("Metadata").start_object();
        for (key_173, value_174) in var_171 {
            {
                object_172.key(key_173).string(value_174.as_str());
            }
        }
        object_172.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSiteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_175) = &input.description {
        object.key("Description").string(var_175.as_str());
    }
    if let Some(var_176) = &input.location {
        let mut object_177 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_177, var_176)?;
        object_177.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_vpc_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVpcAttachmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_178) = &input.add_subnet_arns {
        let mut array_179 = object.key("AddSubnetArns").start_array();
        for item_180 in var_178 {
            {
                array_179.value().string(item_180.as_str());
            }
        }
        array_179.finish();
    }
    if let Some(var_181) = &input.options {
        let mut object_182 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_182, var_181)?;
        object_182.finish();
    }
    if let Some(var_183) = &input.remove_subnet_arns {
        let mut array_184 = object.key("RemoveSubnetArns").start_array();
        for item_185 in var_183 {
            {
                array_184.value().string(item_185.as_str());
            }
        }
        array_184.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_connect_attachment_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectAttachmentOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_186) = &input.protocol {
        object.key("Protocol").string(var_186.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_187) = &input.key {
        object.key("Key").string(var_187.as_str());
    }
    if let Some(var_188) = &input.value {
        object.key("Value").string(var_188.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bgp_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BgpOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_189) = &input.peer_asn {
        object.key("PeerAsn").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_189).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_aws_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsLocation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_190) = &input.zone {
        object.key("Zone").string(var_190.as_str());
    }
    if let Some(var_191) = &input.subnet_arn {
        object.key("SubnetArn").string(var_191.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Location,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_192) = &input.address {
        object.key("Address").string(var_192.as_str());
    }
    if let Some(var_193) = &input.latitude {
        object.key("Latitude").string(var_193.as_str());
    }
    if let Some(var_194) = &input.longitude {
        object.key("Longitude").string(var_194.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bandwidth(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bandwidth,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_195) = &input.upload_speed {
        object.key("UploadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_195).into()),
        );
    }
    if let Some(var_196) = &input.download_speed {
        object.key("DownloadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_196).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.ipv6_support {
        object.key("Ipv6Support").boolean(input.ipv6_support);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_route_table_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteTableIdentifier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_197) = &input.transit_gateway_route_table_arn {
        object
            .key("TransitGatewayRouteTableArn")
            .string(var_197.as_str());
    }
    if let Some(var_198) = &input.core_network_segment_edge {
        let mut object_199 = object.key("CoreNetworkSegmentEdge").start_object();
        crate::json_ser::serialize_structure_crate_model_core_network_segment_edge_identifier(
            &mut object_199,
            var_198,
        )?;
        object_199.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_route_analysis_endpoint_options_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteAnalysisEndpointOptionsSpecification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_200) = &input.transit_gateway_attachment_arn {
        object
            .key("TransitGatewayAttachmentArn")
            .string(var_200.as_str());
    }
    if let Some(var_201) = &input.ip_address {
        object.key("IpAddress").string(var_201.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_core_network_segment_edge_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CoreNetworkSegmentEdgeIdentifier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_202) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_202.as_str());
    }
    if let Some(var_203) = &input.segment_name {
        object.key("SegmentName").string(var_203.as_str());
    }
    if let Some(var_204) = &input.edge_location {
        object.key("EdgeLocation").string(var_204.as_str());
    }
    Ok(())
}
