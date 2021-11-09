// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_1);
    }
    if let Some(var_2) = &input.ops_center_enabled {
        object.key("OpsCenterEnabled").boolean(*var_2);
    }
    if let Some(var_3) = &input.cwe_monitor_enabled {
        object.key("CWEMonitorEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.ops_item_sns_topic_arn {
        object.key("OpsItemSNSTopicArn").string(var_4);
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
            {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.auto_config_enabled {
        object.key("AutoConfigEnabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.auto_create {
        object.key("AutoCreate").boolean(*var_10);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateComponentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_11);
    }
    if let Some(var_12) = &input.component_name {
        object.key("ComponentName").string(var_12);
    }
    if let Some(var_13) = &input.resource_list {
        let mut array_14 = object.key("ResourceList").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15);
            }
        }
        array_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_log_pattern_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLogPatternInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_16) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_16);
    }
    if let Some(var_17) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_17);
    }
    if let Some(var_18) = &input.pattern_name {
        object.key("PatternName").string(var_18);
    }
    if let Some(var_19) = &input.pattern {
        object.key("Pattern").string(var_19);
    }
    {
        object.key("Rank").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rank).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_20);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteComponentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_21);
    }
    if let Some(var_22) = &input.component_name {
        object.key("ComponentName").string(var_22);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_log_pattern_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLogPatternInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_23);
    }
    if let Some(var_24) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_24);
    }
    if let Some(var_25) = &input.pattern_name {
        object.key("PatternName").string(var_25);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeComponentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_27);
    }
    if let Some(var_28) = &input.component_name {
        object.key("ComponentName").string(var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_component_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeComponentConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_29);
    }
    if let Some(var_30) = &input.component_name {
        object.key("ComponentName").string(var_30);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_component_configuration_recommendation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeComponentConfigurationRecommendationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_31);
    }
    if let Some(var_32) = &input.component_name {
        object.key("ComponentName").string(var_32);
    }
    if let Some(var_33) = &input.tier {
        object.key("Tier").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_log_pattern_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLogPatternInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_34);
    }
    if let Some(var_35) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_35);
    }
    if let Some(var_36) = &input.pattern_name {
        object.key("PatternName").string(var_36);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_observation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeObservationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.observation_id {
        object.key("ObservationId").string(var_37);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_problem_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeProblemInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.problem_id {
        object.key("ProblemId").string(var_38);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_problem_observations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeProblemObservationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.problem_id {
        object.key("ProblemId").string(var_39);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_applications_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListApplicationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_40).into()),
        );
    }
    if let Some(var_41) = &input.next_token {
        object.key("NextToken").string(var_41);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_components_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListComponentsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_42);
    }
    if let Some(var_43) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_43).into()),
        );
    }
    if let Some(var_44) = &input.next_token {
        object.key("NextToken").string(var_44);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_configuration_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListConfigurationHistoryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_45);
    }
    if let Some(var_46) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_46, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_47) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_47, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_48) = &input.event_status {
        object.key("EventStatus").string(var_48.as_str());
    }
    if let Some(var_49) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.next_token {
        object.key("NextToken").string(var_50);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_log_patterns_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLogPatternsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_51);
    }
    if let Some(var_52) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_52);
    }
    if let Some(var_53) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_53).into()),
        );
    }
    if let Some(var_54) = &input.next_token {
        object.key("NextToken").string(var_54);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_log_pattern_sets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLogPatternSetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_55);
    }
    if let Some(var_56) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    if let Some(var_57) = &input.next_token {
        object.key("NextToken").string(var_57);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_problems_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListProblemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_58);
    }
    if let Some(var_59) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_59, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_60) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_60, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_61) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.next_token {
        object.key("NextToken").string(var_62);
    }
    if let Some(var_63) = &input.component_name {
        object.key("ComponentName").string(var_63);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.resource_arn {
        object.key("ResourceARN").string(var_64);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.resource_arn {
        object.key("ResourceARN").string(var_65);
    }
    if let Some(var_66) = &input.tags {
        let mut array_67 = object.key("Tags").start_array();
        for item_68 in var_66 {
            {
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_69, item_68)?;
                object_69.finish();
            }
        }
        array_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.resource_arn {
        object.key("ResourceARN").string(var_70);
    }
    if let Some(var_71) = &input.tag_keys {
        let mut array_72 = object.key("TagKeys").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73);
            }
        }
        array_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_74);
    }
    if let Some(var_75) = &input.ops_center_enabled {
        object.key("OpsCenterEnabled").boolean(*var_75);
    }
    if let Some(var_76) = &input.cwe_monitor_enabled {
        object.key("CWEMonitorEnabled").boolean(*var_76);
    }
    if let Some(var_77) = &input.ops_item_sns_topic_arn {
        object.key("OpsItemSNSTopicArn").string(var_77);
    }
    if let Some(var_78) = &input.remove_sns_topic {
        object.key("RemoveSNSTopic").boolean(*var_78);
    }
    if let Some(var_79) = &input.auto_config_enabled {
        object.key("AutoConfigEnabled").boolean(*var_79);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateComponentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_80);
    }
    if let Some(var_81) = &input.component_name {
        object.key("ComponentName").string(var_81);
    }
    if let Some(var_82) = &input.new_component_name {
        object.key("NewComponentName").string(var_82);
    }
    if let Some(var_83) = &input.resource_list {
        let mut array_84 = object.key("ResourceList").start_array();
        for item_85 in var_83 {
            {
                array_84.value().string(item_85);
            }
        }
        array_84.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_component_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateComponentConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_86);
    }
    if let Some(var_87) = &input.component_name {
        object.key("ComponentName").string(var_87);
    }
    if let Some(var_88) = &input.monitor {
        object.key("Monitor").boolean(*var_88);
    }
    if let Some(var_89) = &input.tier {
        object.key("Tier").string(var_89.as_str());
    }
    if let Some(var_90) = &input.component_configuration {
        object.key("ComponentConfiguration").string(var_90);
    }
    if let Some(var_91) = &input.auto_config_enabled {
        object.key("AutoConfigEnabled").boolean(*var_91);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_log_pattern_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLogPatternInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_92);
    }
    if let Some(var_93) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_93);
    }
    if let Some(var_94) = &input.pattern_name {
        object.key("PatternName").string(var_94);
    }
    if let Some(var_95) = &input.pattern {
        object.key("Pattern").string(var_95);
    }
    if input.rank != 0 {
        object.key("Rank").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rank).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_96) = &input.key {
        object.key("Key").string(var_96);
    }
    if let Some(var_97) = &input.value {
        object.key("Value").string(var_97);
    }
    Ok(())
}
