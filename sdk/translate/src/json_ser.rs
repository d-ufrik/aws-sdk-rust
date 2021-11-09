// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_parallel_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateParallelDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1);
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2);
    }
    if let Some(var_3) = &input.parallel_data_config {
        let mut object_4 = object.key("ParallelDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_parallel_data_config(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.encryption_key {
        let mut object_6 = object.key("EncryptionKey").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_key(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.client_token {
        object.key("ClientToken").string(var_7);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_parallel_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteParallelDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_terminology_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTerminologyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.name {
        object.key("Name").string(var_9);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_text_translation_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTextTranslationJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.job_id {
        object.key("JobId").string(var_10);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_parallel_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetParallelDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.name {
        object.key("Name").string(var_11);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_terminology_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTerminologyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.name {
        object.key("Name").string(var_12);
    }
    if let Some(var_13) = &input.terminology_data_format {
        object.key("TerminologyDataFormat").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_terminology_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportTerminologyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.name {
        object.key("Name").string(var_14);
    }
    if let Some(var_15) = &input.merge_strategy {
        object.key("MergeStrategy").string(var_15.as_str());
    }
    if let Some(var_16) = &input.description {
        object.key("Description").string(var_16);
    }
    if let Some(var_17) = &input.terminology_data {
        let mut object_18 = object.key("TerminologyData").start_object();
        crate::json_ser::serialize_structure_crate_model_terminology_data(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.encryption_key {
        let mut object_20 = object.key("EncryptionKey").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_key(&mut object_20, var_19)?;
        object_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_parallel_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListParallelDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.next_token {
        object.key("NextToken").string(var_21);
    }
    if let Some(var_22) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_terminologies_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTerminologiesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23);
    }
    if let Some(var_24) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_text_translation_jobs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTextTranslationJobsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.filter {
        let mut object_26 = object.key("Filter").start_object();
        crate::json_ser::serialize_structure_crate_model_text_translation_job_filter(
            &mut object_26,
            var_25,
        )?;
        object_26.finish();
    }
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27);
    }
    if let Some(var_28) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_28).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_text_translation_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartTextTranslationJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.job_name {
        object.key("JobName").string(var_29);
    }
    if let Some(var_30) = &input.input_data_config {
        let mut object_31 = object.key("InputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_input_data_config(&mut object_31, var_30)?;
        object_31.finish();
    }
    if let Some(var_32) = &input.output_data_config {
        let mut object_33 = object.key("OutputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_data_config(
            &mut object_33,
            var_32,
        )?;
        object_33.finish();
    }
    if let Some(var_34) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_34);
    }
    if let Some(var_35) = &input.source_language_code {
        object.key("SourceLanguageCode").string(var_35);
    }
    if let Some(var_36) = &input.target_language_codes {
        let mut array_37 = object.key("TargetLanguageCodes").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.terminology_names {
        let mut array_40 = object.key("TerminologyNames").start_array();
        for item_41 in var_39 {
            {
                array_40.value().string(item_41);
            }
        }
        array_40.finish();
    }
    if let Some(var_42) = &input.parallel_data_names {
        let mut array_43 = object.key("ParallelDataNames").start_array();
        for item_44 in var_42 {
            {
                array_43.value().string(item_44);
            }
        }
        array_43.finish();
    }
    if let Some(var_45) = &input.client_token {
        object.key("ClientToken").string(var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_text_translation_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopTextTranslationJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.job_id {
        object.key("JobId").string(var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_translate_text_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TranslateTextInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.text {
        object.key("Text").string(var_47);
    }
    if let Some(var_48) = &input.terminology_names {
        let mut array_49 = object.key("TerminologyNames").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50);
            }
        }
        array_49.finish();
    }
    if let Some(var_51) = &input.source_language_code {
        object.key("SourceLanguageCode").string(var_51);
    }
    if let Some(var_52) = &input.target_language_code {
        object.key("TargetLanguageCode").string(var_52);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_parallel_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateParallelDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.name {
        object.key("Name").string(var_53);
    }
    if let Some(var_54) = &input.description {
        object.key("Description").string(var_54);
    }
    if let Some(var_55) = &input.parallel_data_config {
        let mut object_56 = object.key("ParallelDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_parallel_data_config(
            &mut object_56,
            var_55,
        )?;
        object_56.finish();
    }
    if let Some(var_57) = &input.client_token {
        object.key("ClientToken").string(var_57);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parallel_data_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParallelDataConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.s3_uri {
        object.key("S3Uri").string(var_58);
    }
    if let Some(var_59) = &input.format {
        object.key("Format").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_key(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionKey,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.r#type {
        object.key("Type").string(var_60.as_str());
    }
    if let Some(var_61) = &input.id {
        object.key("Id").string(var_61);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_terminology_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TerminologyData,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.file {
        object
            .key("File")
            .string_unchecked(&aws_smithy_types::base64::encode(var_62));
    }
    if let Some(var_63) = &input.format {
        object.key("Format").string(var_63.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_text_translation_job_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TextTranslationJobFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.job_name {
        object.key("JobName").string(var_64);
    }
    if let Some(var_65) = &input.job_status {
        object.key("JobStatus").string(var_65.as_str());
    }
    if let Some(var_66) = &input.submitted_before_time {
        object
            .key("SubmittedBeforeTime")
            .instant(var_66, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_67) = &input.submitted_after_time {
        object
            .key("SubmittedAfterTime")
            .instant(var_67, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_input_data_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputDataConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.s3_uri {
        object.key("S3Uri").string(var_68);
    }
    if let Some(var_69) = &input.content_type {
        object.key("ContentType").string(var_69);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_output_data_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputDataConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.s3_uri {
        object.key("S3Uri").string(var_70);
    }
    Ok(())
}
