// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BaseException(crate::error::BaseException),
    DisabledOperationException(crate::error::DisabledOperationException),
    InternalException(crate::error::InternalException),
    InvalidTypeException(crate::error::InvalidTypeException),
    LimitExceededException(crate::error::LimitExceededException),
    ResourceAlreadyExistsException(crate::error::ResourceAlreadyExistsException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BaseException(inner) => inner.fmt(f),
            Error::DisabledOperationException(inner) => inner.fmt(f),
            Error::InternalException(inner) => inner.fmt(f),
            Error::InvalidTypeException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::BuildSuggestersError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::BuildSuggestersError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BuildSuggestersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::BuildSuggestersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::BuildSuggestersErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::BuildSuggestersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateDomainError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateDomainError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDomainErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::CreateDomainErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::CreateDomainErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateDomainErrorKind::ResourceAlreadyExistsException(inner) => {
                    Error::ResourceAlreadyExistsException(inner)
                }
                crate::error::CreateDomainErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateDomainErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DefineAnalysisSchemeError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DefineAnalysisSchemeError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineAnalysisSchemeErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineAnalysisSchemeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DefineExpressionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DefineExpressionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineExpressionErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineExpressionErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineExpressionErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineExpressionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineExpressionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineExpressionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DefineIndexFieldError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DefineIndexFieldError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineIndexFieldErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineIndexFieldErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DefineSuggesterError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DefineSuggesterError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DefineSuggesterErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DefineSuggesterErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DefineSuggesterErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DefineSuggesterErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DefineSuggesterErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DefineSuggesterErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteAnalysisSchemeError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteAnalysisSchemeError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteAnalysisSchemeErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteAnalysisSchemeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteDomainError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteDomainError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDomainErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteDomainErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteDomainErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteExpressionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteExpressionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteExpressionErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteExpressionErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteExpressionErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteExpressionErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteExpressionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteIndexFieldError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteIndexFieldError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteIndexFieldErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteIndexFieldErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteSuggesterError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteSuggesterError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteSuggesterErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteSuggesterErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAnalysisSchemesError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAnalysisSchemesError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAnalysisSchemesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeAnalysisSchemesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeAnalysisSchemesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeAnalysisSchemesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAvailabilityOptionsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAvailabilityOptionsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAvailabilityOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::DescribeAvailabilityOptionsErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeAvailabilityOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeDomainEndpointOptionsError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeDomainEndpointOptionsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeDomainEndpointOptionsErrorKind::BaseException(inner) => Error::BaseException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::DisabledOperationException(inner) => Error::DisabledOperationException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::InternalException(inner) => Error::InternalException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeDomainEndpointOptionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeDomainsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeDomainsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDomainsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeDomainsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeDomainsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeExpressionsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeExpressionsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeExpressionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeExpressionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeIndexFieldsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeIndexFieldsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIndexFieldsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeIndexFieldsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeScalingParametersError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeScalingParametersError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingParametersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeScalingParametersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeScalingParametersErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeScalingParametersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeServiceAccessPoliciesError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeServiceAccessPoliciesError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeServiceAccessPoliciesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeServiceAccessPoliciesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeServiceAccessPoliciesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeServiceAccessPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeSuggestersError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeSuggestersError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeSuggestersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeSuggestersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::IndexDocumentsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::IndexDocumentsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::IndexDocumentsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::IndexDocumentsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::IndexDocumentsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::IndexDocumentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListDomainNamesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListDomainNamesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDomainNamesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::ListDomainNamesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateAvailabilityOptionsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateAvailabilityOptionsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateAvailabilityOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::UpdateAvailabilityOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateAvailabilityOptionsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateAvailabilityOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateDomainEndpointOptionsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateDomainEndpointOptionsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDomainEndpointOptionsErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::DisabledOperationException(
                    inner,
                ) => Error::DisabledOperationException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateDomainEndpointOptionsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateDomainEndpointOptionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateScalingParametersError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateScalingParametersError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateScalingParametersErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateScalingParametersErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateScalingParametersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateServiceAccessPoliciesError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::UpdateServiceAccessPoliciesError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateServiceAccessPoliciesErrorKind::BaseException(inner) => {
                    Error::BaseException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::InvalidTypeException(inner) => {
                    Error::InvalidTypeException(inner)
                }
                crate::error::UpdateServiceAccessPoliciesErrorKind::LimitExceededException(
                    inner,
                ) => Error::LimitExceededException(inner),
                crate::error::UpdateServiceAccessPoliciesErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateServiceAccessPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
