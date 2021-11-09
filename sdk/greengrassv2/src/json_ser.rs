// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_associate_client_device_with_core_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchAssociateClientDeviceWithCoreDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.entries {
        let mut array_2 = object.key("entries").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_associate_client_device_with_core_device_entry(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_disassociate_client_device_from_core_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDisassociateClientDeviceFromCoreDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.entries {
        let mut array_6 = object.key("entries").start_array();
        for item_7 in var_5 {
            {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_disassociate_client_device_from_core_device_entry(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_component_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateComponentVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.client_token {
        object.key("clientToken").string(var_9);
    }
    if let Some(var_10) = &input.inline_recipe {
        object
            .key("inlineRecipe")
            .string_unchecked(&aws_smithy_types::base64::encode(var_10));
    }
    if let Some(var_11) = &input.lambda_function {
        let mut object_12 = object.key("lambdaFunction").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_function_recipe_source(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15).string(value_16);
            }
        }
        object_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_deployment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeploymentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.client_token {
        object.key("clientToken").string(var_17);
    }
    if let Some(var_18) = &input.components {
        let mut object_19 = object.key("components").start_object();
        for (key_20, value_21) in var_18 {
            {
                let mut object_22 = object_19.key(key_20).start_object();
                crate::json_ser::serialize_structure_crate_model_component_deployment_specification(&mut object_22, value_21)?;
                object_22.finish();
            }
        }
        object_19.finish();
    }
    if let Some(var_23) = &input.deployment_name {
        object.key("deploymentName").string(var_23);
    }
    if let Some(var_24) = &input.deployment_policies {
        let mut object_25 = object.key("deploymentPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_policies(
            &mut object_25,
            var_24,
        )?;
        object_25.finish();
    }
    if let Some(var_26) = &input.iot_job_configuration {
        let mut object_27 = object.key("iotJobConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_io_t_job_configuration(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.tags {
        let mut object_29 = object.key("tags").start_object();
        for (key_30, value_31) in var_28 {
            {
                object_29.key(key_30).string(value_31);
            }
        }
        object_29.finish();
    }
    if let Some(var_32) = &input.target_arn {
        object.key("targetArn").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resolve_component_candidates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResolveComponentCandidatesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.component_candidates {
        let mut array_34 = object.key("componentCandidates").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_component_candidate(
                    &mut object_36,
                    item_35,
                )?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.platform {
        let mut object_38 = object.key("platform").start_object();
        crate::json_ser::serialize_structure_crate_model_component_platform(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.tags {
        let mut object_40 = object.key("tags").start_object();
        for (key_41, value_42) in var_39 {
            {
                object_40.key(key_41).string(value_42);
            }
        }
        object_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_associate_client_device_with_core_device_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssociateClientDeviceWithCoreDeviceEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.thing_name {
        object.key("thingName").string(var_43);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_disassociate_client_device_from_core_device_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DisassociateClientDeviceFromCoreDeviceEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.thing_name {
        object.key("thingName").string(var_44);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_function_recipe_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaFunctionRecipeSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.lambda_arn {
        object.key("lambdaArn").string(var_45);
    }
    if let Some(var_46) = &input.component_name {
        object.key("componentName").string(var_46);
    }
    if let Some(var_47) = &input.component_version {
        object.key("componentVersion").string(var_47);
    }
    if let Some(var_48) = &input.component_platforms {
        let mut array_49 = object.key("componentPlatforms").start_array();
        for item_50 in var_48 {
            {
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_component_platform(
                    &mut object_51,
                    item_50,
                )?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    if let Some(var_52) = &input.component_dependencies {
        let mut object_53 = object.key("componentDependencies").start_object();
        for (key_54, value_55) in var_52 {
            {
                let mut object_56 = object_53.key(key_54).start_object();
                crate::json_ser::serialize_structure_crate_model_component_dependency_requirement(
                    &mut object_56,
                    value_55,
                )?;
                object_56.finish();
            }
        }
        object_53.finish();
    }
    if let Some(var_57) = &input.component_lambda_parameters {
        let mut object_58 = object.key("componentLambdaParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_execution_parameters(
            &mut object_58,
            var_57,
        )?;
        object_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_deployment_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentDeploymentSpecification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.component_version {
        object.key("componentVersion").string(var_59);
    }
    if let Some(var_60) = &input.configuration_update {
        let mut object_61 = object.key("configurationUpdate").start_object();
        crate::json_ser::serialize_structure_crate_model_component_configuration_update(
            &mut object_61,
            var_60,
        )?;
        object_61.finish();
    }
    if let Some(var_62) = &input.run_with {
        let mut object_63 = object.key("runWith").start_object();
        crate::json_ser::serialize_structure_crate_model_component_run_with(
            &mut object_63,
            var_62,
        )?;
        object_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_policies(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentPolicies,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.failure_handling_policy {
        object.key("failureHandlingPolicy").string(var_64.as_str());
    }
    if let Some(var_65) = &input.component_update_policy {
        let mut object_66 = object.key("componentUpdatePolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_component_update_policy(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.configuration_validation_policy {
        let mut object_68 = object.key("configurationValidationPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_configuration_validation_policy(&mut object_68, var_67)?;
        object_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_io_t_job_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentIoTJobConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.job_executions_rollout_config {
        let mut object_70 = object.key("jobExecutionsRolloutConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_executions_rollout_config(
            &mut object_70,
            var_69,
        )?;
        object_70.finish();
    }
    if let Some(var_71) = &input.abort_config {
        let mut object_72 = object.key("abortConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_abort_config(
            &mut object_72,
            var_71,
        )?;
        object_72.finish();
    }
    if let Some(var_73) = &input.timeout_config {
        let mut object_74 = object.key("timeoutConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_timeout_config(
            &mut object_74,
            var_73,
        )?;
        object_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_candidate(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentCandidate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.component_name {
        object.key("componentName").string(var_75);
    }
    if let Some(var_76) = &input.component_version {
        object.key("componentVersion").string(var_76);
    }
    if let Some(var_77) = &input.version_requirements {
        let mut object_78 = object.key("versionRequirements").start_object();
        for (key_79, value_80) in var_77 {
            {
                object_78.key(key_79).string(value_80);
            }
        }
        object_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_platform(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentPlatform,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.name {
        object.key("name").string(var_81);
    }
    if let Some(var_82) = &input.attributes {
        let mut object_83 = object.key("attributes").start_object();
        for (key_84, value_85) in var_82 {
            {
                object_83.key(key_84).string(value_85);
            }
        }
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_dependency_requirement(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentDependencyRequirement,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.version_requirement {
        object.key("versionRequirement").string(var_86);
    }
    if let Some(var_87) = &input.dependency_type {
        object.key("dependencyType").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_execution_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaExecutionParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.event_sources {
        let mut array_89 = object.key("eventSources").start_array();
        for item_90 in var_88 {
            {
                let mut object_91 = array_89.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_event_source(
                    &mut object_91,
                    item_90,
                )?;
                object_91.finish();
            }
        }
        array_89.finish();
    }
    if let Some(var_92) = &input.max_queue_size {
        object.key("maxQueueSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_92).into()),
        );
    }
    if let Some(var_93) = &input.max_instances_count {
        object.key("maxInstancesCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_93).into()),
        );
    }
    if let Some(var_94) = &input.max_idle_time_in_seconds {
        object.key("maxIdleTimeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_94).into()),
        );
    }
    if let Some(var_95) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_95).into()),
        );
    }
    if let Some(var_96) = &input.status_timeout_in_seconds {
        object.key("statusTimeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_96).into()),
        );
    }
    if let Some(var_97) = &input.pinned {
        object.key("pinned").boolean(*var_97);
    }
    if let Some(var_98) = &input.input_payload_encoding_type {
        object
            .key("inputPayloadEncodingType")
            .string(var_98.as_str());
    }
    if let Some(var_99) = &input.exec_args {
        let mut array_100 = object.key("execArgs").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101);
            }
        }
        array_100.finish();
    }
    if let Some(var_102) = &input.environment_variables {
        let mut object_103 = object.key("environmentVariables").start_object();
        for (key_104, value_105) in var_102 {
            {
                object_103.key(key_104).string(value_105);
            }
        }
        object_103.finish();
    }
    if let Some(var_106) = &input.linux_process_params {
        let mut object_107 = object.key("linuxProcessParams").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_linux_process_params(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_configuration_update(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentConfigurationUpdate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.merge {
        object.key("merge").string(var_108);
    }
    if let Some(var_109) = &input.reset {
        let mut array_110 = object.key("reset").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111);
            }
        }
        array_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_run_with(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentRunWith,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_112) = &input.posix_user {
        object.key("posixUser").string(var_112);
    }
    if let Some(var_113) = &input.system_resource_limits {
        let mut object_114 = object.key("systemResourceLimits").start_object();
        crate::json_ser::serialize_structure_crate_model_system_resource_limits(
            &mut object_114,
            var_113,
        )?;
        object_114.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_component_update_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentComponentUpdatePolicy,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_115).into()),
        );
    }
    if let Some(var_116) = &input.action {
        object.key("action").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_configuration_validation_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentConfigurationValidationPolicy,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_executions_rollout_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobExecutionsRolloutConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.exponential_rate {
        let mut object_119 = object.key("exponentialRate").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_exponential_rollout_rate(
            &mut object_119,
            var_118,
        )?;
        object_119.finish();
    }
    if let Some(var_120) = &input.maximum_per_minute {
        object.key("maximumPerMinute").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_120).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_abort_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobAbortConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.criteria_list {
        let mut array_122 = object.key("criteriaList").start_array();
        for item_123 in var_121 {
            {
                let mut object_124 = array_122.value().start_object();
                crate::json_ser::serialize_structure_crate_model_io_t_job_abort_criteria(
                    &mut object_124,
                    item_123,
                )?;
                object_124.finish();
            }
        }
        array_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_timeout_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobTimeoutConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.in_progress_timeout_in_minutes {
        object.key("inProgressTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_125).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_event_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaEventSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_126) = &input.topic {
        object.key("topic").string(var_126);
    }
    if let Some(var_127) = &input.r#type {
        object.key("type").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_linux_process_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaLinuxProcessParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.isolation_mode {
        object.key("isolationMode").string(var_128.as_str());
    }
    if let Some(var_129) = &input.container_params {
        let mut object_130 = object.key("containerParams").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_container_params(
            &mut object_130,
            var_129,
        )?;
        object_130.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_system_resource_limits(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SystemResourceLimits,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.memory != 0 {
        object.key("memory").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.memory).into()),
        );
    }
    if input.cpus != 0.0 {
        object.key("cpus").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.cpus).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_exponential_rollout_rate(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobExponentialRolloutRate,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("baseRatePerMinute").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.base_rate_per_minute).into()),
        );
    }
    {
        object.key("incrementFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.increment_factor).into()),
        );
    }
    if let Some(var_131) = &input.rate_increase_criteria {
        let mut object_132 = object.key("rateIncreaseCriteria").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_rate_increase_criteria(
            &mut object_132,
            var_131,
        )?;
        object_132.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_abort_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobAbortCriteria,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_133) = &input.failure_type {
        object.key("failureType").string(var_133.as_str());
    }
    if let Some(var_134) = &input.action {
        object.key("action").string(var_134.as_str());
    }
    {
        object.key("thresholdPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.threshold_percentage).into()),
        );
    }
    {
        object.key("minNumberOfExecutedThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_number_of_executed_things).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_container_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaContainerParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_135) = &input.memory_size_in_kb {
        object.key("memorySizeInKB").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.mount_ro_sysfs {
        object.key("mountROSysfs").boolean(*var_136);
    }
    if let Some(var_137) = &input.volumes {
        let mut array_138 = object.key("volumes").start_array();
        for item_139 in var_137 {
            {
                let mut object_140 = array_138.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_volume_mount(
                    &mut object_140,
                    item_139,
                )?;
                object_140.finish();
            }
        }
        array_138.finish();
    }
    if let Some(var_141) = &input.devices {
        let mut array_142 = object.key("devices").start_array();
        for item_143 in var_141 {
            {
                let mut object_144 = array_142.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_device_mount(
                    &mut object_144,
                    item_143,
                )?;
                object_144.finish();
            }
        }
        array_142.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_rate_increase_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobRateIncreaseCriteria,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_145) = &input.number_of_notified_things {
        object.key("numberOfNotifiedThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_145).into()),
        );
    }
    if let Some(var_146) = &input.number_of_succeeded_things {
        object.key("numberOfSucceededThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_146).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_volume_mount(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaVolumeMount,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.source_path {
        object.key("sourcePath").string(var_147);
    }
    if let Some(var_148) = &input.destination_path {
        object.key("destinationPath").string(var_148);
    }
    if let Some(var_149) = &input.permission {
        object.key("permission").string(var_149.as_str());
    }
    if let Some(var_150) = &input.add_group_owner {
        object.key("addGroupOwner").boolean(*var_150);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_device_mount(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaDeviceMount,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.path {
        object.key("path").string(var_151);
    }
    if let Some(var_152) = &input.permission {
        object.key("permission").string(var_152.as_str());
    }
    if let Some(var_153) = &input.add_group_owner {
        object.key("addGroupOwner").boolean(*var_153);
    }
    Ok(())
}
