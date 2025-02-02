// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BadRequestException(crate::error::BadRequestException),
    GatewayTimeoutException(crate::error::GatewayTimeoutException),
    NotFoundException(crate::error::NotFoundException),
    TooManyRequestsException(crate::error::TooManyRequestsException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::GatewayTimeoutException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CloneBackendError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CloneBackendError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CloneBackendErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CloneBackendErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CloneBackendErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CloneBackendErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CloneBackendErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateBackendError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateBackendError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateBackendErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateBackendErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CreateBackendErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateBackendErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateBackendErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateBackendAPIError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateBackendAPIError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateBackendAPIErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateBackendAPIErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CreateBackendAPIErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateBackendAPIErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateBackendAPIErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateBackendAuthError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateBackendAuthError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateBackendAuthErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateBackendAuthErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CreateBackendAuthErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateBackendAuthErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateBackendAuthErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateBackendConfigError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateBackendConfigError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateBackendConfigErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateBackendConfigErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CreateBackendConfigErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateBackendConfigErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateBackendConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateTokenError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateTokenError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateTokenErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateTokenErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::CreateTokenErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateTokenErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateTokenErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteBackendError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteBackendError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteBackendErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteBackendErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::DeleteBackendErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteBackendErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteBackendErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteBackendAPIError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteBackendAPIError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteBackendAPIErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteBackendAPIErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::DeleteBackendAPIErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteBackendAPIErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteBackendAPIErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteBackendAuthError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteBackendAuthError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteBackendAuthErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteBackendAuthErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::DeleteBackendAuthErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteBackendAuthErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteBackendAuthErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteTokenError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteTokenError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteTokenErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteTokenErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::DeleteTokenErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteTokenErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteTokenErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GenerateBackendAPIModelsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::GenerateBackendAPIModelsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GenerateBackendAPIModelsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GenerateBackendAPIModelsErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GenerateBackendAPIModelsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GenerateBackendAPIModelsErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::GenerateBackendAPIModelsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBackendError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBackendError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBackendErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetBackendErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetBackendErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetBackendErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetBackendErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBackendAPIError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBackendAPIError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBackendAPIErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetBackendAPIErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetBackendAPIErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetBackendAPIErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetBackendAPIErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBackendAPIModelsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBackendAPIModelsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBackendAPIModelsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetBackendAPIModelsErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetBackendAPIModelsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetBackendAPIModelsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetBackendAPIModelsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBackendAuthError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBackendAuthError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBackendAuthErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetBackendAuthErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetBackendAuthErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetBackendAuthErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetBackendAuthErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBackendJobError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBackendJobError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBackendJobErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetBackendJobErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetBackendJobErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetBackendJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetBackendJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetTokenError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetTokenError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetTokenErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetTokenErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::GetTokenErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetTokenErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetTokenErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ImportBackendAuthError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ImportBackendAuthError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ImportBackendAuthErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ImportBackendAuthErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::ImportBackendAuthErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ImportBackendAuthErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ImportBackendAuthErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListBackendJobsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListBackendJobsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListBackendJobsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListBackendJobsErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::ListBackendJobsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ListBackendJobsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListBackendJobsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::RemoveAllBackendsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::RemoveAllBackendsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemoveAllBackendsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::RemoveAllBackendsErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::RemoveAllBackendsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::RemoveAllBackendsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::RemoveAllBackendsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::RemoveBackendConfigError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::RemoveBackendConfigError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemoveBackendConfigErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::RemoveBackendConfigErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::RemoveBackendConfigErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::RemoveBackendConfigErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::RemoveBackendConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateBackendAPIError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateBackendAPIError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateBackendAPIErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateBackendAPIErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::UpdateBackendAPIErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateBackendAPIErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateBackendAPIErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateBackendAuthError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateBackendAuthError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateBackendAuthErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateBackendAuthErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::UpdateBackendAuthErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateBackendAuthErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateBackendAuthErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateBackendConfigError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateBackendConfigError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateBackendConfigErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateBackendConfigErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::UpdateBackendConfigErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateBackendConfigErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateBackendConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateBackendJobError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateBackendJobError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateBackendJobErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateBackendJobErrorKind::GatewayTimeoutException(inner) => {
                    Error::GatewayTimeoutException(inner)
                }
                crate::error::UpdateBackendJobErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateBackendJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateBackendJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
