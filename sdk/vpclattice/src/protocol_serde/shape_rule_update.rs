// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RuleUpdate,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rule_identifier {
        object.key("ruleIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#match {
        #[allow(unused_mut)]
        let mut object_3 = object.key("match").start_object();
        crate::protocol_serde::shape_rule_match::ser_rule_match(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.action {
        #[allow(unused_mut)]
        let mut object_6 = object.key("action").start_object();
        crate::protocol_serde::shape_rule_action::ser_rule_action(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
