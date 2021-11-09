// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_analyze_document_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AnalyzeDocumentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.document {
        let mut object_2 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.feature_types {
        let mut array_4 = object.key("FeatureTypes").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.human_loop_config {
        let mut object_7 = object.key("HumanLoopConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_human_loop_config(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_analyze_expense_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AnalyzeExpenseInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.document {
        let mut object_9 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_detect_document_text_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DetectDocumentTextInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.document {
        let mut object_11 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_document_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDocumentAnalysisInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.job_id {
        object.key("JobId").string(var_12);
    }
    if let Some(var_13) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.next_token {
        object.key("NextToken").string(var_14);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_document_text_detection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDocumentTextDetectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.job_id {
        object.key("JobId").string(var_15);
    }
    if let Some(var_16) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.next_token {
        object.key("NextToken").string(var_17);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_expense_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetExpenseAnalysisInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.job_id {
        object.key("JobId").string(var_18);
    }
    if let Some(var_19) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.next_token {
        object.key("NextToken").string(var_20);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_document_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDocumentAnalysisInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.document_location {
        let mut object_22 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.feature_types {
        let mut array_24 = object.key("FeatureTypes").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_26);
    }
    if let Some(var_27) = &input.job_tag {
        object.key("JobTag").string(var_27);
    }
    if let Some(var_28) = &input.notification_channel {
        let mut object_29 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_29,
            var_28,
        )?;
        object_29.finish();
    }
    if let Some(var_30) = &input.output_config {
        let mut object_31 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_31, var_30)?;
        object_31.finish();
    }
    if let Some(var_32) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_document_text_detection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDocumentTextDetectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.document_location {
        let mut object_34 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_35);
    }
    if let Some(var_36) = &input.job_tag {
        object.key("JobTag").string(var_36);
    }
    if let Some(var_37) = &input.notification_channel {
        let mut object_38 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    if let Some(var_39) = &input.output_config {
        let mut object_40 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_41);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_expense_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartExpenseAnalysisInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.document_location {
        let mut object_43 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_43, var_42)?;
        object_43.finish();
    }
    if let Some(var_44) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_44);
    }
    if let Some(var_45) = &input.job_tag {
        object.key("JobTag").string(var_45);
    }
    if let Some(var_46) = &input.notification_channel {
        let mut object_47 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    if let Some(var_48) = &input.output_config {
        let mut object_49 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_49, var_48)?;
        object_49.finish();
    }
    if let Some(var_50) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_50);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_document(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Document,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.bytes {
        object
            .key("Bytes")
            .string_unchecked(&aws_smithy_types::base64::encode(var_51));
    }
    if let Some(var_52) = &input.s3_object {
        let mut object_53 = object.key("S3Object").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_object(&mut object_53, var_52)?;
        object_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_human_loop_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.human_loop_name {
        object.key("HumanLoopName").string(var_54);
    }
    if let Some(var_55) = &input.flow_definition_arn {
        object.key("FlowDefinitionArn").string(var_55);
    }
    if let Some(var_56) = &input.data_attributes {
        let mut object_57 = object.key("DataAttributes").start_object();
        crate::json_ser::serialize_structure_crate_model_human_loop_data_attributes(
            &mut object_57,
            var_56,
        )?;
        object_57.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_document_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DocumentLocation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.s3_object {
        let mut object_59 = object.key("S3Object").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_object(&mut object_59, var_58)?;
        object_59.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_channel(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationChannel,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.sns_topic_arn {
        object.key("SNSTopicArn").string(var_60);
    }
    if let Some(var_61) = &input.role_arn {
        object.key("RoleArn").string(var_61);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_output_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.s3_bucket {
        object.key("S3Bucket").string(var_62);
    }
    if let Some(var_63) = &input.s3_prefix {
        object.key("S3Prefix").string(var_63);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_object(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Object,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.bucket {
        object.key("Bucket").string(var_64);
    }
    if let Some(var_65) = &input.name {
        object.key("Name").string(var_65);
    }
    if let Some(var_66) = &input.version {
        object.key("Version").string(var_66);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_human_loop_data_attributes(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopDataAttributes,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.content_classifiers {
        let mut array_68 = object.key("ContentClassifiers").start_array();
        for item_69 in var_67 {
            {
                array_68.value().string(item_69.as_str());
            }
        }
        array_68.finish();
    }
    Ok(())
}
