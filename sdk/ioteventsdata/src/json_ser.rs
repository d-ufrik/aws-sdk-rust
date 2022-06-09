// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_acknowledge_alarm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchAcknowledgeAlarmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.acknowledge_action_requests {
        let mut array_2 = object.key("acknowledgeActionRequests").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_acknowledge_alarm_action_request(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_delete_detector_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteDetectorInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.detectors {
        let mut array_6 = object.key("detectors").start_array();
        for item_7 in var_5 {
            {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_delete_detector_request(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_disable_alarm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDisableAlarmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.disable_action_requests {
        let mut array_10 = object.key("disableActionRequests").start_array();
        for item_11 in var_9 {
            {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_disable_alarm_action_request(
                    &mut object_12,
                    item_11,
                )?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_enable_alarm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchEnableAlarmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.enable_action_requests {
        let mut array_14 = object.key("enableActionRequests").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_enable_alarm_action_request(
                    &mut object_16,
                    item_15,
                )?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_put_message_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchPutMessageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.messages {
        let mut array_18 = object.key("messages").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_message(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_reset_alarm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchResetAlarmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.reset_action_requests {
        let mut array_22 = object.key("resetActionRequests").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_reset_alarm_action_request(
                    &mut object_24,
                    item_23,
                )?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_snooze_alarm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchSnoozeAlarmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.snooze_action_requests {
        let mut array_26 = object.key("snoozeActionRequests").start_array();
        for item_27 in var_25 {
            {
                let mut object_28 = array_26.value().start_object();
                crate::json_ser::serialize_structure_crate_model_snooze_alarm_action_request(
                    &mut object_28,
                    item_27,
                )?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_update_detector_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpdateDetectorInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.detectors {
        let mut array_30 = object.key("detectors").start_array();
        for item_31 in var_29 {
            {
                let mut object_32 = array_30.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_detector_request(
                    &mut object_32,
                    item_31,
                )?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_acknowledge_alarm_action_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AcknowledgeAlarmActionRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.request_id {
        object.key("requestId").string(var_33.as_str());
    }
    if let Some(var_34) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_34.as_str());
    }
    if let Some(var_35) = &input.key_value {
        object.key("keyValue").string(var_35.as_str());
    }
    if let Some(var_36) = &input.note {
        object.key("note").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_delete_detector_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeleteDetectorRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.message_id {
        object.key("messageId").string(var_37.as_str());
    }
    if let Some(var_38) = &input.detector_model_name {
        object.key("detectorModelName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.key_value {
        object.key("keyValue").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_disable_alarm_action_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DisableAlarmActionRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.request_id {
        object.key("requestId").string(var_40.as_str());
    }
    if let Some(var_41) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.key_value {
        object.key("keyValue").string(var_42.as_str());
    }
    if let Some(var_43) = &input.note {
        object.key("note").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_enable_alarm_action_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EnableAlarmActionRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.request_id {
        object.key("requestId").string(var_44.as_str());
    }
    if let Some(var_45) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_45.as_str());
    }
    if let Some(var_46) = &input.key_value {
        object.key("keyValue").string(var_46.as_str());
    }
    if let Some(var_47) = &input.note {
        object.key("note").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Message,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.message_id {
        object.key("messageId").string(var_48.as_str());
    }
    if let Some(var_49) = &input.input_name {
        object.key("inputName").string(var_49.as_str());
    }
    if let Some(var_50) = &input.payload {
        object
            .key("payload")
            .string_unchecked(&aws_smithy_types::base64::encode(var_50));
    }
    if let Some(var_51) = &input.timestamp {
        let mut object_52 = object.key("timestamp").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_value(&mut object_52, var_51)?;
        object_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_reset_alarm_action_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResetAlarmActionRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.request_id {
        object.key("requestId").string(var_53.as_str());
    }
    if let Some(var_54) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_54.as_str());
    }
    if let Some(var_55) = &input.key_value {
        object.key("keyValue").string(var_55.as_str());
    }
    if let Some(var_56) = &input.note {
        object.key("note").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_snooze_alarm_action_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnoozeAlarmActionRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.request_id {
        object.key("requestId").string(var_57.as_str());
    }
    if let Some(var_58) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_58.as_str());
    }
    if let Some(var_59) = &input.key_value {
        object.key("keyValue").string(var_59.as_str());
    }
    if let Some(var_60) = &input.note {
        object.key("note").string(var_60.as_str());
    }
    if let Some(var_61) = &input.snooze_duration {
        object.key("snoozeDuration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_detector_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateDetectorRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.message_id {
        object.key("messageId").string(var_62.as_str());
    }
    if let Some(var_63) = &input.detector_model_name {
        object.key("detectorModelName").string(var_63.as_str());
    }
    if let Some(var_64) = &input.key_value {
        object.key("keyValue").string(var_64.as_str());
    }
    if let Some(var_65) = &input.state {
        let mut object_66 = object.key("state").start_object();
        crate::json_ser::serialize_structure_crate_model_detector_state_definition(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timestamp_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimestampValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.time_in_millis {
        object.key("timeInMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_67).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_detector_state_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DetectorStateDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.state_name {
        object.key("stateName").string(var_68.as_str());
    }
    if let Some(var_69) = &input.variables {
        let mut array_70 = object.key("variables").start_array();
        for item_71 in var_69 {
            {
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_crate_model_variable_definition(
                    &mut object_72,
                    item_71,
                )?;
                object_72.finish();
            }
        }
        array_70.finish();
    }
    if let Some(var_73) = &input.timers {
        let mut array_74 = object.key("timers").start_array();
        for item_75 in var_73 {
            {
                let mut object_76 = array_74.value().start_object();
                crate::json_ser::serialize_structure_crate_model_timer_definition(
                    &mut object_76,
                    item_75,
                )?;
                object_76.finish();
            }
        }
        array_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_variable_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VariableDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.name {
        object.key("name").string(var_77.as_str());
    }
    if let Some(var_78) = &input.value {
        object.key("value").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timer_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimerDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.name {
        object.key("name").string(var_79.as_str());
    }
    if let Some(var_80) = &input.seconds {
        object.key("seconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    Ok(())
}
