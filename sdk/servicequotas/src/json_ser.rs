// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_delete_service_quota_increase_request_from_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteServiceQuotaIncreaseRequestFromTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.service_code {
        object.key("ServiceCode").string(var_1);
    }
    if let Some(var_2) = &input.quota_code {
        object.key("QuotaCode").string(var_2);
    }
    if let Some(var_3) = &input.aws_region {
        object.key("AwsRegion").string(var_3);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_aws_default_service_quota_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAwsDefaultServiceQuotaInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.service_code {
        object.key("ServiceCode").string(var_4);
    }
    if let Some(var_5) = &input.quota_code {
        object.key("QuotaCode").string(var_5);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_requested_service_quota_change_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRequestedServiceQuotaChangeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.request_id {
        object.key("RequestId").string(var_6);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_service_quota_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceQuotaInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.service_code {
        object.key("ServiceCode").string(var_7);
    }
    if let Some(var_8) = &input.quota_code {
        object.key("QuotaCode").string(var_8);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_service_quota_increase_request_from_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceQuotaIncreaseRequestFromTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.service_code {
        object.key("ServiceCode").string(var_9);
    }
    if let Some(var_10) = &input.quota_code {
        object.key("QuotaCode").string(var_10);
    }
    if let Some(var_11) = &input.aws_region {
        object.key("AwsRegion").string(var_11);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_aws_default_service_quotas_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAwsDefaultServiceQuotasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.service_code {
        object.key("ServiceCode").string(var_12);
    }
    if let Some(var_13) = &input.next_token {
        object.key("NextToken").string(var_13);
    }
    if let Some(var_14) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_requested_service_quota_change_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRequestedServiceQuotaChangeHistoryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.service_code {
        object.key("ServiceCode").string(var_15);
    }
    if let Some(var_16) = &input.status {
        object.key("Status").string(var_16.as_str());
    }
    if let Some(var_17) = &input.next_token {
        object.key("NextToken").string(var_17);
    }
    if let Some(var_18) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_requested_service_quota_change_history_by_quota_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.service_code {
        object.key("ServiceCode").string(var_19);
    }
    if let Some(var_20) = &input.quota_code {
        object.key("QuotaCode").string(var_20);
    }
    if let Some(var_21) = &input.status {
        object.key("Status").string(var_21.as_str());
    }
    if let Some(var_22) = &input.next_token {
        object.key("NextToken").string(var_22);
    }
    if let Some(var_23) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_service_quota_increase_requests_in_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServiceQuotaIncreaseRequestsInTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.service_code {
        object.key("ServiceCode").string(var_24);
    }
    if let Some(var_25) = &input.aws_region {
        object.key("AwsRegion").string(var_25);
    }
    if let Some(var_26) = &input.next_token {
        object.key("NextToken").string(var_26);
    }
    if let Some(var_27) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_service_quotas_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServiceQuotasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.service_code {
        object.key("ServiceCode").string(var_28);
    }
    if let Some(var_29) = &input.next_token {
        object.key("NextToken").string(var_29);
    }
    if let Some(var_30) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_30).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_services_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServicesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.next_token {
        object.key("NextToken").string(var_31);
    }
    if let Some(var_32) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.resource_arn {
        object.key("ResourceARN").string(var_33);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_service_quota_increase_request_into_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutServiceQuotaIncreaseRequestIntoTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.quota_code {
        object.key("QuotaCode").string(var_34);
    }
    if let Some(var_35) = &input.service_code {
        object.key("ServiceCode").string(var_35);
    }
    if let Some(var_36) = &input.aws_region {
        object.key("AwsRegion").string(var_36);
    }
    if let Some(var_37) = &input.desired_value {
        object.key("DesiredValue").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_37).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_request_service_quota_increase_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RequestServiceQuotaIncreaseInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.service_code {
        object.key("ServiceCode").string(var_38);
    }
    if let Some(var_39) = &input.quota_code {
        object.key("QuotaCode").string(var_39);
    }
    if let Some(var_40) = &input.desired_value {
        object.key("DesiredValue").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_40).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.resource_arn {
        object.key("ResourceARN").string(var_41);
    }
    if let Some(var_42) = &input.tags {
        let mut array_43 = object.key("Tags").start_array();
        for item_44 in var_42 {
            {
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_45, item_44)?;
                object_45.finish();
            }
        }
        array_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.resource_arn {
        object.key("ResourceARN").string(var_46);
    }
    if let Some(var_47) = &input.tag_keys {
        let mut array_48 = object.key("TagKeys").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49);
            }
        }
        array_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.key {
        object.key("Key").string(var_50);
    }
    if let Some(var_51) = &input.value {
        object.key("Value").string(var_51);
    }
    Ok(())
}
