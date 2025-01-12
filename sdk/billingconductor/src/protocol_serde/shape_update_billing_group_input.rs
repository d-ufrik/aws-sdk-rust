// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_billing_group_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_billing_group::UpdateBillingGroupInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.computation_preference {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ComputationPreference").start_object();
        crate::protocol_serde::shape_computation_preference::ser_computation_preference(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("Name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.status {
        object.key("Status").string(var_6.as_str());
    }
    Ok(())
}
