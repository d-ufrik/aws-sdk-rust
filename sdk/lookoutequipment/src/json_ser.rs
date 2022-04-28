// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.dataset_name {
        object.key("DatasetName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dataset_schema {
        let mut object_3 = object.key("DatasetSchema").start_object();
        crate::json_ser::serialize_structure_crate_model_dataset_schema(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.client_token {
        object.key("ClientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.model_name {
        object.key("ModelName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.data_delay_offset_in_minutes {
        object.key("DataDelayOffsetInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.data_upload_frequency {
        object.key("DataUploadFrequency").string(var_13.as_str());
    }
    if let Some(var_14) = &input.data_input_configuration {
        let mut object_15 = object.key("DataInputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_input_configuration(
            &mut object_15,
            var_14,
        )?;
        object_15.finish();
    }
    if let Some(var_16) = &input.data_output_configuration {
        let mut object_17 = object.key("DataOutputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_output_configuration(
            &mut object_17,
            var_16,
        )?;
        object_17.finish();
    }
    if let Some(var_18) = &input.role_arn {
        object.key("RoleArn").string(var_18.as_str());
    }
    if let Some(var_19) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.client_token {
        object.key("ClientToken").string(var_20.as_str());
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_model_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateModelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.model_name {
        object.key("ModelName").string(var_25.as_str());
    }
    if let Some(var_26) = &input.dataset_name {
        object.key("DatasetName").string(var_26.as_str());
    }
    if let Some(var_27) = &input.dataset_schema {
        let mut object_28 = object.key("DatasetSchema").start_object();
        crate::json_ser::serialize_structure_crate_model_dataset_schema(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.labels_input_configuration {
        let mut object_30 = object.key("LabelsInputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_labels_input_configuration(
            &mut object_30,
            var_29,
        )?;
        object_30.finish();
    }
    if let Some(var_31) = &input.client_token {
        object.key("ClientToken").string(var_31.as_str());
    }
    if let Some(var_32) = &input.training_data_start_time {
        object
            .key("TrainingDataStartTime")
            .date_time(var_32, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_33) = &input.training_data_end_time {
        object
            .key("TrainingDataEndTime")
            .date_time(var_33, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_34) = &input.evaluation_data_start_time {
        object
            .key("EvaluationDataStartTime")
            .date_time(var_34, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_35) = &input.evaluation_data_end_time {
        object
            .key("EvaluationDataEndTime")
            .date_time(var_35, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_36) = &input.role_arn {
        object.key("RoleArn").string(var_36.as_str());
    }
    if let Some(var_37) = &input.data_pre_processing_configuration {
        let mut object_38 = object.key("DataPreProcessingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_data_pre_processing_configuration(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    if let Some(var_39) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_39.as_str());
    }
    if let Some(var_40) = &input.tags {
        let mut array_41 = object.key("Tags").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    if let Some(var_44) = &input.off_condition {
        object.key("OffCondition").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.dataset_name {
        object.key("DatasetName").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_model_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteModelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.model_name {
        object.key("ModelName").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_data_ingestion_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDataIngestionJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.job_id {
        object.key("JobId").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.dataset_name {
        object.key("DatasetName").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_model_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeModelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.model_name {
        object.key("ModelName").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_data_ingestion_jobs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDataIngestionJobsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.dataset_name {
        object.key("DatasetName").string(var_52.as_str());
    }
    if let Some(var_53) = &input.next_token {
        object.key("NextToken").string(var_53.as_str());
    }
    if let Some(var_54) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    if let Some(var_55) = &input.status {
        object.key("Status").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_datasets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatasetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.next_token {
        object.key("NextToken").string(var_56.as_str());
    }
    if let Some(var_57) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.dataset_name_begins_with {
        object.key("DatasetNameBeginsWith").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_inference_executions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInferenceExecutionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.next_token {
        object.key("NextToken").string(var_59.as_str());
    }
    if let Some(var_60) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    if let Some(var_61) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_61.as_str());
    }
    if let Some(var_62) = &input.data_start_time_after {
        object
            .key("DataStartTimeAfter")
            .date_time(var_62, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_63) = &input.data_end_time_before {
        object
            .key("DataEndTimeBefore")
            .date_time(var_63, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_64) = &input.status {
        object.key("Status").string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_inference_schedulers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInferenceSchedulersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    if let Some(var_66) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    if let Some(var_67) = &input.inference_scheduler_name_begins_with {
        object
            .key("InferenceSchedulerNameBeginsWith")
            .string(var_67.as_str());
    }
    if let Some(var_68) = &input.model_name {
        object.key("ModelName").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_models_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListModelsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.next_token {
        object.key("NextToken").string(var_69.as_str());
    }
    if let Some(var_70) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_70).into()),
        );
    }
    if let Some(var_71) = &input.status {
        object.key("Status").string(var_71.as_str());
    }
    if let Some(var_72) = &input.model_name_begins_with {
        object.key("ModelNameBeginsWith").string(var_72.as_str());
    }
    if let Some(var_73) = &input.dataset_name_begins_with {
        object.key("DatasetNameBeginsWith").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_sensor_statistics_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSensorStatisticsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.dataset_name {
        object.key("DatasetName").string(var_74.as_str());
    }
    if let Some(var_75) = &input.ingestion_job_id {
        object.key("IngestionJobId").string(var_75.as_str());
    }
    if let Some(var_76) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    if let Some(var_77) = &input.next_token {
        object.key("NextToken").string(var_77.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.resource_arn {
        object.key("ResourceArn").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_data_ingestion_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDataIngestionJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.dataset_name {
        object.key("DatasetName").string(var_79.as_str());
    }
    if let Some(var_80) = &input.ingestion_input_configuration {
        let mut object_81 = object.key("IngestionInputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ingestion_input_configuration(
            &mut object_81,
            var_80,
        )?;
        object_81.finish();
    }
    if let Some(var_82) = &input.role_arn {
        object.key("RoleArn").string(var_82.as_str());
    }
    if let Some(var_83) = &input.client_token {
        object.key("ClientToken").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.resource_arn {
        object.key("ResourceArn").string(var_86.as_str());
    }
    if let Some(var_87) = &input.tags {
        let mut array_88 = object.key("Tags").start_array();
        for item_89 in var_87 {
            {
                let mut object_90 = array_88.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_90, item_89)?;
                object_90.finish();
            }
        }
        array_88.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.resource_arn {
        object.key("ResourceArn").string(var_91.as_str());
    }
    if let Some(var_92) = &input.tag_keys {
        let mut array_93 = object.key("TagKeys").start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94.as_str());
            }
        }
        array_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_inference_scheduler_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateInferenceSchedulerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_95.as_str());
    }
    if let Some(var_96) = &input.data_delay_offset_in_minutes {
        object.key("DataDelayOffsetInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_96).into()),
        );
    }
    if let Some(var_97) = &input.data_upload_frequency {
        object.key("DataUploadFrequency").string(var_97.as_str());
    }
    if let Some(var_98) = &input.data_input_configuration {
        let mut object_99 = object.key("DataInputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_input_configuration(
            &mut object_99,
            var_98,
        )?;
        object_99.finish();
    }
    if let Some(var_100) = &input.data_output_configuration {
        let mut object_101 = object.key("DataOutputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_output_configuration(
            &mut object_101,
            var_100,
        )?;
        object_101.finish();
    }
    if let Some(var_102) = &input.role_arn {
        object.key("RoleArn").string(var_102.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dataset_schema(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetSchema,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.inline_data_schema {
        object.key("InlineDataSchema").string(var_103.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_104) = &input.key {
        object.key("Key").string(var_104.as_str());
    }
    if let Some(var_105) = &input.value {
        object.key("Value").string(var_105.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_inference_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceInputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.s3_input_configuration {
        let mut object_107 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_s3_input_configuration(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    if let Some(var_108) = &input.input_time_zone_offset {
        object.key("InputTimeZoneOffset").string(var_108.as_str());
    }
    if let Some(var_109) = &input.inference_input_name_configuration {
        let mut object_110 = object.key("InferenceInputNameConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_input_name_configuration(
            &mut object_110,
            var_109,
        )?;
        object_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_inference_output_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceOutputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.s3_output_configuration {
        let mut object_112 = object.key("S3OutputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_inference_s3_output_configuration(
            &mut object_112,
            var_111,
        )?;
        object_112.finish();
    }
    if let Some(var_113) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_113.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_labels_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LabelsInputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.s3_input_configuration {
        let mut object_115 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_labels_s3_input_configuration(
            &mut object_115,
            var_114,
        )?;
        object_115.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_pre_processing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataPreProcessingConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_116) = &input.target_sampling_rate {
        object.key("TargetSamplingRate").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ingestion_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IngestionInputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.s3_input_configuration {
        let mut object_118 = object.key("S3InputConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ingestion_s3_input_configuration(
            &mut object_118,
            var_117,
        )?;
        object_118.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_inference_s3_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceS3InputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.bucket {
        object.key("Bucket").string(var_119.as_str());
    }
    if let Some(var_120) = &input.prefix {
        object.key("Prefix").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_inference_input_name_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceInputNameConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.timestamp_format {
        object.key("TimestampFormat").string(var_121.as_str());
    }
    if let Some(var_122) = &input.component_timestamp_delimiter {
        object
            .key("ComponentTimestampDelimiter")
            .string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_inference_s3_output_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InferenceS3OutputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.bucket {
        object.key("Bucket").string(var_123.as_str());
    }
    if let Some(var_124) = &input.prefix {
        object.key("Prefix").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_labels_s3_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LabelsS3InputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.bucket {
        object.key("Bucket").string(var_125.as_str());
    }
    if let Some(var_126) = &input.prefix {
        object.key("Prefix").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ingestion_s3_input_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IngestionS3InputConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.bucket {
        object.key("Bucket").string(var_127.as_str());
    }
    if let Some(var_128) = &input.prefix {
        object.key("Prefix").string(var_128.as_str());
    }
    if let Some(var_129) = &input.key_pattern {
        object.key("KeyPattern").string(var_129.as_str());
    }
    Ok(())
}
