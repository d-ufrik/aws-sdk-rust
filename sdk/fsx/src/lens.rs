// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_backups_output_next_token(
    input: &crate::output::DescribeBackupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_data_repository_associations_output_next_token(
    input: &crate::output::DescribeDataRepositoryAssociationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_data_repository_tasks_output_next_token(
    input: &crate::output::DescribeDataRepositoryTasksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_file_systems_output_next_token(
    input: &crate::output::DescribeFileSystemsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_snapshots_output_next_token(
    input: &crate::output::DescribeSnapshotsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_storage_virtual_machines_output_next_token(
    input: &crate::output::DescribeStorageVirtualMachinesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_volumes_output_next_token(
    input: &crate::output::DescribeVolumesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tags_for_resource_output_next_token(
    input: &crate::output::ListTagsForResourceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_storage_virtual_machines_output_storage_virtual_machines(
    input: crate::output::DescribeStorageVirtualMachinesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StorageVirtualMachine>> {
    let input = match input.storage_virtual_machines {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_volumes_output_volumes(
    input: crate::output::DescribeVolumesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Volume>> {
    let input = match input.volumes {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
