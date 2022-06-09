// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_draft_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddDraftAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.app_arn {
        object.key("appArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_mappings {
        let mut array_3 = object.key("resourceMappings").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_mapping(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.assessment_schedule {
        object.key("assessmentSchedule").string(var_6.as_str());
    }
    if let Some(var_7) = &input.client_token {
        object.key("clientToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.description {
        object.key("description").string(var_8.as_str());
    }
    if let Some(var_9) = &input.name {
        object.key("name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.policy_arn {
        object.key("policyArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_recommendation_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRecommendationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.assessment_arn {
        object.key("assessmentArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.bucket_name {
        object.key("bucketName").string(var_16.as_str());
    }
    if let Some(var_17) = &input.client_token {
        object.key("clientToken").string(var_17.as_str());
    }
    if let Some(var_18) = &input.format {
        object.key("format").string(var_18.as_str());
    }
    if let Some(var_19) = &input.name {
        object.key("name").string(var_19.as_str());
    }
    if let Some(var_20) = &input.recommendation_ids {
        let mut array_21 = object.key("recommendationIds").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.recommendation_types {
        let mut array_24 = object.key("recommendationTypes").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.tags {
        let mut object_27 = object.key("tags").start_object();
        for (key_28, value_29) in var_26 {
            {
                object_27.key(key_28).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.client_token {
        object.key("clientToken").string(var_30.as_str());
    }
    if let Some(var_31) = &input.data_location_constraint {
        object.key("dataLocationConstraint").string(var_31.as_str());
    }
    if let Some(var_32) = &input.policy {
        let mut object_33 = object.key("policy").start_object();
        for (key_34, value_35) in var_32 {
            {
                let mut object_36 = object_33.key(key_34.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_failure_policy(
                    &mut object_36,
                    value_35,
                )?;
                object_36.finish();
            }
        }
        object_33.finish();
    }
    if let Some(var_37) = &input.policy_description {
        object.key("policyDescription").string(var_37.as_str());
    }
    if let Some(var_38) = &input.policy_name {
        object.key("policyName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.tags {
        let mut object_40 = object.key("tags").start_object();
        for (key_41, value_42) in var_39 {
            {
                object_40.key(key_41).string(value_42.as_str());
            }
        }
        object_40.finish();
    }
    if let Some(var_43) = &input.tier {
        object.key("tier").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.app_arn {
        object.key("appArn").string(var_44.as_str());
    }
    if let Some(var_45) = &input.client_token {
        object.key("clientToken").string(var_45.as_str());
    }
    if let Some(var_46) = &input.force_delete {
        object.key("forceDelete").boolean(*var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.assessment_arn {
        object.key("assessmentArn").string(var_47.as_str());
    }
    if let Some(var_48) = &input.client_token {
        object.key("clientToken").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_recommendation_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRecommendationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.client_token {
        object.key("clientToken").string(var_49.as_str());
    }
    if let Some(var_50) = &input.recommendation_template_arn {
        object
            .key("recommendationTemplateArn")
            .string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.client_token {
        object.key("clientToken").string(var_51.as_str());
    }
    if let Some(var_52) = &input.policy_arn {
        object.key("policyArn").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.app_arn {
        object.key("appArn").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.assessment_arn {
        object.key("assessmentArn").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_version_resources_resolution_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppVersionResourcesResolutionStatusInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.app_arn {
        object.key("appArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.app_version {
        object.key("appVersion").string(var_56.as_str());
    }
    if let Some(var_57) = &input.resolution_id {
        object.key("resolutionId").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_version_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppVersionTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.app_arn {
        object.key("appArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.app_version {
        object.key("appVersion").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_draft_app_version_resources_import_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDraftAppVersionResourcesImportStatusInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.app_arn {
        object.key("appArn").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.policy_arn {
        object.key("policyArn").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_resources_to_draft_app_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportResourcesToDraftAppVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.app_arn {
        object.key("appArn").string(var_62.as_str());
    }
    if let Some(var_63) = &input.source_arns {
        let mut array_64 = object.key("sourceArns").start_array();
        for item_65 in var_63 {
            {
                array_64.value().string(item_65.as_str());
            }
        }
        array_64.finish();
    }
    if let Some(var_66) = &input.terraform_sources {
        let mut array_67 = object.key("terraformSources").start_array();
        for item_68 in var_66 {
            {
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_crate_model_terraform_source(
                    &mut object_69,
                    item_68,
                )?;
                object_69.finish();
            }
        }
        array_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_alarm_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAlarmRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.assessment_arn {
        object.key("assessmentArn").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    if let Some(var_72) = &input.next_token {
        object.key("nextToken").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_component_compliances_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppComponentCompliancesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.assessment_arn {
        object.key("assessmentArn").string(var_73.as_str());
    }
    if let Some(var_74) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_74).into()),
        );
    }
    if let Some(var_75) = &input.next_token {
        object.key("nextToken").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_component_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppComponentRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.assessment_arn {
        object.key("assessmentArn").string(var_76.as_str());
    }
    if let Some(var_77) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    if let Some(var_78) = &input.next_token {
        object.key("nextToken").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.app_arn {
        object.key("appArn").string(var_79.as_str());
    }
    if let Some(var_80) = &input.app_version {
        object.key("appVersion").string(var_80.as_str());
    }
    if let Some(var_81) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_81).into()),
        );
    }
    if let Some(var_82) = &input.next_token {
        object.key("nextToken").string(var_82.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.app_arn {
        object.key("appArn").string(var_83.as_str());
    }
    if let Some(var_84) = &input.app_version {
        object.key("appVersion").string(var_84.as_str());
    }
    if let Some(var_85) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_85).into()),
        );
    }
    if let Some(var_86) = &input.next_token {
        object.key("nextToken").string(var_86.as_str());
    }
    if let Some(var_87) = &input.resolution_id {
        object.key("resolutionId").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.app_arn {
        object.key("appArn").string(var_88.as_str());
    }
    if let Some(var_89) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_89).into()),
        );
    }
    if let Some(var_90) = &input.next_token {
        object.key("nextToken").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_sop_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSopRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.assessment_arn {
        object.key("assessmentArn").string(var_91.as_str());
    }
    if let Some(var_92) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_92).into()),
        );
    }
    if let Some(var_93) = &input.next_token {
        object.key("nextToken").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_test_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTestRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.assessment_arn {
        object.key("assessmentArn").string(var_94.as_str());
    }
    if let Some(var_95) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_95).into()),
        );
    }
    if let Some(var_96) = &input.next_token {
        object.key("nextToken").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_unsupported_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListUnsupportedAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.app_arn {
        object.key("appArn").string(var_97.as_str());
    }
    if let Some(var_98) = &input.app_version {
        object.key("appVersion").string(var_98.as_str());
    }
    if let Some(var_99) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_99).into()),
        );
    }
    if let Some(var_100) = &input.next_token {
        object.key("nextToken").string(var_100.as_str());
    }
    if let Some(var_101) = &input.resolution_id {
        object.key("resolutionId").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_publish_app_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishAppVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.app_arn {
        object.key("appArn").string(var_102.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_draft_app_version_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDraftAppVersionTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.app_arn {
        object.key("appArn").string(var_103.as_str());
    }
    if let Some(var_104) = &input.app_template_body {
        object.key("appTemplateBody").string(var_104.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_draft_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveDraftAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.app_arn {
        object.key("appArn").string(var_105.as_str());
    }
    if let Some(var_106) = &input.app_registry_app_names {
        let mut array_107 = object.key("appRegistryAppNames").start_array();
        for item_108 in var_106 {
            {
                array_107.value().string(item_108.as_str());
            }
        }
        array_107.finish();
    }
    if let Some(var_109) = &input.logical_stack_names {
        let mut array_110 = object.key("logicalStackNames").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111.as_str());
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.resource_group_names {
        let mut array_113 = object.key("resourceGroupNames").start_array();
        for item_114 in var_112 {
            {
                array_113.value().string(item_114.as_str());
            }
        }
        array_113.finish();
    }
    if let Some(var_115) = &input.resource_names {
        let mut array_116 = object.key("resourceNames").start_array();
        for item_117 in var_115 {
            {
                array_116.value().string(item_117.as_str());
            }
        }
        array_116.finish();
    }
    if let Some(var_118) = &input.terraform_source_names {
        let mut array_119 = object.key("terraformSourceNames").start_array();
        for item_120 in var_118 {
            {
                array_119.value().string(item_120.as_str());
            }
        }
        array_119.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resolve_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResolveAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.app_arn {
        object.key("appArn").string(var_121.as_str());
    }
    if let Some(var_122) = &input.app_version {
        object.key("appVersion").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.app_arn {
        object.key("appArn").string(var_123.as_str());
    }
    if let Some(var_124) = &input.app_version {
        object.key("appVersion").string(var_124.as_str());
    }
    if let Some(var_125) = &input.assessment_name {
        object.key("assessmentName").string(var_125.as_str());
    }
    if let Some(var_126) = &input.client_token {
        object.key("clientToken").string(var_126.as_str());
    }
    if let Some(var_127) = &input.tags {
        let mut object_128 = object.key("tags").start_object();
        for (key_129, value_130) in var_127 {
            {
                object_128.key(key_129).string(value_130.as_str());
            }
        }
        object_128.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.tags {
        let mut object_132 = object.key("tags").start_object();
        for (key_133, value_134) in var_131 {
            {
                object_132.key(key_133).string(value_134.as_str());
            }
        }
        object_132.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_135) = &input.app_arn {
        object.key("appArn").string(var_135.as_str());
    }
    if let Some(var_136) = &input.assessment_schedule {
        object.key("assessmentSchedule").string(var_136.as_str());
    }
    if let Some(var_137) = &input.clear_resiliency_policy_arn {
        object.key("clearResiliencyPolicyArn").boolean(*var_137);
    }
    if let Some(var_138) = &input.description {
        object.key("description").string(var_138.as_str());
    }
    if let Some(var_139) = &input.policy_arn {
        object.key("policyArn").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.data_location_constraint {
        object
            .key("dataLocationConstraint")
            .string(var_140.as_str());
    }
    if let Some(var_141) = &input.policy {
        let mut object_142 = object.key("policy").start_object();
        for (key_143, value_144) in var_141 {
            {
                let mut object_145 = object_142.key(key_143.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_failure_policy(
                    &mut object_145,
                    value_144,
                )?;
                object_145.finish();
            }
        }
        object_142.finish();
    }
    if let Some(var_146) = &input.policy_arn {
        object.key("policyArn").string(var_146.as_str());
    }
    if let Some(var_147) = &input.policy_description {
        object.key("policyDescription").string(var_147.as_str());
    }
    if let Some(var_148) = &input.policy_name {
        object.key("policyName").string(var_148.as_str());
    }
    if let Some(var_149) = &input.tier {
        object.key("tier").string(var_149.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_mapping(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceMapping,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_150) = &input.resource_name {
        object.key("resourceName").string(var_150.as_str());
    }
    if let Some(var_151) = &input.logical_stack_name {
        object.key("logicalStackName").string(var_151.as_str());
    }
    if let Some(var_152) = &input.app_registry_app_name {
        object.key("appRegistryAppName").string(var_152.as_str());
    }
    if let Some(var_153) = &input.resource_group_name {
        object.key("resourceGroupName").string(var_153.as_str());
    }
    if let Some(var_154) = &input.mapping_type {
        object.key("mappingType").string(var_154.as_str());
    }
    if let Some(var_155) = &input.physical_resource_id {
        let mut object_156 = object.key("physicalResourceId").start_object();
        crate::json_ser::serialize_structure_crate_model_physical_resource_id(
            &mut object_156,
            var_155,
        )?;
        object_156.finish();
    }
    if let Some(var_157) = &input.terraform_source_name {
        object.key("terraformSourceName").string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_failure_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FailurePolicy,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("rtoInSecs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rto_in_secs).into()),
        );
    }
    {
        object.key("rpoInSecs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rpo_in_secs).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_terraform_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TerraformSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_158) = &input.s3_state_file_url {
        object.key("s3StateFileUrl").string(var_158.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_physical_resource_id(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PhysicalResourceId,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_159) = &input.identifier {
        object.key("identifier").string(var_159.as_str());
    }
    if let Some(var_160) = &input.r#type {
        object.key("type").string(var_160.as_str());
    }
    if let Some(var_161) = &input.aws_region {
        object.key("awsRegion").string(var_161.as_str());
    }
    if let Some(var_162) = &input.aws_account_id {
        object.key("awsAccountId").string(var_162.as_str());
    }
    Ok(())
}
