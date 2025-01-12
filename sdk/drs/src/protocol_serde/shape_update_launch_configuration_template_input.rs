// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_launch_configuration_template_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_launch_configuration_template::UpdateLaunchConfigurationTemplateInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.copy_private_ip {
        object.key("copyPrivateIp").boolean(*var_1);
    }
    if let Some(var_2) = &input.copy_tags {
        object.key("copyTags").boolean(*var_2);
    }
    if let Some(var_3) = &input.launch_configuration_template_id {
        object
            .key("launchConfigurationTemplateID")
            .string(var_3.as_str());
    }
    if let Some(var_4) = &input.launch_disposition {
        object.key("launchDisposition").string(var_4.as_str());
    }
    if let Some(var_5) = &input.licensing {
        #[allow(unused_mut)]
        let mut object_6 = object.key("licensing").start_object();
        crate::protocol_serde::shape_licensing::ser_licensing(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.target_instance_type_right_sizing_method {
        object
            .key("targetInstanceTypeRightSizingMethod")
            .string(var_7.as_str());
    }
    Ok(())
}
