// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_domains_output_next_token(
    input: &crate::output::ListDomainsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_fraudster_registration_jobs_output_next_token(
    input: &crate::output::ListFraudsterRegistrationJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_speaker_enrollment_jobs_output_next_token(
    input: &crate::output::ListSpeakerEnrollmentJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_speakers_output_next_token(
    input: &crate::output::ListSpeakersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_domains_output_domain_summaries(
    input: crate::output::ListDomainsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DomainSummary>> {
    let input = match input.domain_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_fraudster_registration_jobs_output_job_summaries(
    input: crate::output::ListFraudsterRegistrationJobsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::FraudsterRegistrationJobSummary>> {
    let input = match input.job_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_speaker_enrollment_jobs_output_job_summaries(
    input: crate::output::ListSpeakerEnrollmentJobsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SpeakerEnrollmentJobSummary>> {
    let input = match input.job_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_speakers_output_speaker_summaries(
    input: crate::output::ListSpeakersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SpeakerSummary>> {
    let input = match input.speaker_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
