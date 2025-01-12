// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Statistics about the images in a dataset.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DatasetImageStats {
    /// <p>The total number of images in the dataset.</p>
    #[doc(hidden)]
    pub total: ::std::option::Option<i32>,
    /// <p>The total number of labeled images.</p>
    #[doc(hidden)]
    pub labeled: ::std::option::Option<i32>,
    /// <p>The total number of images labeled as normal.</p>
    #[doc(hidden)]
    pub normal: ::std::option::Option<i32>,
    /// <p>the total number of images labeled as an anomaly.</p>
    #[doc(hidden)]
    pub anomaly: ::std::option::Option<i32>,
}
impl DatasetImageStats {
    /// <p>The total number of images in the dataset.</p>
    pub fn total(&self) -> ::std::option::Option<i32> {
        self.total
    }
    /// <p>The total number of labeled images.</p>
    pub fn labeled(&self) -> ::std::option::Option<i32> {
        self.labeled
    }
    /// <p>The total number of images labeled as normal.</p>
    pub fn normal(&self) -> ::std::option::Option<i32> {
        self.normal
    }
    /// <p>the total number of images labeled as an anomaly.</p>
    pub fn anomaly(&self) -> ::std::option::Option<i32> {
        self.anomaly
    }
}
impl DatasetImageStats {
    /// Creates a new builder-style object to manufacture [`DatasetImageStats`](crate::types::DatasetImageStats).
    pub fn builder() -> crate::types::builders::DatasetImageStatsBuilder {
        crate::types::builders::DatasetImageStatsBuilder::default()
    }
}

/// A builder for [`DatasetImageStats`](crate::types::DatasetImageStats).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatasetImageStatsBuilder {
    pub(crate) total: ::std::option::Option<i32>,
    pub(crate) labeled: ::std::option::Option<i32>,
    pub(crate) normal: ::std::option::Option<i32>,
    pub(crate) anomaly: ::std::option::Option<i32>,
}
impl DatasetImageStatsBuilder {
    /// <p>The total number of images in the dataset.</p>
    pub fn total(mut self, input: i32) -> Self {
        self.total = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of images in the dataset.</p>
    pub fn set_total(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total = input;
        self
    }
    /// <p>The total number of labeled images.</p>
    pub fn labeled(mut self, input: i32) -> Self {
        self.labeled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of labeled images.</p>
    pub fn set_labeled(mut self, input: ::std::option::Option<i32>) -> Self {
        self.labeled = input;
        self
    }
    /// <p>The total number of images labeled as normal.</p>
    pub fn normal(mut self, input: i32) -> Self {
        self.normal = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of images labeled as normal.</p>
    pub fn set_normal(mut self, input: ::std::option::Option<i32>) -> Self {
        self.normal = input;
        self
    }
    /// <p>the total number of images labeled as an anomaly.</p>
    pub fn anomaly(mut self, input: i32) -> Self {
        self.anomaly = ::std::option::Option::Some(input);
        self
    }
    /// <p>the total number of images labeled as an anomaly.</p>
    pub fn set_anomaly(mut self, input: ::std::option::Option<i32>) -> Self {
        self.anomaly = input;
        self
    }
    /// Consumes the builder and constructs a [`DatasetImageStats`](crate::types::DatasetImageStats).
    pub fn build(self) -> crate::types::DatasetImageStats {
        crate::types::DatasetImageStats {
            total: self.total,
            labeled: self.labeled,
            normal: self.normal,
            anomaly: self.anomaly,
        }
    }
}
