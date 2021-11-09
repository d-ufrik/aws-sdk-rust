// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_send_serial_console_ssh_public_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendSerialConsoleSshPublicKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.instance_id {
        object.key("InstanceId").string(var_1);
    }
    if input.serial_port != 0 {
        object.key("SerialPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.serial_port).into()),
        );
    }
    if let Some(var_2) = &input.ssh_public_key {
        object.key("SSHPublicKey").string(var_2);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_send_ssh_public_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendSshPublicKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_3) = &input.instance_id {
        object.key("InstanceId").string(var_3);
    }
    if let Some(var_4) = &input.instance_os_user {
        object.key("InstanceOSUser").string(var_4);
    }
    if let Some(var_5) = &input.ssh_public_key {
        object.key("SSHPublicKey").string(var_5);
    }
    if let Some(var_6) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_6);
    }
    Ok(())
}
