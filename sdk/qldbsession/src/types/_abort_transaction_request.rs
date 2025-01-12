// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the transaction to abort.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AbortTransactionRequest {}
impl AbortTransactionRequest {
    /// Creates a new builder-style object to manufacture [`AbortTransactionRequest`](crate::types::AbortTransactionRequest).
    pub fn builder() -> crate::types::builders::AbortTransactionRequestBuilder {
        crate::types::builders::AbortTransactionRequestBuilder::default()
    }
}

/// A builder for [`AbortTransactionRequest`](crate::types::AbortTransactionRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AbortTransactionRequestBuilder {}
impl AbortTransactionRequestBuilder {
    /// Consumes the builder and constructs a [`AbortTransactionRequest`](crate::types::AbortTransactionRequest).
    pub fn build(self) -> crate::types::AbortTransactionRequest {
        crate::types::AbortTransactionRequest {}
    }
}
