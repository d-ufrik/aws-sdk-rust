// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Includes information about a quantum task.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct QuantumTaskSummary {
    /// <p>The ARN of the task.</p>
    pub quantum_task_arn: std::option::Option<std::string::String>,
    /// <p>The status of the task.</p>
    pub status: std::option::Option<crate::model::QuantumTaskStatus>,
    /// <p>The ARN of the device the task ran on.</p>
    pub device_arn: std::option::Option<std::string::String>,
    /// <p>The shots used for the task.</p>
    pub shots: std::option::Option<i64>,
    /// <p>The S3 bucket where the task result file is stored..</p>
    pub output_s3_bucket: std::option::Option<std::string::String>,
    /// <p>The folder in the S3 bucket where the task result file is stored.</p>
    pub output_s3_directory: std::option::Option<std::string::String>,
    /// <p>The time at which the task was created.</p>
    pub created_at: std::option::Option<aws_smithy_types::Instant>,
    /// <p>The time at which the task finished.</p>
    pub ended_at: std::option::Option<aws_smithy_types::Instant>,
    /// <p>Displays the key, value pairs of tags associated with this quantum task.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl QuantumTaskSummary {
    /// <p>The ARN of the task.</p>
    pub fn quantum_task_arn(&self) -> std::option::Option<&str> {
        self.quantum_task_arn.as_deref()
    }
    /// <p>The status of the task.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::QuantumTaskStatus> {
        self.status.as_ref()
    }
    /// <p>The ARN of the device the task ran on.</p>
    pub fn device_arn(&self) -> std::option::Option<&str> {
        self.device_arn.as_deref()
    }
    /// <p>The shots used for the task.</p>
    pub fn shots(&self) -> std::option::Option<i64> {
        self.shots
    }
    /// <p>The S3 bucket where the task result file is stored..</p>
    pub fn output_s3_bucket(&self) -> std::option::Option<&str> {
        self.output_s3_bucket.as_deref()
    }
    /// <p>The folder in the S3 bucket where the task result file is stored.</p>
    pub fn output_s3_directory(&self) -> std::option::Option<&str> {
        self.output_s3_directory.as_deref()
    }
    /// <p>The time at which the task was created.</p>
    pub fn created_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.created_at.as_ref()
    }
    /// <p>The time at which the task finished.</p>
    pub fn ended_at(&self) -> std::option::Option<&aws_smithy_types::Instant> {
        self.ended_at.as_ref()
    }
    /// <p>Displays the key, value pairs of tags associated with this quantum task.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for QuantumTaskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("QuantumTaskSummary");
        formatter.field("quantum_task_arn", &self.quantum_task_arn);
        formatter.field("status", &self.status);
        formatter.field("device_arn", &self.device_arn);
        formatter.field("shots", &self.shots);
        formatter.field("output_s3_bucket", &self.output_s3_bucket);
        formatter.field("output_s3_directory", &self.output_s3_directory);
        formatter.field("created_at", &self.created_at);
        formatter.field("ended_at", &self.ended_at);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`QuantumTaskSummary`](crate::model::QuantumTaskSummary)
pub mod quantum_task_summary {
    /// A builder for [`QuantumTaskSummary`](crate::model::QuantumTaskSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) quantum_task_arn: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::QuantumTaskStatus>,
        pub(crate) device_arn: std::option::Option<std::string::String>,
        pub(crate) shots: std::option::Option<i64>,
        pub(crate) output_s3_bucket: std::option::Option<std::string::String>,
        pub(crate) output_s3_directory: std::option::Option<std::string::String>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) ended_at: std::option::Option<aws_smithy_types::Instant>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The ARN of the task.</p>
        pub fn quantum_task_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.quantum_task_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the task.</p>
        pub fn set_quantum_task_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.quantum_task_arn = input;
            self
        }
        /// <p>The status of the task.</p>
        pub fn status(mut self, input: crate::model::QuantumTaskStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the task.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::QuantumTaskStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>The ARN of the device the task ran on.</p>
        pub fn device_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the device the task ran on.</p>
        pub fn set_device_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_arn = input;
            self
        }
        /// <p>The shots used for the task.</p>
        pub fn shots(mut self, input: i64) -> Self {
            self.shots = Some(input);
            self
        }
        /// <p>The shots used for the task.</p>
        pub fn set_shots(mut self, input: std::option::Option<i64>) -> Self {
            self.shots = input;
            self
        }
        /// <p>The S3 bucket where the task result file is stored..</p>
        pub fn output_s3_bucket(mut self, input: impl Into<std::string::String>) -> Self {
            self.output_s3_bucket = Some(input.into());
            self
        }
        /// <p>The S3 bucket where the task result file is stored..</p>
        pub fn set_output_s3_bucket(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.output_s3_bucket = input;
            self
        }
        /// <p>The folder in the S3 bucket where the task result file is stored.</p>
        pub fn output_s3_directory(mut self, input: impl Into<std::string::String>) -> Self {
            self.output_s3_directory = Some(input.into());
            self
        }
        /// <p>The folder in the S3 bucket where the task result file is stored.</p>
        pub fn set_output_s3_directory(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.output_s3_directory = input;
            self
        }
        /// <p>The time at which the task was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>The time at which the task was created.</p>
        pub fn set_created_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.created_at = input;
            self
        }
        /// <p>The time at which the task finished.</p>
        pub fn ended_at(mut self, input: aws_smithy_types::Instant) -> Self {
            self.ended_at = Some(input);
            self
        }
        /// <p>The time at which the task finished.</p>
        pub fn set_ended_at(
            mut self,
            input: std::option::Option<aws_smithy_types::Instant>,
        ) -> Self {
            self.ended_at = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Displays the key, value pairs of tags associated with this quantum task.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>Displays the key, value pairs of tags associated with this quantum task.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`QuantumTaskSummary`](crate::model::QuantumTaskSummary)
        pub fn build(self) -> crate::model::QuantumTaskSummary {
            crate::model::QuantumTaskSummary {
                quantum_task_arn: self.quantum_task_arn,
                status: self.status,
                device_arn: self.device_arn,
                shots: self.shots,
                output_s3_bucket: self.output_s3_bucket,
                output_s3_directory: self.output_s3_directory,
                created_at: self.created_at,
                ended_at: self.ended_at,
                tags: self.tags,
            }
        }
    }
}
impl QuantumTaskSummary {
    /// Creates a new builder-style object to manufacture [`QuantumTaskSummary`](crate::model::QuantumTaskSummary)
    pub fn builder() -> crate::model::quantum_task_summary::Builder {
        crate::model::quantum_task_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum QuantumTaskStatus {
    #[allow(missing_docs)] // documentation missing in model
    Cancelled,
    #[allow(missing_docs)] // documentation missing in model
    Cancelling,
    #[allow(missing_docs)] // documentation missing in model
    Completed,
    #[allow(missing_docs)] // documentation missing in model
    Created,
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    Queued,
    #[allow(missing_docs)] // documentation missing in model
    Running,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for QuantumTaskStatus {
    fn from(s: &str) -> Self {
        match s {
            "CANCELLED" => QuantumTaskStatus::Cancelled,
            "CANCELLING" => QuantumTaskStatus::Cancelling,
            "COMPLETED" => QuantumTaskStatus::Completed,
            "CREATED" => QuantumTaskStatus::Created,
            "FAILED" => QuantumTaskStatus::Failed,
            "QUEUED" => QuantumTaskStatus::Queued,
            "RUNNING" => QuantumTaskStatus::Running,
            other => QuantumTaskStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for QuantumTaskStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(QuantumTaskStatus::from(s))
    }
}
impl QuantumTaskStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            QuantumTaskStatus::Cancelled => "CANCELLED",
            QuantumTaskStatus::Cancelling => "CANCELLING",
            QuantumTaskStatus::Completed => "COMPLETED",
            QuantumTaskStatus::Created => "CREATED",
            QuantumTaskStatus::Failed => "FAILED",
            QuantumTaskStatus::Queued => "QUEUED",
            QuantumTaskStatus::Running => "RUNNING",
            QuantumTaskStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "CANCELLED",
            "CANCELLING",
            "COMPLETED",
            "CREATED",
            "FAILED",
            "QUEUED",
            "RUNNING",
        ]
    }
}
impl AsRef<str> for QuantumTaskStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A filter to use to search for tasks.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SearchQuantumTasksFilter {
    /// <p>The name of the device used for the task.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The values to use for the filter.</p>
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>An operator to use in the filter.</p>
    pub operator: std::option::Option<crate::model::SearchQuantumTasksFilterOperator>,
}
impl SearchQuantumTasksFilter {
    /// <p>The name of the device used for the task.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The values to use for the filter.</p>
    pub fn values(&self) -> std::option::Option<&[std::string::String]> {
        self.values.as_deref()
    }
    /// <p>An operator to use in the filter.</p>
    pub fn operator(&self) -> std::option::Option<&crate::model::SearchQuantumTasksFilterOperator> {
        self.operator.as_ref()
    }
}
impl std::fmt::Debug for SearchQuantumTasksFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchQuantumTasksFilter");
        formatter.field("name", &self.name);
        formatter.field("values", &self.values);
        formatter.field("operator", &self.operator);
        formatter.finish()
    }
}
/// See [`SearchQuantumTasksFilter`](crate::model::SearchQuantumTasksFilter)
pub mod search_quantum_tasks_filter {
    /// A builder for [`SearchQuantumTasksFilter`](crate::model::SearchQuantumTasksFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) values: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) operator: std::option::Option<crate::model::SearchQuantumTasksFilterOperator>,
    }
    impl Builder {
        /// <p>The name of the device used for the task.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the device used for the task.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Appends an item to `values`.
        ///
        /// To override the contents of this collection use [`set_values`](Self::set_values).
        ///
        /// <p>The values to use for the filter.</p>
        pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
            v.push(input.into());
            self.values = Some(v);
            self
        }
        /// <p>The values to use for the filter.</p>
        pub fn set_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.values = input;
            self
        }
        /// <p>An operator to use in the filter.</p>
        pub fn operator(mut self, input: crate::model::SearchQuantumTasksFilterOperator) -> Self {
            self.operator = Some(input);
            self
        }
        /// <p>An operator to use in the filter.</p>
        pub fn set_operator(
            mut self,
            input: std::option::Option<crate::model::SearchQuantumTasksFilterOperator>,
        ) -> Self {
            self.operator = input;
            self
        }
        /// Consumes the builder and constructs a [`SearchQuantumTasksFilter`](crate::model::SearchQuantumTasksFilter)
        pub fn build(self) -> crate::model::SearchQuantumTasksFilter {
            crate::model::SearchQuantumTasksFilter {
                name: self.name,
                values: self.values,
                operator: self.operator,
            }
        }
    }
}
impl SearchQuantumTasksFilter {
    /// Creates a new builder-style object to manufacture [`SearchQuantumTasksFilter`](crate::model::SearchQuantumTasksFilter)
    pub fn builder() -> crate::model::search_quantum_tasks_filter::Builder {
        crate::model::search_quantum_tasks_filter::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SearchQuantumTasksFilterOperator {
    #[allow(missing_docs)] // documentation missing in model
    Between,
    #[allow(missing_docs)] // documentation missing in model
    Equal,
    #[allow(missing_docs)] // documentation missing in model
    Gt,
    #[allow(missing_docs)] // documentation missing in model
    Gte,
    #[allow(missing_docs)] // documentation missing in model
    Lt,
    #[allow(missing_docs)] // documentation missing in model
    Lte,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SearchQuantumTasksFilterOperator {
    fn from(s: &str) -> Self {
        match s {
            "BETWEEN" => SearchQuantumTasksFilterOperator::Between,
            "EQUAL" => SearchQuantumTasksFilterOperator::Equal,
            "GT" => SearchQuantumTasksFilterOperator::Gt,
            "GTE" => SearchQuantumTasksFilterOperator::Gte,
            "LT" => SearchQuantumTasksFilterOperator::Lt,
            "LTE" => SearchQuantumTasksFilterOperator::Lte,
            other => SearchQuantumTasksFilterOperator::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SearchQuantumTasksFilterOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SearchQuantumTasksFilterOperator::from(s))
    }
}
impl SearchQuantumTasksFilterOperator {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SearchQuantumTasksFilterOperator::Between => "BETWEEN",
            SearchQuantumTasksFilterOperator::Equal => "EQUAL",
            SearchQuantumTasksFilterOperator::Gt => "GT",
            SearchQuantumTasksFilterOperator::Gte => "GTE",
            SearchQuantumTasksFilterOperator::Lt => "LT",
            SearchQuantumTasksFilterOperator::Lte => "LTE",
            SearchQuantumTasksFilterOperator::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["BETWEEN", "EQUAL", "GT", "GTE", "LT", "LTE"]
    }
}
impl AsRef<str> for SearchQuantumTasksFilterOperator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum CancellationStatus {
    #[allow(missing_docs)] // documentation missing in model
    Cancelled,
    #[allow(missing_docs)] // documentation missing in model
    Cancelling,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for CancellationStatus {
    fn from(s: &str) -> Self {
        match s {
            "CANCELLED" => CancellationStatus::Cancelled,
            "CANCELLING" => CancellationStatus::Cancelling,
            other => CancellationStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for CancellationStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CancellationStatus::from(s))
    }
}
impl CancellationStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CancellationStatus::Cancelled => "CANCELLED",
            CancellationStatus::Cancelling => "CANCELLING",
            CancellationStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CANCELLED", "CANCELLING"]
    }
}
impl AsRef<str> for CancellationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Includes information about the device.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeviceSummary {
    /// <p>The ARN of the device.</p>
    pub device_arn: std::option::Option<std::string::String>,
    /// <p>The name of the device.</p>
    pub device_name: std::option::Option<std::string::String>,
    /// <p>The provider of the device.</p>
    pub provider_name: std::option::Option<std::string::String>,
    /// <p>The type of the device.</p>
    pub device_type: std::option::Option<crate::model::DeviceType>,
    /// <p>The status of the device.</p>
    pub device_status: std::option::Option<crate::model::DeviceStatus>,
}
impl DeviceSummary {
    /// <p>The ARN of the device.</p>
    pub fn device_arn(&self) -> std::option::Option<&str> {
        self.device_arn.as_deref()
    }
    /// <p>The name of the device.</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The provider of the device.</p>
    pub fn provider_name(&self) -> std::option::Option<&str> {
        self.provider_name.as_deref()
    }
    /// <p>The type of the device.</p>
    pub fn device_type(&self) -> std::option::Option<&crate::model::DeviceType> {
        self.device_type.as_ref()
    }
    /// <p>The status of the device.</p>
    pub fn device_status(&self) -> std::option::Option<&crate::model::DeviceStatus> {
        self.device_status.as_ref()
    }
}
impl std::fmt::Debug for DeviceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeviceSummary");
        formatter.field("device_arn", &self.device_arn);
        formatter.field("device_name", &self.device_name);
        formatter.field("provider_name", &self.provider_name);
        formatter.field("device_type", &self.device_type);
        formatter.field("device_status", &self.device_status);
        formatter.finish()
    }
}
/// See [`DeviceSummary`](crate::model::DeviceSummary)
pub mod device_summary {
    /// A builder for [`DeviceSummary`](crate::model::DeviceSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_arn: std::option::Option<std::string::String>,
        pub(crate) device_name: std::option::Option<std::string::String>,
        pub(crate) provider_name: std::option::Option<std::string::String>,
        pub(crate) device_type: std::option::Option<crate::model::DeviceType>,
        pub(crate) device_status: std::option::Option<crate::model::DeviceStatus>,
    }
    impl Builder {
        /// <p>The ARN of the device.</p>
        pub fn device_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the device.</p>
        pub fn set_device_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_arn = input;
            self
        }
        /// <p>The name of the device.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_name = Some(input.into());
            self
        }
        /// <p>The name of the device.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_name = input;
            self
        }
        /// <p>The provider of the device.</p>
        pub fn provider_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.provider_name = Some(input.into());
            self
        }
        /// <p>The provider of the device.</p>
        pub fn set_provider_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.provider_name = input;
            self
        }
        /// <p>The type of the device.</p>
        pub fn device_type(mut self, input: crate::model::DeviceType) -> Self {
            self.device_type = Some(input);
            self
        }
        /// <p>The type of the device.</p>
        pub fn set_device_type(
            mut self,
            input: std::option::Option<crate::model::DeviceType>,
        ) -> Self {
            self.device_type = input;
            self
        }
        /// <p>The status of the device.</p>
        pub fn device_status(mut self, input: crate::model::DeviceStatus) -> Self {
            self.device_status = Some(input);
            self
        }
        /// <p>The status of the device.</p>
        pub fn set_device_status(
            mut self,
            input: std::option::Option<crate::model::DeviceStatus>,
        ) -> Self {
            self.device_status = input;
            self
        }
        /// Consumes the builder and constructs a [`DeviceSummary`](crate::model::DeviceSummary)
        pub fn build(self) -> crate::model::DeviceSummary {
            crate::model::DeviceSummary {
                device_arn: self.device_arn,
                device_name: self.device_name,
                provider_name: self.provider_name,
                device_type: self.device_type,
                device_status: self.device_status,
            }
        }
    }
}
impl DeviceSummary {
    /// Creates a new builder-style object to manufacture [`DeviceSummary`](crate::model::DeviceSummary)
    pub fn builder() -> crate::model::device_summary::Builder {
        crate::model::device_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DeviceStatus {
    #[allow(missing_docs)] // documentation missing in model
    Offline,
    #[allow(missing_docs)] // documentation missing in model
    Online,
    #[allow(missing_docs)] // documentation missing in model
    Retired,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DeviceStatus {
    fn from(s: &str) -> Self {
        match s {
            "OFFLINE" => DeviceStatus::Offline,
            "ONLINE" => DeviceStatus::Online,
            "RETIRED" => DeviceStatus::Retired,
            other => DeviceStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DeviceStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DeviceStatus::from(s))
    }
}
impl DeviceStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DeviceStatus::Offline => "OFFLINE",
            DeviceStatus::Online => "ONLINE",
            DeviceStatus::Retired => "RETIRED",
            DeviceStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["OFFLINE", "ONLINE", "RETIRED"]
    }
}
impl AsRef<str> for DeviceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DeviceType {
    #[allow(missing_docs)] // documentation missing in model
    Qpu,
    #[allow(missing_docs)] // documentation missing in model
    Simulator,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DeviceType {
    fn from(s: &str) -> Self {
        match s {
            "QPU" => DeviceType::Qpu,
            "SIMULATOR" => DeviceType::Simulator,
            other => DeviceType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DeviceType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DeviceType::from(s))
    }
}
impl DeviceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DeviceType::Qpu => "QPU",
            DeviceType::Simulator => "SIMULATOR",
            DeviceType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["QPU", "SIMULATOR"]
    }
}
impl AsRef<str> for DeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The filter to use for searching devices.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SearchDevicesFilter {
    /// <p>The name to use to filter results.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The values to use to filter results.</p>
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl SearchDevicesFilter {
    /// <p>The name to use to filter results.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The values to use to filter results.</p>
    pub fn values(&self) -> std::option::Option<&[std::string::String]> {
        self.values.as_deref()
    }
}
impl std::fmt::Debug for SearchDevicesFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchDevicesFilter");
        formatter.field("name", &self.name);
        formatter.field("values", &self.values);
        formatter.finish()
    }
}
/// See [`SearchDevicesFilter`](crate::model::SearchDevicesFilter)
pub mod search_devices_filter {
    /// A builder for [`SearchDevicesFilter`](crate::model::SearchDevicesFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) values: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>The name to use to filter results.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name to use to filter results.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Appends an item to `values`.
        ///
        /// To override the contents of this collection use [`set_values`](Self::set_values).
        ///
        /// <p>The values to use to filter results.</p>
        pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
            v.push(input.into());
            self.values = Some(v);
            self
        }
        /// <p>The values to use to filter results.</p>
        pub fn set_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.values = input;
            self
        }
        /// Consumes the builder and constructs a [`SearchDevicesFilter`](crate::model::SearchDevicesFilter)
        pub fn build(self) -> crate::model::SearchDevicesFilter {
            crate::model::SearchDevicesFilter {
                name: self.name,
                values: self.values,
            }
        }
    }
}
impl SearchDevicesFilter {
    /// Creates a new builder-style object to manufacture [`SearchDevicesFilter`](crate::model::SearchDevicesFilter)
    pub fn builder() -> crate::model::search_devices_filter::Builder {
        crate::model::search_devices_filter::Builder::default()
    }
}
