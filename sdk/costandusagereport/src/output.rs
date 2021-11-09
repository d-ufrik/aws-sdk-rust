// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutReportDefinitionOutput {}
impl std::fmt::Debug for PutReportDefinitionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutReportDefinitionOutput");
        formatter.finish()
    }
}
/// See [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput)
pub mod put_report_definition_output {
    /// A builder for [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput)
        pub fn build(self) -> crate::output::PutReportDefinitionOutput {
            crate::output::PutReportDefinitionOutput {}
        }
    }
}
impl PutReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput)
    pub fn builder() -> crate::output::put_report_definition_output::Builder {
        crate::output::put_report_definition_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ModifyReportDefinitionOutput {}
impl std::fmt::Debug for ModifyReportDefinitionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ModifyReportDefinitionOutput");
        formatter.finish()
    }
}
/// See [`ModifyReportDefinitionOutput`](crate::output::ModifyReportDefinitionOutput)
pub mod modify_report_definition_output {
    /// A builder for [`ModifyReportDefinitionOutput`](crate::output::ModifyReportDefinitionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`ModifyReportDefinitionOutput`](crate::output::ModifyReportDefinitionOutput)
        pub fn build(self) -> crate::output::ModifyReportDefinitionOutput {
            crate::output::ModifyReportDefinitionOutput {}
        }
    }
}
impl ModifyReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`ModifyReportDefinitionOutput`](crate::output::ModifyReportDefinitionOutput)
    pub fn builder() -> crate::output::modify_report_definition_output::Builder {
        crate::output::modify_report_definition_output::Builder::default()
    }
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeReportDefinitionsOutput {
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    pub report_definitions: std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
    /// <p>A generic string.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeReportDefinitionsOutput {
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    pub fn report_definitions(&self) -> std::option::Option<&[crate::model::ReportDefinition]> {
        self.report_definitions.as_deref()
    }
    /// <p>A generic string.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeReportDefinitionsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeReportDefinitionsOutput");
        formatter.field("report_definitions", &self.report_definitions);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeReportDefinitionsOutput`](crate::output::DescribeReportDefinitionsOutput)
pub mod describe_report_definitions_output {
    /// A builder for [`DescribeReportDefinitionsOutput`](crate::output::DescribeReportDefinitionsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_definitions:
            std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `report_definitions`.
        ///
        /// To override the contents of this collection use [`set_report_definitions`](Self::set_report_definitions).
        ///
        /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
        pub fn report_definitions(
            mut self,
            input: impl Into<crate::model::ReportDefinition>,
        ) -> Self {
            let mut v = self.report_definitions.unwrap_or_default();
            v.push(input.into());
            self.report_definitions = Some(v);
            self
        }
        /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
        pub fn set_report_definitions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
        ) -> Self {
            self.report_definitions = input;
            self
        }
        /// <p>A generic string.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A generic string.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeReportDefinitionsOutput`](crate::output::DescribeReportDefinitionsOutput)
        pub fn build(self) -> crate::output::DescribeReportDefinitionsOutput {
            crate::output::DescribeReportDefinitionsOutput {
                report_definitions: self.report_definitions,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeReportDefinitionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeReportDefinitionsOutput`](crate::output::DescribeReportDefinitionsOutput)
    pub fn builder() -> crate::output::describe_report_definitions_output::Builder {
        crate::output::describe_report_definitions_output::Builder::default()
    }
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteReportDefinitionOutput {
    /// <p>Whether the deletion was successful or not.</p>
    pub response_message: std::option::Option<std::string::String>,
}
impl DeleteReportDefinitionOutput {
    /// <p>Whether the deletion was successful or not.</p>
    pub fn response_message(&self) -> std::option::Option<&str> {
        self.response_message.as_deref()
    }
}
impl std::fmt::Debug for DeleteReportDefinitionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteReportDefinitionOutput");
        formatter.field("response_message", &self.response_message);
        formatter.finish()
    }
}
/// See [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput)
pub mod delete_report_definition_output {
    /// A builder for [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) response_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Whether the deletion was successful or not.</p>
        pub fn response_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.response_message = Some(input.into());
            self
        }
        /// <p>Whether the deletion was successful or not.</p>
        pub fn set_response_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.response_message = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput)
        pub fn build(self) -> crate::output::DeleteReportDefinitionOutput {
            crate::output::DeleteReportDefinitionOutput {
                response_message: self.response_message,
            }
        }
    }
}
impl DeleteReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput)
    pub fn builder() -> crate::output::delete_report_definition_output::Builder {
        crate::output::delete_report_definition_output::Builder::default()
    }
}
