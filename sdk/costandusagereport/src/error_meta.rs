// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    DuplicateReportNameException(crate::error::DuplicateReportNameException),
    InternalErrorException(crate::error::InternalErrorException),
    ReportLimitReachedException(crate::error::ReportLimitReachedException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateReportNameException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::ReportLimitReachedException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteReportDefinitionErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DeleteReportDefinitionErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteReportDefinitionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeReportDefinitionsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeReportDefinitionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ModifyReportDefinitionErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ModifyReportDefinitionErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ModifyReportDefinitionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutReportDefinitionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutReportDefinitionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutReportDefinitionErrorKind::DuplicateReportNameException(inner) => {
                    Error::DuplicateReportNameException(inner)
                }
                crate::error::PutReportDefinitionErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::PutReportDefinitionErrorKind::ReportLimitReachedException(inner) => {
                    Error::ReportLimitReachedException(inner)
                }
                crate::error::PutReportDefinitionErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::PutReportDefinitionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
