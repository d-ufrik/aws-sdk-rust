// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_renderable_task(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RenderableTask,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.input {
        object.key("Input").string(var_1.as_str());
    }
    Ok(())
}
