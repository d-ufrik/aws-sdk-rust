// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_get_clip(
    input: &crate::input::GetClipInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_clip_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_dash_streaming_session_url(
    input: &crate::input::GetDashStreamingSessionUrlInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_dash_streaming_session_url_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_hls_streaming_session_url(
    input: &crate::input::GetHlsStreamingSessionUrlInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_hls_streaming_session_url_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_media_for_fragment_list(
    input: &crate::input::GetMediaForFragmentListInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_media_for_fragment_list_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_fragments(
    input: &crate::input::ListFragmentsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_fragments_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
