// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_list_realtime_contact_analysis_segments(
    input: &crate::input::ListRealtimeContactAnalysisSegmentsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_realtime_contact_analysis_segments_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
