// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.application_description {
        object.key("applicationDescription").string(var_1);
    }
    if let Some(var_2) = &input.application_name {
        object.key("applicationName").string(var_2);
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3);
    }
    if let Some(var_4) = &input.role_arn {
        object.key("roleArn").string(var_4);
    }
    if let Some(var_5) = &input.tags {
        let mut object_6 = object.key("tags").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7).string(value_8);
            }
        }
        object_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11).string(value_12);
            }
        }
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.application_description {
        object.key("applicationDescription").string(var_13);
    }
    if let Some(var_14) = &input.application_name {
        object.key("applicationName").string(var_14);
    }
    if let Some(var_15) = &input.client_token {
        object.key("clientToken").string(var_15);
    }
    Ok(())
}
