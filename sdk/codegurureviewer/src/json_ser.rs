// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_repository_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateRepositoryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1);
    }
    if let Some(var_2) = &input.kms_key_details {
        let mut object_3 = object.key("KMSKeyDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_kms_key_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.repository {
        let mut object_5 = object.key("Repository").start_object();
        crate::json_ser::serialize_structure_crate_model_repository(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9);
            }
        }
        object_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_code_review_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCodeReviewInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_10);
    }
    if let Some(var_11) = &input.name {
        object.key("Name").string(var_11);
    }
    if let Some(var_12) = &input.repository_association_arn {
        object.key("RepositoryAssociationArn").string(var_12);
    }
    if let Some(var_13) = &input.r#type {
        let mut object_14 = object.key("Type").start_object();
        crate::json_ser::serialize_structure_crate_model_code_review_type(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_recommendation_feedback_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRecommendationFeedbackInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.code_review_arn {
        object.key("CodeReviewArn").string(var_15);
    }
    if let Some(var_16) = &input.reactions {
        let mut array_17 = object.key("Reactions").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.recommendation_id {
        object.key("RecommendationId").string(var_19);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.tags {
        let mut object_21 = object.key("Tags").start_object();
        for (key_22, value_23) in var_20 {
            {
                object_21.key(key_22).string(value_23);
            }
        }
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_kms_key_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KmsKeyDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_24);
    }
    if let Some(var_25) = &input.encryption_option {
        object.key("EncryptionOption").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_repository(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Repository,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.code_commit {
        let mut object_27 = object.key("CodeCommit").start_object();
        crate::json_ser::serialize_structure_crate_model_code_commit_repository(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.bitbucket {
        let mut object_29 = object.key("Bitbucket").start_object();
        crate::json_ser::serialize_structure_crate_model_third_party_source_repository(
            &mut object_29,
            var_28,
        )?;
        object_29.finish();
    }
    if let Some(var_30) = &input.git_hub_enterprise_server {
        let mut object_31 = object.key("GitHubEnterpriseServer").start_object();
        crate::json_ser::serialize_structure_crate_model_third_party_source_repository(
            &mut object_31,
            var_30,
        )?;
        object_31.finish();
    }
    if let Some(var_32) = &input.s3_bucket {
        let mut object_33 = object.key("S3Bucket").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_repository(&mut object_33, var_32)?;
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_review_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeReviewType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.repository_analysis {
        let mut object_35 = object.key("RepositoryAnalysis").start_object();
        crate::json_ser::serialize_structure_crate_model_repository_analysis(
            &mut object_35,
            var_34,
        )?;
        object_35.finish();
    }
    if let Some(var_36) = &input.analysis_types {
        let mut array_37 = object.key("AnalysisTypes").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_commit_repository(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeCommitRepository,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.name {
        object.key("Name").string(var_39);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_third_party_source_repository(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ThirdPartySourceRepository,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.name {
        object.key("Name").string(var_40);
    }
    if let Some(var_41) = &input.connection_arn {
        object.key("ConnectionArn").string(var_41);
    }
    if let Some(var_42) = &input.owner {
        object.key("Owner").string(var_42);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_repository(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Repository,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.name {
        object.key("Name").string(var_43);
    }
    if let Some(var_44) = &input.bucket_name {
        object.key("BucketName").string(var_44);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_repository_analysis(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RepositoryAnalysis,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.repository_head {
        let mut object_46 = object.key("RepositoryHead").start_object();
        crate::json_ser::serialize_structure_crate_model_repository_head_source_code_type(
            &mut object_46,
            var_45,
        )?;
        object_46.finish();
    }
    if let Some(var_47) = &input.source_code_type {
        let mut object_48 = object.key("SourceCodeType").start_object();
        crate::json_ser::serialize_structure_crate_model_source_code_type(&mut object_48, var_47)?;
        object_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_repository_head_source_code_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RepositoryHeadSourceCodeType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.branch_name {
        object.key("BranchName").string(var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_code_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceCodeType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.commit_diff {
        let mut object_51 = object.key("CommitDiff").start_object();
        crate::json_ser::serialize_structure_crate_model_commit_diff_source_code_type(
            &mut object_51,
            var_50,
        )?;
        object_51.finish();
    }
    if let Some(var_52) = &input.repository_head {
        let mut object_53 = object.key("RepositoryHead").start_object();
        crate::json_ser::serialize_structure_crate_model_repository_head_source_code_type(
            &mut object_53,
            var_52,
        )?;
        object_53.finish();
    }
    if let Some(var_54) = &input.branch_diff {
        let mut object_55 = object.key("BranchDiff").start_object();
        crate::json_ser::serialize_structure_crate_model_branch_diff_source_code_type(
            &mut object_55,
            var_54,
        )?;
        object_55.finish();
    }
    if let Some(var_56) = &input.s3_bucket_repository {
        let mut object_57 = object.key("S3BucketRepository").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_bucket_repository(
            &mut object_57,
            var_56,
        )?;
        object_57.finish();
    }
    if let Some(var_58) = &input.request_metadata {
        let mut object_59 = object.key("RequestMetadata").start_object();
        crate::json_ser::serialize_structure_crate_model_request_metadata(&mut object_59, var_58)?;
        object_59.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_commit_diff_source_code_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CommitDiffSourceCodeType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.source_commit {
        object.key("SourceCommit").string(var_60);
    }
    if let Some(var_61) = &input.destination_commit {
        object.key("DestinationCommit").string(var_61);
    }
    if let Some(var_62) = &input.merge_base_commit {
        object.key("MergeBaseCommit").string(var_62);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_branch_diff_source_code_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BranchDiffSourceCodeType,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.source_branch_name {
        object.key("SourceBranchName").string(var_63);
    }
    if let Some(var_64) = &input.destination_branch_name {
        object.key("DestinationBranchName").string(var_64);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_bucket_repository(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3BucketRepository,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.name {
        object.key("Name").string(var_65);
    }
    if let Some(var_66) = &input.details {
        let mut object_67 = object.key("Details").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_repository_details(
            &mut object_67,
            var_66,
        )?;
        object_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_request_metadata(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestMetadata,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.request_id {
        object.key("RequestId").string(var_68);
    }
    if let Some(var_69) = &input.requester {
        object.key("Requester").string(var_69);
    }
    if let Some(var_70) = &input.event_info {
        let mut object_71 = object.key("EventInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_event_info(&mut object_71, var_70)?;
        object_71.finish();
    }
    if let Some(var_72) = &input.vendor_name {
        object.key("VendorName").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_repository_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3RepositoryDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.bucket_name {
        object.key("BucketName").string(var_73);
    }
    if let Some(var_74) = &input.code_artifacts {
        let mut object_75 = object.key("CodeArtifacts").start_object();
        crate::json_ser::serialize_structure_crate_model_code_artifacts(&mut object_75, var_74)?;
        object_75.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.name {
        object.key("Name").string(var_76);
    }
    if let Some(var_77) = &input.state {
        object.key("State").string(var_77);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_artifacts(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeArtifacts,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.source_code_artifacts_object_key {
        object.key("SourceCodeArtifactsObjectKey").string(var_78);
    }
    if let Some(var_79) = &input.build_artifacts_object_key {
        object.key("BuildArtifactsObjectKey").string(var_79);
    }
    Ok(())
}
