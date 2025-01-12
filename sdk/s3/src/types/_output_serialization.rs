// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes how results of the Select job are serialized.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    #[doc(hidden)]
    pub csv: ::std::option::Option<crate::types::CsvOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    #[doc(hidden)]
    pub json: ::std::option::Option<crate::types::JsonOutput>,
}
impl OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub fn csv(&self) -> ::std::option::Option<&crate::types::CsvOutput> {
        self.csv.as_ref()
    }
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub fn json(&self) -> ::std::option::Option<&crate::types::JsonOutput> {
        self.json.as_ref()
    }
}
impl OutputSerialization {
    /// Creates a new builder-style object to manufacture [`OutputSerialization`](crate::types::OutputSerialization).
    pub fn builder() -> crate::types::builders::OutputSerializationBuilder {
        crate::types::builders::OutputSerializationBuilder::default()
    }
}

/// A builder for [`OutputSerialization`](crate::types::OutputSerialization).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OutputSerializationBuilder {
    pub(crate) csv: ::std::option::Option<crate::types::CsvOutput>,
    pub(crate) json: ::std::option::Option<crate::types::JsonOutput>,
}
impl OutputSerializationBuilder {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub fn csv(mut self, input: crate::types::CsvOutput) -> Self {
        self.csv = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub fn set_csv(mut self, input: ::std::option::Option<crate::types::CsvOutput>) -> Self {
        self.csv = input;
        self
    }
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub fn json(mut self, input: crate::types::JsonOutput) -> Self {
        self.json = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub fn set_json(mut self, input: ::std::option::Option<crate::types::JsonOutput>) -> Self {
        self.json = input;
        self
    }
    /// Consumes the builder and constructs a [`OutputSerialization`](crate::types::OutputSerialization).
    pub fn build(self) -> crate::types::OutputSerialization {
        crate::types::OutputSerialization {
            csv: self.csv,
            json: self.json,
        }
    }
}
