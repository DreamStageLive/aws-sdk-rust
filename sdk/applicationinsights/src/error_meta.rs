// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    BadRequestException(crate::error::BadRequestException),
    InternalServerException(crate::error::InternalServerException),
    ResourceInUseException(crate::error::ResourceInUseException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    TagsAlreadyExistException(crate::error::TagsAlreadyExistException),
    TooManyTagsException(crate::error::TooManyTagsException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TagsAlreadyExistException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateApplicationErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateApplicationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateApplicationErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CreateApplicationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateApplicationErrorKind::TagsAlreadyExistException(inner) => {
                    Error::TagsAlreadyExistException(inner)
                }
                crate::error::CreateApplicationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateComponentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateComponentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateComponentErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateComponentErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CreateComponentErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateComponentErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateComponentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateLogPatternError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateLogPatternError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateLogPatternErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateLogPatternErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CreateLogPatternErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateLogPatternErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateLogPatternErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteApplicationErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteApplicationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteApplicationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteApplicationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteComponentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteComponentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteComponentErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteComponentErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteComponentErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteComponentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteLogPatternError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteLogPatternError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLogPatternErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteLogPatternErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteLogPatternErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteLogPatternErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteLogPatternErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeApplicationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeApplicationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeApplicationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeComponentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeComponentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeComponentErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeComponentErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeComponentErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeComponentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeComponentConfigurationError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeComponentConfigurationError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeComponentConfigurationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeComponentConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeComponentConfigurationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DescribeComponentConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl
    From<
        smithy_http::result::SdkError<
            crate::error::DescribeComponentConfigurationRecommendationError,
        >,
    > for Error
{
    fn from(
        err: smithy_http::result::SdkError<
            crate::error::DescribeComponentConfigurationRecommendationError,
        >,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeComponentConfigurationRecommendationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeComponentConfigurationRecommendationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeComponentConfigurationRecommendationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DescribeComponentConfigurationRecommendationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeLogPatternError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeLogPatternError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeLogPatternErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeLogPatternErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeLogPatternErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeLogPatternErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeObservationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeObservationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeObservationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeObservationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeObservationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeObservationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeProblemError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeProblemError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeProblemErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeProblemErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeProblemErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeProblemErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeProblemObservationsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeProblemObservationsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeProblemObservationsErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeProblemObservationsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeProblemObservationsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeProblemObservationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListApplicationsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListApplicationsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListApplicationsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListApplicationsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListApplicationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListComponentsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListComponentsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListComponentsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListComponentsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListComponentsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListComponentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListConfigurationHistoryError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListConfigurationHistoryError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListConfigurationHistoryErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListConfigurationHistoryErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::ListConfigurationHistoryErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListConfigurationHistoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListLogPatternsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListLogPatternsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLogPatternsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListLogPatternsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListLogPatternsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListLogPatternsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListLogPatternSetsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListLogPatternSetsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLogPatternSetsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListLogPatternSetsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListLogPatternSetsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListLogPatternSetsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListProblemsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListProblemsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListProblemsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListProblemsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListProblemsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListProblemsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
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
impl From<smithy_http::result::SdkError<crate::error::TagResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
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
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
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
impl From<smithy_http::result::SdkError<crate::error::UpdateApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateApplicationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateApplicationErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateApplicationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateComponentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateComponentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateComponentErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateComponentErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateComponentErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateComponentErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateComponentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateComponentConfigurationError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateComponentConfigurationError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateComponentConfigurationErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::UpdateComponentConfigurationErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateComponentConfigurationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateComponentConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateLogPatternError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateLogPatternError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateLogPatternErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateLogPatternErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdateLogPatternErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateLogPatternErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateLogPatternErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
