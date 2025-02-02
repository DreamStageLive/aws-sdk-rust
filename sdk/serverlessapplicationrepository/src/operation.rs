// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput)
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// <p>Creates an application version.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplicationVersion {
    _private: (),
}
impl CreateApplicationVersion {
    /// Creates a new builder-style object to manufacture [`CreateApplicationVersionInput`](crate::input::CreateApplicationVersionInput)
    pub fn builder() -> crate::input::create_application_version_input::Builder {
        crate::input::create_application_version_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateApplicationVersion {
    type Output = std::result::Result<
        crate::output::CreateApplicationVersionOutput,
        crate::error::CreateApplicationVersionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_application_version_error(response)
        } else {
            crate::operation_deser::parse_create_application_version_response(response)
        }
    }
}

/// <p>Creates an AWS CloudFormation change set for the given application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateCloudFormationChangeSet {
    _private: (),
}
impl CreateCloudFormationChangeSet {
    /// Creates a new builder-style object to manufacture [`CreateCloudFormationChangeSetInput`](crate::input::CreateCloudFormationChangeSetInput)
    pub fn builder() -> crate::input::create_cloud_formation_change_set_input::Builder {
        crate::input::create_cloud_formation_change_set_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateCloudFormationChangeSet {
    type Output = std::result::Result<
        crate::output::CreateCloudFormationChangeSetOutput,
        crate::error::CreateCloudFormationChangeSetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_cloud_formation_change_set_error(response)
        } else {
            crate::operation_deser::parse_create_cloud_formation_change_set_response(response)
        }
    }
}

/// <p>Creates an AWS CloudFormation template.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateCloudFormationTemplate {
    _private: (),
}
impl CreateCloudFormationTemplate {
    /// Creates a new builder-style object to manufacture [`CreateCloudFormationTemplateInput`](crate::input::CreateCloudFormationTemplateInput)
    pub fn builder() -> crate::input::create_cloud_formation_template_input::Builder {
        crate::input::create_cloud_formation_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateCloudFormationTemplate {
    type Output = std::result::Result<
        crate::output::CreateCloudFormationTemplateOutput,
        crate::error::CreateCloudFormationTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_cloud_formation_template_error(response)
        } else {
            crate::operation_deser::parse_create_cloud_formation_template_response(response)
        }
    }
}

/// <p>Deletes the specified application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput)
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// <p>Gets the specified application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetApplication {
    _private: (),
}
impl GetApplication {
    /// Creates a new builder-style object to manufacture [`GetApplicationInput`](crate::input::GetApplicationInput)
    pub fn builder() -> crate::input::get_application_input::Builder {
        crate::input::get_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetApplication {
    type Output =
        std::result::Result<crate::output::GetApplicationOutput, crate::error::GetApplicationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_application_error(response)
        } else {
            crate::operation_deser::parse_get_application_response(response)
        }
    }
}

/// <p>Retrieves the policy for the application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetApplicationPolicy {
    _private: (),
}
impl GetApplicationPolicy {
    /// Creates a new builder-style object to manufacture [`GetApplicationPolicyInput`](crate::input::GetApplicationPolicyInput)
    pub fn builder() -> crate::input::get_application_policy_input::Builder {
        crate::input::get_application_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetApplicationPolicy {
    type Output = std::result::Result<
        crate::output::GetApplicationPolicyOutput,
        crate::error::GetApplicationPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_application_policy_error(response)
        } else {
            crate::operation_deser::parse_get_application_policy_response(response)
        }
    }
}

/// <p>Gets the specified AWS CloudFormation template.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetCloudFormationTemplate {
    _private: (),
}
impl GetCloudFormationTemplate {
    /// Creates a new builder-style object to manufacture [`GetCloudFormationTemplateInput`](crate::input::GetCloudFormationTemplateInput)
    pub fn builder() -> crate::input::get_cloud_formation_template_input::Builder {
        crate::input::get_cloud_formation_template_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetCloudFormationTemplate {
    type Output = std::result::Result<
        crate::output::GetCloudFormationTemplateOutput,
        crate::error::GetCloudFormationTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_cloud_formation_template_error(response)
        } else {
            crate::operation_deser::parse_get_cloud_formation_template_response(response)
        }
    }
}

/// <p>Retrieves the list of applications nested in the containing application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplicationDependencies {
    _private: (),
}
impl ListApplicationDependencies {
    /// Creates a new builder-style object to manufacture [`ListApplicationDependenciesInput`](crate::input::ListApplicationDependenciesInput)
    pub fn builder() -> crate::input::list_application_dependencies_input::Builder {
        crate::input::list_application_dependencies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListApplicationDependencies {
    type Output = std::result::Result<
        crate::output::ListApplicationDependenciesOutput,
        crate::error::ListApplicationDependenciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_application_dependencies_error(response)
        } else {
            crate::operation_deser::parse_list_application_dependencies_response(response)
        }
    }
}

/// <p>Lists applications owned by the requester.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput)
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
        }
    }
}

/// <p>Lists versions for the specified application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplicationVersions {
    _private: (),
}
impl ListApplicationVersions {
    /// Creates a new builder-style object to manufacture [`ListApplicationVersionsInput`](crate::input::ListApplicationVersionsInput)
    pub fn builder() -> crate::input::list_application_versions_input::Builder {
        crate::input::list_application_versions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListApplicationVersions {
    type Output = std::result::Result<
        crate::output::ListApplicationVersionsOutput,
        crate::error::ListApplicationVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_application_versions_error(response)
        } else {
            crate::operation_deser::parse_list_application_versions_response(response)
        }
    }
}

/// <p>Sets the permission policy for an application. For the list of actions supported for this operation, see
/// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
/// Permissions</a>
/// .</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutApplicationPolicy {
    _private: (),
}
impl PutApplicationPolicy {
    /// Creates a new builder-style object to manufacture [`PutApplicationPolicyInput`](crate::input::PutApplicationPolicyInput)
    pub fn builder() -> crate::input::put_application_policy_input::Builder {
        crate::input::put_application_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutApplicationPolicy {
    type Output = std::result::Result<
        crate::output::PutApplicationPolicyOutput,
        crate::error::PutApplicationPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_application_policy_error(response)
        } else {
            crate::operation_deser::parse_put_application_policy_response(response)
        }
    }
}

/// <p>Unshares an application from an AWS Organization.</p><p>This operation can be called only from the organization's master account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UnshareApplication {
    _private: (),
}
impl UnshareApplication {
    /// Creates a new builder-style object to manufacture [`UnshareApplicationInput`](crate::input::UnshareApplicationInput)
    pub fn builder() -> crate::input::unshare_application_input::Builder {
        crate::input::unshare_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UnshareApplication {
    type Output = std::result::Result<
        crate::output::UnshareApplicationOutput,
        crate::error::UnshareApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_unshare_application_error(response)
        } else {
            crate::operation_deser::parse_unshare_application_response(response)
        }
    }
}

/// <p>Updates the specified application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput)
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}
