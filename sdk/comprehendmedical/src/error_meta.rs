// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    InternalServerException(crate::error::InternalServerException),
    InvalidEncodingException(crate::error::InvalidEncodingException),
    InvalidRequestException(crate::error::InvalidRequestException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    TextSizeLimitExceededException(crate::error::TextSizeLimitExceededException),
    TooManyRequestsException(crate::error::TooManyRequestsException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidEncodingException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TextSizeLimitExceededException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeEntitiesDetectionV2JobError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeEntitiesDetectionV2JobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeEntitiesDetectionV2JobErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeEntitiesDetectionV2JobErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::DescribeEntitiesDetectionV2JobErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeEntitiesDetectionV2JobErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::DescribeEntitiesDetectionV2JobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeICD10CMInferenceJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeICD10CMInferenceJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeICD10CMInferenceJobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeICD10CMInferenceJobErrorKind::InvalidRequestException(
                    inner,
                ) => Error::InvalidRequestException(inner),
                crate::error::DescribeICD10CMInferenceJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeICD10CMInferenceJobErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::DescribeICD10CMInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribePHIDetectionJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribePHIDetectionJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribePHIDetectionJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribePHIDetectionJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DescribePHIDetectionJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribePHIDetectionJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribePHIDetectionJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeRxNormInferenceJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeRxNormInferenceJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeRxNormInferenceJobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeRxNormInferenceJobErrorKind::InvalidRequestException(
                    inner,
                ) => Error::InvalidRequestException(inner),
                crate::error::DescribeRxNormInferenceJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeRxNormInferenceJobErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::DescribeRxNormInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DetectEntitiesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DetectEntitiesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DetectEntitiesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DetectEntitiesErrorKind::InvalidEncodingException(inner) => {
                    Error::InvalidEncodingException(inner)
                }
                crate::error::DetectEntitiesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DetectEntitiesErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DetectEntitiesErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::DetectEntitiesErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DetectEntitiesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DetectEntitiesV2Error>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DetectEntitiesV2Error>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DetectEntitiesV2ErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::InvalidEncodingException(inner) => {
                    Error::InvalidEncodingException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DetectEntitiesV2ErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DetectPHIError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DetectPHIError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DetectPHIErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DetectPHIErrorKind::InvalidEncodingException(inner) => {
                    Error::InvalidEncodingException(inner)
                }
                crate::error::DetectPHIErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DetectPHIErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DetectPHIErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::DetectPHIErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DetectPHIErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::InferICD10CMError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::InferICD10CMError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InferICD10CMErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::InferICD10CMErrorKind::InvalidEncodingException(inner) => {
                    Error::InvalidEncodingException(inner)
                }
                crate::error::InferICD10CMErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::InferICD10CMErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::InferICD10CMErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::InferICD10CMErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::InferICD10CMErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::InferRxNormError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::InferRxNormError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InferRxNormErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::InferRxNormErrorKind::InvalidEncodingException(inner) => {
                    Error::InvalidEncodingException(inner)
                }
                crate::error::InferRxNormErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::InferRxNormErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::InferRxNormErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::InferRxNormErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::InferRxNormErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListEntitiesDetectionV2JobsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListEntitiesDetectionV2JobsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListEntitiesDetectionV2JobsErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::ListEntitiesDetectionV2JobsErrorKind::InvalidRequestException(
                    inner,
                ) => Error::InvalidRequestException(inner),
                crate::error::ListEntitiesDetectionV2JobsErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::ListEntitiesDetectionV2JobsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListEntitiesDetectionV2JobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListICD10CMInferenceJobsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListICD10CMInferenceJobsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListICD10CMInferenceJobsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListICD10CMInferenceJobsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListICD10CMInferenceJobsErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::ListICD10CMInferenceJobsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListICD10CMInferenceJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListPHIDetectionJobsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListPHIDetectionJobsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListPHIDetectionJobsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListPHIDetectionJobsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListPHIDetectionJobsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListPHIDetectionJobsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListPHIDetectionJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListRxNormInferenceJobsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListRxNormInferenceJobsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRxNormInferenceJobsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListRxNormInferenceJobsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListRxNormInferenceJobsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListRxNormInferenceJobsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListRxNormInferenceJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartEntitiesDetectionV2JobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartEntitiesDetectionV2JobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartEntitiesDetectionV2JobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::StartEntitiesDetectionV2JobErrorKind::InvalidRequestException(
                    inner,
                ) => Error::InvalidRequestException(inner),
                crate::error::StartEntitiesDetectionV2JobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StartEntitiesDetectionV2JobErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::StartEntitiesDetectionV2JobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartICD10CMInferenceJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartICD10CMInferenceJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartICD10CMInferenceJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartICD10CMInferenceJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StartICD10CMInferenceJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StartICD10CMInferenceJobErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::StartICD10CMInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartPHIDetectionJobError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartPHIDetectionJobError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartPHIDetectionJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartPHIDetectionJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StartPHIDetectionJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StartPHIDetectionJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::StartPHIDetectionJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartRxNormInferenceJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartRxNormInferenceJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartRxNormInferenceJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartRxNormInferenceJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StartRxNormInferenceJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StartRxNormInferenceJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::StartRxNormInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopEntitiesDetectionV2JobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StopEntitiesDetectionV2JobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopEntitiesDetectionV2JobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::StopEntitiesDetectionV2JobErrorKind::InvalidRequestException(
                    inner,
                ) => Error::InvalidRequestException(inner),
                crate::error::StopEntitiesDetectionV2JobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StopEntitiesDetectionV2JobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopICD10CMInferenceJobError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StopICD10CMInferenceJobError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopICD10CMInferenceJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopICD10CMInferenceJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StopICD10CMInferenceJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StopICD10CMInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopPHIDetectionJobError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopPHIDetectionJobError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopPHIDetectionJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopPHIDetectionJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StopPHIDetectionJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopPHIDetectionJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopRxNormInferenceJobError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopRxNormInferenceJobError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopRxNormInferenceJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopRxNormInferenceJobErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StopRxNormInferenceJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopRxNormInferenceJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
