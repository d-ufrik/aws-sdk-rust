// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_register_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterDeviceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.platform {
        object.key("Platform").string(var_1.as_str());
    }
    if let Some(var_2) = &input.token {
        object.key("Token").string(var_2);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_cognito_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetCognitoEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_3) = &input.events {
        let mut object_4 = object.key("Events").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5).string(value_6);
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_identity_pool_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetIdentityPoolConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.cognito_streams {
        let mut object_8 = object.key("CognitoStreams").start_object();
        crate::json_ser::serialize_structure_crate_model_cognito_streams(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.push_sync {
        let mut object_10 = object.key("PushSync").start_object();
        crate::json_ser::serialize_structure_crate_model_push_sync(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRecordsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.device_id {
        object.key("DeviceId").string(var_11);
    }
    if let Some(var_12) = &input.record_patches {
        let mut array_13 = object.key("RecordPatches").start_array();
        for item_14 in var_12 {
            {
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_record_patch(
                    &mut object_15,
                    item_14,
                )?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.sync_session_token {
        object.key("SyncSessionToken").string(var_16);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cognito_streams(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoStreams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.stream_name {
        object.key("StreamName").string(var_17);
    }
    if let Some(var_18) = &input.role_arn {
        object.key("RoleArn").string(var_18);
    }
    if let Some(var_19) = &input.streaming_status {
        object.key("StreamingStatus").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_push_sync(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PushSync,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.application_arns {
        let mut array_21 = object.key("ApplicationArns").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22);
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.role_arn {
        object.key("RoleArn").string(var_23);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_record_patch(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RecordPatch,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.op {
        object.key("Op").string(var_24.as_str());
    }
    if let Some(var_25) = &input.key {
        object.key("Key").string(var_25);
    }
    if let Some(var_26) = &input.value {
        object.key("Value").string(var_26);
    }
    if let Some(var_27) = &input.sync_count {
        object.key("SyncCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    if let Some(var_28) = &input.device_last_modified_date {
        object
            .key("DeviceLastModifiedDate")
            .instant(var_28, aws_smithy_types::instant::Format::EpochSeconds);
    }
    Ok(())
}
