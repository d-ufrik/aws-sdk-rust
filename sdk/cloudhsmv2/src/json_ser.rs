// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_copy_backup_to_region_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CopyBackupToRegionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.destination_region {
        object.key("DestinationRegion").string(var_1);
    }
    if let Some(var_2) = &input.backup_id {
        object.key("BackupId").string(var_2);
    }
    if let Some(var_3) = &input.tag_list {
        let mut array_4 = object.key("TagList").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.backup_retention_policy {
        let mut object_8 = object.key("BackupRetentionPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_backup_retention_policy(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.hsm_type {
        object.key("HsmType").string(var_9);
    }
    if let Some(var_10) = &input.source_backup_id {
        object.key("SourceBackupId").string(var_10);
    }
    if let Some(var_11) = &input.subnet_ids {
        let mut array_12 = object.key("SubnetIds").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13);
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.tag_list {
        let mut array_15 = object.key("TagList").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.cluster_id {
        object.key("ClusterId").string(var_18);
    }
    if let Some(var_19) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_19);
    }
    if let Some(var_20) = &input.ip_address {
        object.key("IpAddress").string(var_20);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_backup_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBackupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.backup_id {
        object.key("BackupId").string(var_21);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.cluster_id {
        object.key("ClusterId").string(var_22);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.cluster_id {
        object.key("ClusterId").string(var_23);
    }
    if let Some(var_24) = &input.hsm_id {
        object.key("HsmId").string(var_24);
    }
    if let Some(var_25) = &input.eni_id {
        object.key("EniId").string(var_25);
    }
    if let Some(var_26) = &input.eni_ip {
        object.key("EniIp").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_backups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBackupsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27);
    }
    if let Some(var_28) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_28).into()),
        );
    }
    if let Some(var_29) = &input.filters {
        let mut object_30 = object.key("Filters").start_object();
        for (key_31, value_32) in var_29 {
            {
                let mut array_33 = object_30.key(key_31).start_array();
                for item_34 in value_32 {
                    {
                        array_33.value().string(item_34);
                    }
                }
                array_33.finish();
            }
        }
        object_30.finish();
    }
    if let Some(var_35) = &input.sort_ascending {
        object.key("SortAscending").boolean(*var_35);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_clusters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeClustersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.filters {
        let mut object_37 = object.key("Filters").start_object();
        for (key_38, value_39) in var_36 {
            {
                let mut array_40 = object_37.key(key_38).start_array();
                for item_41 in value_39 {
                    {
                        array_40.value().string(item_41);
                    }
                }
                array_40.finish();
            }
        }
        object_37.finish();
    }
    if let Some(var_42) = &input.next_token {
        object.key("NextToken").string(var_42);
    }
    if let Some(var_43) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_43).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_initialize_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::InitializeClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.cluster_id {
        object.key("ClusterId").string(var_44);
    }
    if let Some(var_45) = &input.signed_cert {
        object.key("SignedCert").string(var_45);
    }
    if let Some(var_46) = &input.trust_anchor {
        object.key("TrustAnchor").string(var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.resource_id {
        object.key("ResourceId").string(var_47);
    }
    if let Some(var_48) = &input.next_token {
        object.key("NextToken").string(var_48);
    }
    if let Some(var_49) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_modify_backup_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ModifyBackupAttributesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.backup_id {
        object.key("BackupId").string(var_50);
    }
    if let Some(var_51) = &input.never_expires {
        object.key("NeverExpires").boolean(*var_51);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_modify_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ModifyClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.backup_retention_policy {
        let mut object_53 = object.key("BackupRetentionPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_backup_retention_policy(
            &mut object_53,
            var_52,
        )?;
        object_53.finish();
    }
    if let Some(var_54) = &input.cluster_id {
        object.key("ClusterId").string(var_54);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_backup_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreBackupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.backup_id {
        object.key("BackupId").string(var_55);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.resource_id {
        object.key("ResourceId").string(var_56);
    }
    if let Some(var_57) = &input.tag_list {
        let mut array_58 = object.key("TagList").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_60, item_59)?;
                object_60.finish();
            }
        }
        array_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.resource_id {
        object.key("ResourceId").string(var_61);
    }
    if let Some(var_62) = &input.tag_key_list {
        let mut array_63 = object.key("TagKeyList").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.key {
        object.key("Key").string(var_65);
    }
    if let Some(var_66) = &input.value {
        object.key("Value").string(var_66);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_backup_retention_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackupRetentionPolicy,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.r#type {
        object.key("Type").string(var_67.as_str());
    }
    if let Some(var_68) = &input.value {
        object.key("Value").string(var_68);
    }
    Ok(())
}
