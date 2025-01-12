// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetFinding`](crate::operation::get_finding::builders::GetFindingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`analyzer_arn(impl ::std::convert::Into<String>)`](crate::operation::get_finding::builders::GetFindingFluentBuilder::analyzer_arn) / [`set_analyzer_arn(Option<String>)`](crate::operation::get_finding::builders::GetFindingFluentBuilder::set_analyzer_arn): <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> that generated the finding.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_finding::builders::GetFindingFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_finding::builders::GetFindingFluentBuilder::set_id): <p>The ID of the finding to retrieve.</p>
    /// - On success, responds with [`GetFindingOutput`](crate::operation::get_finding::GetFindingOutput) with field(s):
    ///   - [`finding(Option<Finding>)`](crate::operation::get_finding::GetFindingOutput::finding): <p>A <code>finding</code> object that contains finding details.</p>
    /// - On failure, responds with [`SdkError<GetFindingError>`](crate::operation::get_finding::GetFindingError)
    pub fn get_finding(&self) -> crate::operation::get_finding::builders::GetFindingFluentBuilder {
        crate::operation::get_finding::builders::GetFindingFluentBuilder::new(self.handle.clone())
    }
}
