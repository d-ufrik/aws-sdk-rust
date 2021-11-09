// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_add_tags_to_vault(
    input: &crate::input::AddTagsToVaultInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_add_tags_to_vault_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn ser_payload_initiate_job_input(
    payload: &std::option::Option<crate::model::JobParameters>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_glacier_synthetic_initiate_job_input_job_parameters(&payload)?
    ))
}

pub fn ser_payload_initiate_vault_lock_input(
    payload: &std::option::Option<crate::model::VaultLockPolicy>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_glacier_synthetic_initiate_vault_lock_input_policy(&payload)?
    ))
}

pub fn serialize_operation_crate_operation_remove_tags_from_vault(
    input: &crate::input::RemoveTagsFromVaultInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_remove_tags_from_vault_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_set_data_retrieval_policy(
    input: &crate::input::SetDataRetrievalPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_set_data_retrieval_policy_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn ser_payload_set_vault_access_policy_input(
    payload: &std::option::Option<crate::model::VaultAccessPolicy>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_glacier_synthetic_set_vault_access_policy_input_policy(&payload)?
    ))
}

pub fn ser_payload_set_vault_notifications_input(
    payload: &std::option::Option<crate::model::VaultNotificationConfig>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_glacier_synthetic_set_vault_notifications_input_vault_notification_config(&payload)?
    ))
}

pub fn ser_payload_upload_archive_input(
    payload: aws_smithy_http::byte_stream::ByteStream,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    #[allow(clippy::useless_conversion)]
    Ok(aws_smithy_http::body::SdkBody::from(payload.into_inner()))
}

pub fn ser_payload_upload_multipart_part_input(
    payload: aws_smithy_http::byte_stream::ByteStream,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    #[allow(clippy::useless_conversion)]
    Ok(aws_smithy_http::body::SdkBody::from(payload.into_inner()))
}

pub fn rest_json_unsetpayload() -> std::vec::Vec<u8> {
    b"{}"[..].into()
}

pub fn serialize_member_com_amazonaws_glacier_synthetic_initiate_job_input_job_parameters(
    input: &crate::model::JobParameters,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_job_parameters(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_glacier_synthetic_initiate_vault_lock_input_policy(
    input: &crate::model::VaultLockPolicy,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_vault_lock_policy(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_glacier_synthetic_set_vault_access_policy_input_policy(
    input: &crate::model::VaultAccessPolicy,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_vault_access_policy(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_glacier_synthetic_set_vault_notifications_input_vault_notification_config(
    input: &crate::model::VaultNotificationConfig,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_vault_notification_config(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}
