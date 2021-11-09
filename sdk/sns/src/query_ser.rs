// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_message_attribute_value(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MessageAttributeValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DataType");
    if let Some(var_6) = &input.data_type {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("StringValue");
    if let Some(var_8) = &input.string_value {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("BinaryValue");
    if let Some(var_10) = &input.binary_value {
        scope_9.string(&aws_smithy_types::base64::encode(var_10));
    }
    Ok(())
}
