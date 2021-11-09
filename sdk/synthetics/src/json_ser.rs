// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_canary_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCanaryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.artifact_config {
        let mut object_2 = object.key("ArtifactConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_artifact_config_input(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.artifact_s3_location {
        object.key("ArtifactS3Location").string(var_3);
    }
    if let Some(var_4) = &input.code {
        let mut object_5 = object.key("Code").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_code_input(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_6);
    }
    if let Some(var_7) = &input.failure_retention_period_in_days {
        object.key("FailureRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8);
    }
    if let Some(var_9) = &input.run_config {
        let mut object_10 = object.key("RunConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_run_config_input(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    if let Some(var_11) = &input.runtime_version {
        object.key("RuntimeVersion").string(var_11);
    }
    if let Some(var_12) = &input.schedule {
        let mut object_13 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_schedule_input(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.success_retention_period_in_days {
        object.key("SuccessRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.tags {
        let mut object_16 = object.key("Tags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17).string(value_18);
            }
        }
        object_16.finish();
    }
    if let Some(var_19) = &input.vpc_config {
        let mut object_20 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config_input(&mut object_20, var_19)?;
        object_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_canaries_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCanariesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_21).into()),
        );
    }
    if let Some(var_22) = &input.next_token {
        object.key("NextToken").string(var_22);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_canaries_last_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCanariesLastRunInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    if let Some(var_24) = &input.next_token {
        object.key("NextToken").string(var_24);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_runtime_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRuntimeVersionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_25).into()),
        );
    }
    if let Some(var_26) = &input.next_token {
        object.key("NextToken").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_canary_runs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCanaryRunsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    if let Some(var_28) = &input.next_token {
        object.key("NextToken").string(var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.tags {
        let mut object_30 = object.key("Tags").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31).string(value_32);
            }
        }
        object_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_canary_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCanaryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.artifact_config {
        let mut object_34 = object.key("ArtifactConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_artifact_config_input(
            &mut object_34,
            var_33,
        )?;
        object_34.finish();
    }
    if let Some(var_35) = &input.artifact_s3_location {
        object.key("ArtifactS3Location").string(var_35);
    }
    if let Some(var_36) = &input.code {
        let mut object_37 = object.key("Code").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_code_input(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_38);
    }
    if let Some(var_39) = &input.failure_retention_period_in_days {
        object.key("FailureRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    if let Some(var_40) = &input.run_config {
        let mut object_41 = object.key("RunConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_run_config_input(
            &mut object_41,
            var_40,
        )?;
        object_41.finish();
    }
    if let Some(var_42) = &input.runtime_version {
        object.key("RuntimeVersion").string(var_42);
    }
    if let Some(var_43) = &input.schedule {
        let mut object_44 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_crate_model_canary_schedule_input(
            &mut object_44,
            var_43,
        )?;
        object_44.finish();
    }
    if let Some(var_45) = &input.success_retention_period_in_days {
        object.key("SuccessRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.visual_reference {
        let mut object_47 = object.key("VisualReference").start_object();
        crate::json_ser::serialize_structure_crate_model_visual_reference_input(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    if let Some(var_48) = &input.vpc_config {
        let mut object_49 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config_input(&mut object_49, var_48)?;
        object_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_artifact_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ArtifactConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.s3_encryption {
        let mut object_51 = object.key("S3Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_encryption_config(
            &mut object_51,
            var_50,
        )?;
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_canary_code_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CanaryCodeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.s3_bucket {
        object.key("S3Bucket").string(var_52);
    }
    if let Some(var_53) = &input.s3_key {
        object.key("S3Key").string(var_53);
    }
    if let Some(var_54) = &input.s3_version {
        object.key("S3Version").string(var_54);
    }
    if let Some(var_55) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&aws_smithy_types::base64::encode(var_55));
    }
    if let Some(var_56) = &input.handler {
        object.key("Handler").string(var_56);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_canary_run_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CanaryRunConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.timeout_in_seconds {
        object.key("TimeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.memory_in_mb {
        object.key("MemoryInMB").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    if let Some(var_59) = &input.active_tracing {
        object.key("ActiveTracing").boolean(*var_59);
    }
    if let Some(var_60) = &input.environment_variables {
        let mut object_61 = object.key("EnvironmentVariables").start_object();
        for (key_62, value_63) in var_60 {
            {
                object_61.key(key_62).string(value_63);
            }
        }
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_canary_schedule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CanaryScheduleInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.expression {
        object.key("Expression").string(var_64);
    }
    if let Some(var_65) = &input.duration_in_seconds {
        object.key("DurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_65).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.subnet_ids {
        let mut array_67 = object.key("SubnetIds").start_array();
        for item_68 in var_66 {
            {
                array_67.value().string(item_68);
            }
        }
        array_67.finish();
    }
    if let Some(var_69) = &input.security_group_ids {
        let mut array_70 = object.key("SecurityGroupIds").start_array();
        for item_71 in var_69 {
            {
                array_70.value().string(item_71);
            }
        }
        array_70.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_visual_reference_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VisualReferenceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.base_screenshots {
        let mut array_73 = object.key("BaseScreenshots").start_array();
        for item_74 in var_72 {
            {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_base_screenshot(
                    &mut object_75,
                    item_74,
                )?;
                object_75.finish();
            }
        }
        array_73.finish();
    }
    if let Some(var_76) = &input.base_canary_run_id {
        object.key("BaseCanaryRunId").string(var_76);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_encryption_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3EncryptionConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.encryption_mode {
        object.key("EncryptionMode").string(var_77.as_str());
    }
    if let Some(var_78) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_base_screenshot(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BaseScreenshot,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.screenshot_name {
        object.key("ScreenshotName").string(var_79);
    }
    if let Some(var_80) = &input.ignore_coordinates {
        let mut array_81 = object.key("IgnoreCoordinates").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82);
            }
        }
        array_81.finish();
    }
    Ok(())
}
