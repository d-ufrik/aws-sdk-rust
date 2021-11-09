// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_instance_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInstanceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.application_instance_id_to_replace {
        object.key("ApplicationInstanceIdToReplace").string(var_1);
    }
    if let Some(var_2) = &input.default_runtime_context_device {
        object.key("DefaultRuntimeContextDevice").string(var_2);
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3);
    }
    if let Some(var_4) = &input.manifest_overrides_payload {
        let mut object_5 = object.key("ManifestOverridesPayload").start_object();
        crate::json_ser::serialize_union_crate_model_manifest_overrides_payload(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.manifest_payload {
        let mut object_7 = object.key("ManifestPayload").start_object();
        crate::json_ser::serialize_union_crate_model_manifest_payload(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8);
    }
    if let Some(var_9) = &input.runtime_role_arn {
        object.key("RuntimeRoleArn").string(var_9);
    }
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("Tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_job_for_devices_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateJobForDevicesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.device_ids {
        let mut array_15 = object.key("DeviceIds").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16);
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.device_job_config {
        let mut object_18 = object.key("DeviceJobConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_device_job_config(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.job_type {
        object.key("JobType").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_node_from_template_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNodeFromTemplateJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.job_tags {
        let mut array_21 = object.key("JobTags").start_array();
        for item_22 in var_20 {
            {
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_job_resource_tags(
                    &mut object_23,
                    item_22,
                )?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.node_description {
        object.key("NodeDescription").string(var_24);
    }
    if let Some(var_25) = &input.node_name {
        object.key("NodeName").string(var_25);
    }
    if let Some(var_26) = &input.output_package_name {
        object.key("OutputPackageName").string(var_26);
    }
    if let Some(var_27) = &input.output_package_version {
        object.key("OutputPackageVersion").string(var_27);
    }
    if let Some(var_28) = &input.template_parameters {
        let mut object_29 = object.key("TemplateParameters").start_object();
        for (key_30, value_31) in var_28 {
            {
                object_29.key(key_30).string(value_31);
            }
        }
        object_29.finish();
    }
    if let Some(var_32) = &input.template_type {
        object.key("TemplateType").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_package_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePackageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.package_name {
        object.key("PackageName").string(var_33);
    }
    if let Some(var_34) = &input.tags {
        let mut object_35 = object.key("Tags").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36).string(value_37);
            }
        }
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_package_import_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePackageImportJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.client_token {
        object.key("ClientToken").string(var_38);
    }
    if let Some(var_39) = &input.input_config {
        let mut object_40 = object.key("InputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_package_import_job_input_config(
            &mut object_40,
            var_39,
        )?;
        object_40.finish();
    }
    if let Some(var_41) = &input.job_tags {
        let mut array_42 = object.key("JobTags").start_array();
        for item_43 in var_41 {
            {
                let mut object_44 = array_42.value().start_object();
                crate::json_ser::serialize_structure_crate_model_job_resource_tags(
                    &mut object_44,
                    item_43,
                )?;
                object_44.finish();
            }
        }
        array_42.finish();
    }
    if let Some(var_45) = &input.job_type {
        object.key("JobType").string(var_45.as_str());
    }
    if let Some(var_46) = &input.output_config {
        let mut object_47 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_package_import_job_output_config(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_provision_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ProvisionDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.description {
        object.key("Description").string(var_48);
    }
    if let Some(var_49) = &input.name {
        object.key("Name").string(var_49);
    }
    if let Some(var_50) = &input.networking_configuration {
        let mut object_51 = object.key("NetworkingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_payload(&mut object_51, var_50)?;
        object_51.finish();
    }
    if let Some(var_52) = &input.tags {
        let mut object_53 = object.key("Tags").start_object();
        for (key_54, value_55) in var_52 {
            {
                object_53.key(key_54).string(value_55);
            }
        }
        object_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_package_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterPackageVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.mark_latest {
        object.key("MarkLatest").boolean(*var_56);
    }
    if let Some(var_57) = &input.owner_account {
        object.key("OwnerAccount").string(var_57);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.tags {
        let mut object_59 = object.key("Tags").start_object();
        for (key_60, value_61) in var_58 {
            {
                object_59.key(key_60).string(value_61);
            }
        }
        object_59.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_device_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeviceMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.description {
        object.key("Description").string(var_62);
    }
    Ok(())
}

pub fn serialize_union_crate_model_manifest_overrides_payload(
    object_5: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestOverridesPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ManifestOverridesPayload::PayloadData(inner) => {
            object_5.key("PayloadData").string(inner);
        }
        crate::model::ManifestOverridesPayload::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "ManifestOverridesPayload",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_manifest_payload(
    object_7: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ManifestPayload::PayloadData(inner) => {
            object_7.key("PayloadData").string(inner);
        }
        crate::model::ManifestPayload::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant("ManifestPayload"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_job_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeviceJobConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.ota_job_config {
        let mut object_64 = object.key("OTAJobConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_ota_job_config(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_job_resource_tags(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobResourceTags,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.resource_type {
        object.key("ResourceType").string(var_65.as_str());
    }
    if let Some(var_66) = &input.tags {
        let mut object_67 = object.key("Tags").start_object();
        for (key_68, value_69) in var_66 {
            {
                object_67.key(key_68).string(value_69);
            }
        }
        object_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_import_job_input_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageImportJobInputConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.package_version_input_config {
        let mut object_71 = object.key("PackageVersionInputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_package_version_input_config(
            &mut object_71,
            var_70,
        )?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_import_job_output_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageImportJobOutputConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.package_version_output_config {
        let mut object_73 = object.key("PackageVersionOutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_package_version_output_config(
            &mut object_73,
            var_72,
        )?;
        object_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_payload(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.ethernet0 {
        let mut object_75 = object.key("Ethernet0").start_object();
        crate::json_ser::serialize_structure_crate_model_ethernet_payload(&mut object_75, var_74)?;
        object_75.finish();
    }
    if let Some(var_76) = &input.ethernet1 {
        let mut object_77 = object.key("Ethernet1").start_object();
        crate::json_ser::serialize_structure_crate_model_ethernet_payload(&mut object_77, var_76)?;
        object_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ota_job_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OtaJobConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.image_version {
        object.key("ImageVersion").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_version_input_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageVersionInputConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.s3_location {
        let mut object_80 = object.key("S3Location").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_location(&mut object_80, var_79)?;
        object_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_version_output_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageVersionOutputConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.package_name {
        object.key("PackageName").string(var_81);
    }
    if let Some(var_82) = &input.package_version {
        object.key("PackageVersion").string(var_82);
    }
    if let Some(var_83) = &input.mark_latest {
        object.key("MarkLatest").boolean(*var_83);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ethernet_payload(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EthernetPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.connection_type {
        object.key("ConnectionType").string(var_84.as_str());
    }
    if let Some(var_85) = &input.static_ip_connection_info {
        let mut object_86 = object.key("StaticIpConnectionInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_static_ip_connection_info(
            &mut object_86,
            var_85,
        )?;
        object_86.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Location,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.region {
        object.key("Region").string(var_87);
    }
    if let Some(var_88) = &input.bucket_name {
        object.key("BucketName").string(var_88);
    }
    if let Some(var_89) = &input.object_key {
        object.key("ObjectKey").string(var_89);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_static_ip_connection_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StaticIpConnectionInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.ip_address {
        object.key("IpAddress").string(var_90);
    }
    if let Some(var_91) = &input.mask {
        object.key("Mask").string(var_91);
    }
    if let Some(var_92) = &input.dns {
        let mut array_93 = object.key("Dns").start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94);
            }
        }
        array_93.finish();
    }
    if let Some(var_95) = &input.default_gateway {
        object.key("DefaultGateway").string(var_95);
    }
    Ok(())
}
