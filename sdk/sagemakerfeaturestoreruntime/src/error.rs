// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `PutRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutRecordError {
    /// Kind of error that occurred.
    pub kind: PutRecordErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutRecordError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutRecordErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutRecordErrorKind {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalFailure(crate::error::InternalFailure),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::error::ValidationError),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for PutRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutRecordErrorKind::AccessForbidden(_inner) => _inner.fmt(f),
            PutRecordErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            PutRecordErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            PutRecordErrorKind::ValidationError(_inner) => _inner.fmt(f),
            PutRecordErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutRecordError {
    fn code(&self) -> Option<&str> {
        PutRecordError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutRecordError {
    /// Creates a new `PutRecordError`.
    pub fn new(kind: PutRecordErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutRecordError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PutRecordError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutRecordErrorKind::AccessForbidden`.
    pub fn is_access_forbidden(&self) -> bool {
        matches!(&self.kind, PutRecordErrorKind::AccessForbidden(_))
    }
    /// Returns `true` if the error kind is `PutRecordErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, PutRecordErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `PutRecordErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, PutRecordErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `PutRecordErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, PutRecordErrorKind::ValidationError(_))
    }
}
impl std::error::Error for PutRecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutRecordErrorKind::AccessForbidden(_inner) => Some(_inner),
            PutRecordErrorKind::InternalFailure(_inner) => Some(_inner),
            PutRecordErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            PutRecordErrorKind::ValidationError(_inner) => Some(_inner),
            PutRecordErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>There was an error validating your request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ValidationError {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ValidationError {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationError")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ValidationError {}
/// See [`ValidationError`](crate::error::ValidationError).
pub mod validation_error {

    /// A builder for [`ValidationError`](crate::error::ValidationError).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationError`](crate::error::ValidationError).
        pub fn build(self) -> crate::error::ValidationError {
            crate::error::ValidationError {
                message: self.message,
            }
        }
    }
}
impl ValidationError {
    /// Creates a new builder-style object to manufacture [`ValidationError`](crate::error::ValidationError).
    pub fn builder() -> crate::error::validation_error::Builder {
        crate::error::validation_error::Builder::default()
    }
}

/// <p>The service is currently unavailable.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ServiceUnavailable {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ServiceUnavailable {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailable")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailable {}
/// See [`ServiceUnavailable`](crate::error::ServiceUnavailable).
pub mod service_unavailable {

    /// A builder for [`ServiceUnavailable`](crate::error::ServiceUnavailable).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceUnavailable`](crate::error::ServiceUnavailable).
        pub fn build(self) -> crate::error::ServiceUnavailable {
            crate::error::ServiceUnavailable {
                message: self.message,
            }
        }
    }
}
impl ServiceUnavailable {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailable`](crate::error::ServiceUnavailable).
    pub fn builder() -> crate::error::service_unavailable::Builder {
        crate::error::service_unavailable::Builder::default()
    }
}

/// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalFailure {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalFailure {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalFailure")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalFailure {}
/// See [`InternalFailure`](crate::error::InternalFailure).
pub mod internal_failure {

    /// A builder for [`InternalFailure`](crate::error::InternalFailure).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalFailure`](crate::error::InternalFailure).
        pub fn build(self) -> crate::error::InternalFailure {
            crate::error::InternalFailure {
                message: self.message,
            }
        }
    }
}
impl InternalFailure {
    /// Creates a new builder-style object to manufacture [`InternalFailure`](crate::error::InternalFailure).
    pub fn builder() -> crate::error::internal_failure::Builder {
        crate::error::internal_failure::Builder::default()
    }
}

/// <p>You do not have permission to perform an action.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AccessForbidden {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl AccessForbidden {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessForbidden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessForbidden")?;
        if let Some(inner_4) = &self.message {
            {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for AccessForbidden {}
/// See [`AccessForbidden`](crate::error::AccessForbidden).
pub mod access_forbidden {

    /// A builder for [`AccessForbidden`](crate::error::AccessForbidden).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`AccessForbidden`](crate::error::AccessForbidden).
        pub fn build(self) -> crate::error::AccessForbidden {
            crate::error::AccessForbidden {
                message: self.message,
            }
        }
    }
}
impl AccessForbidden {
    /// Creates a new builder-style object to manufacture [`AccessForbidden`](crate::error::AccessForbidden).
    pub fn builder() -> crate::error::access_forbidden::Builder {
        crate::error::access_forbidden::Builder::default()
    }
}

/// Error type for the `GetRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetRecordError {
    /// Kind of error that occurred.
    pub kind: GetRecordErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetRecordError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetRecordErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetRecordErrorKind {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalFailure(crate::error::InternalFailure),
    /// <p>A resource that is required to perform an action was not found.</p>
    ResourceNotFound(crate::error::ResourceNotFound),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::error::ValidationError),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GetRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRecordErrorKind::AccessForbidden(_inner) => _inner.fmt(f),
            GetRecordErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            GetRecordErrorKind::ResourceNotFound(_inner) => _inner.fmt(f),
            GetRecordErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            GetRecordErrorKind::ValidationError(_inner) => _inner.fmt(f),
            GetRecordErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetRecordError {
    fn code(&self) -> Option<&str> {
        GetRecordError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRecordError {
    /// Creates a new `GetRecordError`.
    pub fn new(kind: GetRecordErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetRecordError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GetRecordError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetRecordErrorKind::AccessForbidden`.
    pub fn is_access_forbidden(&self) -> bool {
        matches!(&self.kind, GetRecordErrorKind::AccessForbidden(_))
    }
    /// Returns `true` if the error kind is `GetRecordErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, GetRecordErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `GetRecordErrorKind::ResourceNotFound`.
    pub fn is_resource_not_found(&self) -> bool {
        matches!(&self.kind, GetRecordErrorKind::ResourceNotFound(_))
    }
    /// Returns `true` if the error kind is `GetRecordErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, GetRecordErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `GetRecordErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, GetRecordErrorKind::ValidationError(_))
    }
}
impl std::error::Error for GetRecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRecordErrorKind::AccessForbidden(_inner) => Some(_inner),
            GetRecordErrorKind::InternalFailure(_inner) => Some(_inner),
            GetRecordErrorKind::ResourceNotFound(_inner) => Some(_inner),
            GetRecordErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            GetRecordErrorKind::ValidationError(_inner) => Some(_inner),
            GetRecordErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>A resource that is required to perform an action was not found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceNotFound {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ResourceNotFound {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFound")?;
        if let Some(inner_5) = &self.message {
            {
                write!(f, ": {}", inner_5)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFound {}
/// See [`ResourceNotFound`](crate::error::ResourceNotFound).
pub mod resource_not_found {

    /// A builder for [`ResourceNotFound`](crate::error::ResourceNotFound).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFound`](crate::error::ResourceNotFound).
        pub fn build(self) -> crate::error::ResourceNotFound {
            crate::error::ResourceNotFound {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFound {
    /// Creates a new builder-style object to manufacture [`ResourceNotFound`](crate::error::ResourceNotFound).
    pub fn builder() -> crate::error::resource_not_found::Builder {
        crate::error::resource_not_found::Builder::default()
    }
}

/// Error type for the `DeleteRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteRecordError {
    /// Kind of error that occurred.
    pub kind: DeleteRecordErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for DeleteRecordError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: DeleteRecordErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `DeleteRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteRecordErrorKind {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalFailure(crate::error::InternalFailure),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::error::ValidationError),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for DeleteRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteRecordErrorKind::AccessForbidden(_inner) => _inner.fmt(f),
            DeleteRecordErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            DeleteRecordErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            DeleteRecordErrorKind::ValidationError(_inner) => _inner.fmt(f),
            DeleteRecordErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteRecordError {
    fn code(&self) -> Option<&str> {
        DeleteRecordError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteRecordError {
    /// Creates a new `DeleteRecordError`.
    pub fn new(kind: DeleteRecordErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DeleteRecordError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `DeleteRecordError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DeleteRecordErrorKind::AccessForbidden`.
    pub fn is_access_forbidden(&self) -> bool {
        matches!(&self.kind, DeleteRecordErrorKind::AccessForbidden(_))
    }
    /// Returns `true` if the error kind is `DeleteRecordErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, DeleteRecordErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `DeleteRecordErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, DeleteRecordErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `DeleteRecordErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, DeleteRecordErrorKind::ValidationError(_))
    }
}
impl std::error::Error for DeleteRecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteRecordErrorKind::AccessForbidden(_inner) => Some(_inner),
            DeleteRecordErrorKind::InternalFailure(_inner) => Some(_inner),
            DeleteRecordErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            DeleteRecordErrorKind::ValidationError(_inner) => Some(_inner),
            DeleteRecordErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `BatchGetRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct BatchGetRecordError {
    /// Kind of error that occurred.
    pub kind: BatchGetRecordErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for BatchGetRecordError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: BatchGetRecordErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `BatchGetRecord` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum BatchGetRecordErrorKind {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalFailure(crate::error::InternalFailure),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::error::ValidationError),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for BatchGetRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            BatchGetRecordErrorKind::AccessForbidden(_inner) => _inner.fmt(f),
            BatchGetRecordErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            BatchGetRecordErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            BatchGetRecordErrorKind::ValidationError(_inner) => _inner.fmt(f),
            BatchGetRecordErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for BatchGetRecordError {
    fn code(&self) -> Option<&str> {
        BatchGetRecordError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl BatchGetRecordError {
    /// Creates a new `BatchGetRecordError`.
    pub fn new(kind: BatchGetRecordErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `BatchGetRecordError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: BatchGetRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `BatchGetRecordError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: BatchGetRecordErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `BatchGetRecordErrorKind::AccessForbidden`.
    pub fn is_access_forbidden(&self) -> bool {
        matches!(&self.kind, BatchGetRecordErrorKind::AccessForbidden(_))
    }
    /// Returns `true` if the error kind is `BatchGetRecordErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, BatchGetRecordErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `BatchGetRecordErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, BatchGetRecordErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `BatchGetRecordErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, BatchGetRecordErrorKind::ValidationError(_))
    }
}
impl std::error::Error for BatchGetRecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            BatchGetRecordErrorKind::AccessForbidden(_inner) => Some(_inner),
            BatchGetRecordErrorKind::InternalFailure(_inner) => Some(_inner),
            BatchGetRecordErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            BatchGetRecordErrorKind::ValidationError(_inner) => Some(_inner),
            BatchGetRecordErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
