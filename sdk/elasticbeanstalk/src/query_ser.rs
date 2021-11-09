// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_application_resource_lifecycle_config(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ApplicationResourceLifecycleConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ServiceRole");
    if let Some(var_2) = &input.service_role {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VersionLifecycleConfig");
    if let Some(var_4) = &input.version_lifecycle_config {
        crate::query_ser::serialize_structure_crate_model_application_version_lifecycle_config(
            scope_3, var_4,
        )?;
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
pub fn serialize_structure_crate_model_source_build_information(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::SourceBuildInformation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("SourceType");
    if let Some(var_10) = &input.source_type {
        scope_9.string(var_10.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SourceRepository");
    if let Some(var_12) = &input.source_repository {
        scope_11.string(var_12.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("SourceLocation");
    if let Some(var_14) = &input.source_location {
        scope_13.string(var_14);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_s3_location(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::S3Location,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("S3Bucket");
    if let Some(var_16) = &input.s3_bucket {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("S3Key");
    if let Some(var_18) = &input.s3_key {
        scope_17.string(var_18);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_build_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::BuildConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("ArtifactName");
    if let Some(var_20) = &input.artifact_name {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("CodeBuildServiceRole");
    if let Some(var_22) = &input.code_build_service_role {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("ComputeType");
    if let Some(var_24) = &input.compute_type {
        scope_23.string(var_24.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("Image");
    if let Some(var_26) = &input.image {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("TimeoutInMinutes");
    if let Some(var_28) = &input.timeout_in_minutes {
        scope_27.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_28).into()),
        );
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_source_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::SourceConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("ApplicationName");
    if let Some(var_30) = &input.application_name {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("TemplateName");
    if let Some(var_32) = &input.template_name {
        scope_31.string(var_32);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_configuration_option_setting(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ConfigurationOptionSetting,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("ResourceName");
    if let Some(var_34) = &input.resource_name {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("Namespace");
    if let Some(var_36) = &input.namespace {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("OptionName");
    if let Some(var_38) = &input.option_name {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Value");
    if let Some(var_40) = &input.value {
        scope_39.string(var_40);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_environment_tier(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::EnvironmentTier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("Name");
    if let Some(var_42) = &input.name {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("Type");
    if let Some(var_44) = &input.r#type {
        scope_43.string(var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("Version");
    if let Some(var_46) = &input.version {
        scope_45.string(var_46);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_option_specification(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::OptionSpecification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("ResourceName");
    if let Some(var_48) = &input.resource_name {
        scope_47.string(var_48);
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("Namespace");
    if let Some(var_50) = &input.namespace {
        scope_49.string(var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("OptionName");
    if let Some(var_52) = &input.option_name {
        scope_51.string(var_52);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_search_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::SearchFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("Attribute");
    if let Some(var_54) = &input.attribute {
        scope_53.string(var_54);
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("Operator");
    if let Some(var_56) = &input.operator {
        scope_55.string(var_56);
    }
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("Values");
    if let Some(var_58) = &input.values {
        let mut list_60 = scope_57.start_list(false, None);
        for item_59 in var_58 {
            #[allow(unused_mut)]
            let mut entry_61 = list_60.entry();
            entry_61.string(item_59);
        }
        list_60.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_platform_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::PlatformFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("Type");
    if let Some(var_63) = &input.r#type {
        scope_62.string(var_63);
    }
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("Operator");
    if let Some(var_65) = &input.operator {
        scope_64.string(var_65);
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("Values");
    if let Some(var_67) = &input.values {
        let mut list_69 = scope_66.start_list(false, None);
        for item_68 in var_67 {
            #[allow(unused_mut)]
            let mut entry_70 = list_69.entry();
            entry_70.string(item_68);
        }
        list_69.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_application_version_lifecycle_config(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ApplicationVersionLifecycleConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("MaxCountRule");
    if let Some(var_72) = &input.max_count_rule {
        crate::query_ser::serialize_structure_crate_model_max_count_rule(scope_71, var_72)?;
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("MaxAgeRule");
    if let Some(var_74) = &input.max_age_rule {
        crate::query_ser::serialize_structure_crate_model_max_age_rule(scope_73, var_74)?;
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_max_count_rule(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MaxCountRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("Enabled");
    if let Some(var_76) = &input.enabled {
        scope_75.boolean(*var_76);
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("MaxCount");
    if let Some(var_78) = &input.max_count {
        scope_77.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_78).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("DeleteSourceFromS3");
    if let Some(var_80) = &input.delete_source_from_s3 {
        scope_79.boolean(*var_80);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_max_age_rule(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MaxAgeRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("Enabled");
    if let Some(var_82) = &input.enabled {
        scope_81.boolean(*var_82);
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("MaxAgeInDays");
    if let Some(var_84) = &input.max_age_in_days {
        scope_83.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("DeleteSourceFromS3");
    if let Some(var_86) = &input.delete_source_from_s3 {
        scope_85.boolean(*var_86);
    }
    Ok(())
}
