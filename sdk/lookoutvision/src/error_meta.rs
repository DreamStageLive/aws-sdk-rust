// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    ConflictException(crate::error::ConflictException),
    InternalServerException(crate::error::InternalServerException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateDatasetErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CreateDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateModelError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateModelError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateModelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateModelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateModelErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateModelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateModelErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CreateModelErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateModelErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateModelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateProjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateProjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateProjectErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateProjectErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateProjectErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateProjectErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateProjectErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CreateProjectErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteModelError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteModelError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteModelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteModelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteModelErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteModelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteModelErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteModelErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteModelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteProjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteProjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteProjectErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteProjectErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteProjectErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteProjectErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteProjectErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DescribeDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeModelError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeModelError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeModelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeModelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DescribeModelErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeModelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeModelErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DescribeModelErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeModelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeProjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeProjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeProjectErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeProjectErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DescribeProjectErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeProjectErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeProjectErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DescribeProjectErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DetectAnomaliesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DetectAnomaliesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DetectAnomaliesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DetectAnomaliesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListDatasetEntriesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListDatasetEntriesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDatasetEntriesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListDatasetEntriesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListModelsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListModelsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListModelsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListModelsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListModelsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListModelsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListModelsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListModelsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListModelsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListProjectsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListProjectsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListProjectsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListProjectsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListProjectsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListProjectsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListProjectsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListProjectsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListProjectsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartModelError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartModelError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartModelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StartModelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartModelErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartModelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StartModelErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::StartModelErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StartModelErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StartModelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopModelError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopModelError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopModelErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StopModelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StopModelErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopModelErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopModelErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StopModelErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StopModelErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::TagResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::TagResourceErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::TagResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::TagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UntagResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UntagResourceErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateDatasetEntriesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateDatasetEntriesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDatasetEntriesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateDatasetEntriesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
