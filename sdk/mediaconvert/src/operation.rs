// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateCertificate {
    _private: (),
}
impl AssociateCertificate {
    /// Creates a new builder-style object to manufacture [`AssociateCertificateInput`](crate::input::AssociateCertificateInput)
    pub fn builder() -> crate::input::associate_certificate_input::Builder {
        crate::input::associate_certificate_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateCertificate {
    type Output = std::result::Result<
        crate::output::AssociateCertificateOutput,
        crate::error::AssociateCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_associate_certificate_error(response)
        } else {
            crate::operation_deser::parse_associate_certificate_response(response)
        }
    }
}

/// Permanently cancel a job. Once you have canceled a job, you can't start it again.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJob {
    _private: (),
}
impl CancelJob {
    /// Creates a new builder-style object to manufacture [`CancelJobInput`](crate::input::CancelJobInput)
    pub fn builder() -> crate::input::cancel_job_input::Builder {
        crate::input::cancel_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CancelJob {
    type Output = std::result::Result<crate::output::CancelJobOutput, crate::error::CancelJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_cancel_job_error(response)
        } else {
            crate::operation_deser::parse_cancel_job_response(response)
        }
    }
}

/// Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateJob {
    _private: (),
}
impl CreateJob {
    /// Creates a new builder-style object to manufacture [`CreateJobInput`](crate::input::CreateJobInput)
    pub fn builder() -> crate::input::create_job_input::Builder {
        crate::input::create_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateJob {
    type Output = std::result::Result<crate::output::CreateJobOutput, crate::error::CreateJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_job_error(response)
        } else {
            crate::operation_deser::parse_create_job_response(response)
        }
    }
}

/// Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateJobTemplate {
    _private: (),
}
impl CreateJobTemplate {
    /// Creates a new builder-style object to manufacture [`CreateJobTemplateInput`](crate::input::CreateJobTemplateInput)
    pub fn builder() -> crate::input::create_job_template_input::Builder {
        crate::input::create_job_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateJobTemplate {
    type Output = std::result::Result<
        crate::output::CreateJobTemplateOutput,
        crate::error::CreateJobTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_job_template_error(response)
        } else {
            crate::operation_deser::parse_create_job_template_response(response)
        }
    }
}

/// Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePreset {
    _private: (),
}
impl CreatePreset {
    /// Creates a new builder-style object to manufacture [`CreatePresetInput`](crate::input::CreatePresetInput)
    pub fn builder() -> crate::input::create_preset_input::Builder {
        crate::input::create_preset_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreatePreset {
    type Output =
        std::result::Result<crate::output::CreatePresetOutput, crate::error::CreatePresetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_preset_error(response)
        } else {
            crate::operation_deser::parse_create_preset_response(response)
        }
    }
}

/// Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateQueue {
    _private: (),
}
impl CreateQueue {
    /// Creates a new builder-style object to manufacture [`CreateQueueInput`](crate::input::CreateQueueInput)
    pub fn builder() -> crate::input::create_queue_input::Builder {
        crate::input::create_queue_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateQueue {
    type Output =
        std::result::Result<crate::output::CreateQueueOutput, crate::error::CreateQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_queue_error(response)
        } else {
            crate::operation_deser::parse_create_queue_response(response)
        }
    }
}

/// Permanently delete a job template you have created.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteJobTemplate {
    _private: (),
}
impl DeleteJobTemplate {
    /// Creates a new builder-style object to manufacture [`DeleteJobTemplateInput`](crate::input::DeleteJobTemplateInput)
    pub fn builder() -> crate::input::delete_job_template_input::Builder {
        crate::input::delete_job_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteJobTemplate {
    type Output = std::result::Result<
        crate::output::DeleteJobTemplateOutput,
        crate::error::DeleteJobTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_job_template_error(response)
        } else {
            crate::operation_deser::parse_delete_job_template_response(response)
        }
    }
}

/// Permanently delete a policy that you created.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeletePolicy {
    _private: (),
}
impl DeletePolicy {
    /// Creates a new builder-style object to manufacture [`DeletePolicyInput`](crate::input::DeletePolicyInput)
    pub fn builder() -> crate::input::delete_policy_input::Builder {
        crate::input::delete_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeletePolicy {
    type Output =
        std::result::Result<crate::output::DeletePolicyOutput, crate::error::DeletePolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_policy_response(response)
        }
    }
}

/// Permanently delete a preset you have created.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeletePreset {
    _private: (),
}
impl DeletePreset {
    /// Creates a new builder-style object to manufacture [`DeletePresetInput`](crate::input::DeletePresetInput)
    pub fn builder() -> crate::input::delete_preset_input::Builder {
        crate::input::delete_preset_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeletePreset {
    type Output =
        std::result::Result<crate::output::DeletePresetOutput, crate::error::DeletePresetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_preset_error(response)
        } else {
            crate::operation_deser::parse_delete_preset_response(response)
        }
    }
}

/// Permanently delete a queue you have created.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteQueue {
    _private: (),
}
impl DeleteQueue {
    /// Creates a new builder-style object to manufacture [`DeleteQueueInput`](crate::input::DeleteQueueInput)
    pub fn builder() -> crate::input::delete_queue_input::Builder {
        crate::input::delete_queue_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteQueue {
    type Output =
        std::result::Result<crate::output::DeleteQueueOutput, crate::error::DeleteQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_queue_error(response)
        } else {
            crate::operation_deser::parse_delete_queue_response(response)
        }
    }
}

/// Send an request with an empty body to the regional API endpoint to get your account API endpoint.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEndpoints {
    _private: (),
}
impl DescribeEndpoints {
    /// Creates a new builder-style object to manufacture [`DescribeEndpointsInput`](crate::input::DescribeEndpointsInput)
    pub fn builder() -> crate::input::describe_endpoints_input::Builder {
        crate::input::describe_endpoints_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEndpoints {
    type Output = std::result::Result<
        crate::output::DescribeEndpointsOutput,
        crate::error::DescribeEndpointsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_endpoints_error(response)
        } else {
            crate::operation_deser::parse_describe_endpoints_response(response)
        }
    }
}

/// Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateCertificate {
    _private: (),
}
impl DisassociateCertificate {
    /// Creates a new builder-style object to manufacture [`DisassociateCertificateInput`](crate::input::DisassociateCertificateInput)
    pub fn builder() -> crate::input::disassociate_certificate_input::Builder {
        crate::input::disassociate_certificate_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateCertificate {
    type Output = std::result::Result<
        crate::output::DisassociateCertificateOutput,
        crate::error::DisassociateCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_disassociate_certificate_error(response)
        } else {
            crate::operation_deser::parse_disassociate_certificate_response(response)
        }
    }
}

/// Retrieve the JSON for a specific completed transcoding job.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetJob {
    _private: (),
}
impl GetJob {
    /// Creates a new builder-style object to manufacture [`GetJobInput`](crate::input::GetJobInput)
    pub fn builder() -> crate::input::get_job_input::Builder {
        crate::input::get_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetJob {
    type Output = std::result::Result<crate::output::GetJobOutput, crate::error::GetJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_job_error(response)
        } else {
            crate::operation_deser::parse_get_job_response(response)
        }
    }
}

/// Retrieve the JSON for a specific job template.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetJobTemplate {
    _private: (),
}
impl GetJobTemplate {
    /// Creates a new builder-style object to manufacture [`GetJobTemplateInput`](crate::input::GetJobTemplateInput)
    pub fn builder() -> crate::input::get_job_template_input::Builder {
        crate::input::get_job_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetJobTemplate {
    type Output =
        std::result::Result<crate::output::GetJobTemplateOutput, crate::error::GetJobTemplateError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_job_template_error(response)
        } else {
            crate::operation_deser::parse_get_job_template_response(response)
        }
    }
}

/// Retrieve the JSON for your policy.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPolicy {
    _private: (),
}
impl GetPolicy {
    /// Creates a new builder-style object to manufacture [`GetPolicyInput`](crate::input::GetPolicyInput)
    pub fn builder() -> crate::input::get_policy_input::Builder {
        crate::input::get_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetPolicy {
    type Output = std::result::Result<crate::output::GetPolicyOutput, crate::error::GetPolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_policy_error(response)
        } else {
            crate::operation_deser::parse_get_policy_response(response)
        }
    }
}

/// Retrieve the JSON for a specific preset.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPreset {
    _private: (),
}
impl GetPreset {
    /// Creates a new builder-style object to manufacture [`GetPresetInput`](crate::input::GetPresetInput)
    pub fn builder() -> crate::input::get_preset_input::Builder {
        crate::input::get_preset_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetPreset {
    type Output = std::result::Result<crate::output::GetPresetOutput, crate::error::GetPresetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_preset_error(response)
        } else {
            crate::operation_deser::parse_get_preset_response(response)
        }
    }
}

/// Retrieve the JSON for a specific queue.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetQueue {
    _private: (),
}
impl GetQueue {
    /// Creates a new builder-style object to manufacture [`GetQueueInput`](crate::input::GetQueueInput)
    pub fn builder() -> crate::input::get_queue_input::Builder {
        crate::input::get_queue_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetQueue {
    type Output = std::result::Result<crate::output::GetQueueOutput, crate::error::GetQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_queue_error(response)
        } else {
            crate::operation_deser::parse_get_queue_response(response)
        }
    }
}

/// Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJobs {
    _private: (),
}
impl ListJobs {
    /// Creates a new builder-style object to manufacture [`ListJobsInput`](crate::input::ListJobsInput)
    pub fn builder() -> crate::input::list_jobs_input::Builder {
        crate::input::list_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListJobs {
    type Output = std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_jobs_response(response)
        }
    }
}

/// Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJobTemplates {
    _private: (),
}
impl ListJobTemplates {
    /// Creates a new builder-style object to manufacture [`ListJobTemplatesInput`](crate::input::ListJobTemplatesInput)
    pub fn builder() -> crate::input::list_job_templates_input::Builder {
        crate::input::list_job_templates_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListJobTemplates {
    type Output = std::result::Result<
        crate::output::ListJobTemplatesOutput,
        crate::error::ListJobTemplatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_job_templates_error(response)
        } else {
            crate::operation_deser::parse_list_job_templates_response(response)
        }
    }
}

/// Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPresets {
    _private: (),
}
impl ListPresets {
    /// Creates a new builder-style object to manufacture [`ListPresetsInput`](crate::input::ListPresetsInput)
    pub fn builder() -> crate::input::list_presets_input::Builder {
        crate::input::list_presets_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPresets {
    type Output =
        std::result::Result<crate::output::ListPresetsOutput, crate::error::ListPresetsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_presets_error(response)
        } else {
            crate::operation_deser::parse_list_presets_response(response)
        }
    }
}

/// Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListQueues {
    _private: (),
}
impl ListQueues {
    /// Creates a new builder-style object to manufacture [`ListQueuesInput`](crate::input::ListQueuesInput)
    pub fn builder() -> crate::input::list_queues_input::Builder {
        crate::input::list_queues_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListQueues {
    type Output =
        std::result::Result<crate::output::ListQueuesOutput, crate::error::ListQueuesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_queues_error(response)
        } else {
            crate::operation_deser::parse_list_queues_response(response)
        }
    }
}

/// Retrieve the tags for a MediaConvert resource.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Create or change your policy. For more information about policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutPolicy {
    _private: (),
}
impl PutPolicy {
    /// Creates a new builder-style object to manufacture [`PutPolicyInput`](crate::input::PutPolicyInput)
    pub fn builder() -> crate::input::put_policy_input::Builder {
        crate::input::put_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutPolicy {
    type Output = std::result::Result<crate::output::PutPolicyOutput, crate::error::PutPolicyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_policy_error(response)
        } else {
            crate::operation_deser::parse_put_policy_response(response)
        }
    }
}

/// Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Modify one of your existing job templates.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateJobTemplate {
    _private: (),
}
impl UpdateJobTemplate {
    /// Creates a new builder-style object to manufacture [`UpdateJobTemplateInput`](crate::input::UpdateJobTemplateInput)
    pub fn builder() -> crate::input::update_job_template_input::Builder {
        crate::input::update_job_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateJobTemplate {
    type Output = std::result::Result<
        crate::output::UpdateJobTemplateOutput,
        crate::error::UpdateJobTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_job_template_error(response)
        } else {
            crate::operation_deser::parse_update_job_template_response(response)
        }
    }
}

/// Modify one of your existing presets.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePreset {
    _private: (),
}
impl UpdatePreset {
    /// Creates a new builder-style object to manufacture [`UpdatePresetInput`](crate::input::UpdatePresetInput)
    pub fn builder() -> crate::input::update_preset_input::Builder {
        crate::input::update_preset_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdatePreset {
    type Output =
        std::result::Result<crate::output::UpdatePresetOutput, crate::error::UpdatePresetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_preset_error(response)
        } else {
            crate::operation_deser::parse_update_preset_response(response)
        }
    }
}

/// Modify one of your existing queues.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateQueue {
    _private: (),
}
impl UpdateQueue {
    /// Creates a new builder-style object to manufacture [`UpdateQueueInput`](crate::input::UpdateQueueInput)
    pub fn builder() -> crate::input::update_queue_input::Builder {
        crate::input::update_queue_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateQueue {
    type Output =
        std::result::Result<crate::output::UpdateQueueOutput, crate::error::UpdateQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_queue_error(response)
        } else {
            crate::operation_deser::parse_update_queue_response(response)
        }
    }
}
