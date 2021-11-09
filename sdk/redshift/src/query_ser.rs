// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_delete_cluster_snapshot_message(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::DeleteClusterSnapshotMessage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SnapshotIdentifier");
    if let Some(var_2) = &input.snapshot_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SnapshotClusterIdentifier");
    if let Some(var_4) = &input.snapshot_cluster_identifier {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Key");
    if let Some(var_6) = &input.key {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Value");
    if let Some(var_8) = &input.value {
        scope_7.string(var_8);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_scheduled_action_type(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ScheduledActionType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ResizeCluster");
    if let Some(var_10) = &input.resize_cluster {
        crate::query_ser::serialize_structure_crate_model_resize_cluster_message(scope_9, var_10)?;
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("PauseCluster");
    if let Some(var_12) = &input.pause_cluster {
        crate::query_ser::serialize_structure_crate_model_pause_cluster_message(scope_11, var_12)?;
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("ResumeCluster");
    if let Some(var_14) = &input.resume_cluster {
        crate::query_ser::serialize_structure_crate_model_resume_cluster_message(scope_13, var_14)?;
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_snapshot_sorting_entity(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::SnapshotSortingEntity,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Attribute");
    if let Some(var_16) = &input.attribute {
        scope_15.string(var_16.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("SortOrder");
    if let Some(var_18) = &input.sort_order {
        scope_17.string(var_18.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_node_configuration_options_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::NodeConfigurationOptionsFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Name");
    if let Some(var_20) = &input.name {
        scope_19.string(var_20.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Operator");
    if let Some(var_22) = &input.operator {
        scope_21.string(var_22.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Value");
    if let Some(var_24) = &input.values {
        let mut list_26 = scope_23.start_list(false, Some("item"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            entry_27.string(item_25);
        }
        list_26.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_scheduled_action_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ScheduledActionFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("Name");
    if let Some(var_29) = &input.name {
        scope_28.string(var_29.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("Values");
    if let Some(var_31) = &input.values {
        let mut list_33 = scope_30.start_list(false, Some("item"));
        for item_32 in var_31 {
            #[allow(unused_mut)]
            let mut entry_34 = list_33.entry();
            entry_34.string(item_32);
        }
        list_33.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_parameter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Parameter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("ParameterName");
    if let Some(var_36) = &input.parameter_name {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("ParameterValue");
    if let Some(var_38) = &input.parameter_value {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Description");
    if let Some(var_40) = &input.description {
        scope_39.string(var_40);
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("Source");
    if let Some(var_42) = &input.source {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("DataType");
    if let Some(var_44) = &input.data_type {
        scope_43.string(var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("AllowedValues");
    if let Some(var_46) = &input.allowed_values {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("ApplyType");
    if let Some(var_48) = &input.apply_type {
        scope_47.string(var_48.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_49.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("MinimumEngineVersion");
    if let Some(var_51) = &input.minimum_engine_version {
        scope_50.string(var_51);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_resize_cluster_message(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ResizeClusterMessage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("ClusterIdentifier");
    if let Some(var_53) = &input.cluster_identifier {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("ClusterType");
    if let Some(var_55) = &input.cluster_type {
        scope_54.string(var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("NodeType");
    if let Some(var_57) = &input.node_type {
        scope_56.string(var_57);
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("NumberOfNodes");
    if let Some(var_59) = &input.number_of_nodes {
        scope_58.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("Classic");
    if let Some(var_61) = &input.classic {
        scope_60.boolean(*var_61);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_pause_cluster_message(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::PauseClusterMessage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("ClusterIdentifier");
    if let Some(var_63) = &input.cluster_identifier {
        scope_62.string(var_63);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_resume_cluster_message(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ResumeClusterMessage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("ClusterIdentifier");
    if let Some(var_65) = &input.cluster_identifier {
        scope_64.string(var_65);
    }
    Ok(())
}
