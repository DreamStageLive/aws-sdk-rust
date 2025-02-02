// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Returns a list of accounts in the organization from AWS Organizations that are affected by the
/// provided event. For more information about the different types of AWS Health events, see
/// <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>. </p>
/// <p>Before you can call this operation, you must first enable AWS Health to work with
/// AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's
/// management account.</p>
/// <note>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAffectedAccountsForOrganization {
    _private: (),
}
impl DescribeAffectedAccountsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedAccountsForOrganizationInput`](crate::input::DescribeAffectedAccountsForOrganizationInput)
    pub fn builder() -> crate::input::describe_affected_accounts_for_organization_input::Builder {
        crate::input::describe_affected_accounts_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAffectedAccountsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeAffectedAccountsForOrganizationOutput,
        crate::error::DescribeAffectedAccountsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_accounts_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_affected_accounts_for_organization_response(
                response,
            )
        }
    }
}

/// <p>Returns a list of entities that have been affected by the specified events, based on the
/// specified filter criteria. Entities can refer to individual customer resources, groups of
/// customer resources, or any other construct, depending on the AWS service. Events that
/// have impact beyond that of the affected entities, or where the extent of impact is unknown,
/// include at least one entity indicating this.</p>
/// <p>At least one event ARN is required. Results are sorted by the
/// <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p>
/// <note>
/// <ul>
/// <li>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </li>
/// <li>
/// <p>This operation supports resource-level permissions. You can use this operation to allow or deny access to specific AWS Health events. For more
/// information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>AWS Health User Guide</i>.</p>
/// </li>
/// </ul>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAffectedEntities {
    _private: (),
}
impl DescribeAffectedEntities {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedEntitiesInput`](crate::input::DescribeAffectedEntitiesInput)
    pub fn builder() -> crate::input::describe_affected_entities_input::Builder {
        crate::input::describe_affected_entities_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAffectedEntities {
    type Output = std::result::Result<
        crate::output::DescribeAffectedEntitiesOutput,
        crate::error::DescribeAffectedEntitiesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_entities_error(response)
        } else {
            crate::operation_deser::parse_describe_affected_entities_response(response)
        }
    }
}

/// <p>Returns a list of entities that have been affected by one or more events for one or more
/// accounts in your organization in AWS Organizations, based on the filter criteria. Entities can refer
/// to individual customer resources, groups of customer resources, or any other construct,
/// depending on the AWS service.</p>
/// <p>At least one event Amazon Resource Name (ARN) and account ID are required. Results are
/// sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most
/// recent.</p>
/// <p>Before you can call this operation, you must first enable AWS Health to work with
/// AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a>
/// operation from your organization's management account.</p>
/// <note>
/// <ul>
/// <li>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </li>
/// <li>
/// <p>This operation doesn't support resource-level permissions. You can't use this operation to allow or deny access to specific AWS Health events. For more
/// information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>AWS Health User Guide</i>.</p>
/// </li>
/// </ul>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAffectedEntitiesForOrganization {
    _private: (),
}
impl DescribeAffectedEntitiesForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedEntitiesForOrganizationInput`](crate::input::DescribeAffectedEntitiesForOrganizationInput)
    pub fn builder() -> crate::input::describe_affected_entities_for_organization_input::Builder {
        crate::input::describe_affected_entities_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAffectedEntitiesForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeAffectedEntitiesForOrganizationOutput,
        crate::error::DescribeAffectedEntitiesForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_entities_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_affected_entities_for_organization_response(
                response,
            )
        }
    }
}

/// <p>Returns the number of entities that are affected by each of the specified events. If no
/// events are specified, the counts of all affected entities are returned.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEntityAggregates {
    _private: (),
}
impl DescribeEntityAggregates {
    /// Creates a new builder-style object to manufacture [`DescribeEntityAggregatesInput`](crate::input::DescribeEntityAggregatesInput)
    pub fn builder() -> crate::input::describe_entity_aggregates_input::Builder {
        crate::input::describe_entity_aggregates_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEntityAggregates {
    type Output = std::result::Result<
        crate::output::DescribeEntityAggregatesOutput,
        crate::error::DescribeEntityAggregatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_entity_aggregates_error(response)
        } else {
            crate::operation_deser::parse_describe_entity_aggregates_response(response)
        }
    }
}

/// <p>Returns the number of events of each event type (issue, scheduled change, and account
/// notification). If no filter is specified, the counts of all events in each category are
/// returned.</p>
/// <note>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEventAggregates {
    _private: (),
}
impl DescribeEventAggregates {
    /// Creates a new builder-style object to manufacture [`DescribeEventAggregatesInput`](crate::input::DescribeEventAggregatesInput)
    pub fn builder() -> crate::input::describe_event_aggregates_input::Builder {
        crate::input::describe_event_aggregates_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEventAggregates {
    type Output = std::result::Result<
        crate::output::DescribeEventAggregatesOutput,
        crate::error::DescribeEventAggregatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_aggregates_error(response)
        } else {
            crate::operation_deser::parse_describe_event_aggregates_response(response)
        }
    }
}

/// <p>Returns detailed information about one or more specified events. Information includes
/// standard event data (AWS Region, service, and so on, as returned by <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a>), a detailed event description, and possible additional metadata
/// that depends upon the nature of the event. Affected entities are not included. To retrieve
/// the entities, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operation.</p>
/// <p>If a specified event can't be retrieved, an error message is returned for that
/// event.</p>
/// <note>
/// <p>This operation supports resource-level permissions. You can use this operation to allow or deny access to specific AWS Health events. For more
/// information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>AWS Health User Guide</i>.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEventDetails {
    _private: (),
}
impl DescribeEventDetails {
    /// Creates a new builder-style object to manufacture [`DescribeEventDetailsInput`](crate::input::DescribeEventDetailsInput)
    pub fn builder() -> crate::input::describe_event_details_input::Builder {
        crate::input::describe_event_details_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEventDetails {
    type Output = std::result::Result<
        crate::output::DescribeEventDetailsOutput,
        crate::error::DescribeEventDetailsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_details_error(response)
        } else {
            crate::operation_deser::parse_describe_event_details_response(response)
        }
    }
}

/// <p>Returns detailed information about one or more specified events for one or more AWS
/// accounts in your organization. This information includes standard event data (such as the
/// AWS Region and service), an event description, and (depending on the event) possible
/// metadata. This operation doesn't return affected entities, such as the resources related to
/// the event. To return affected entities, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operation.</p>
/// <note>
/// <p>Before you can call this operation, you must first enable AWS Health to work with
/// AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's
/// management account.</p>
/// </note>
/// <p>When you call the <code>DescribeEventDetailsForOrganization</code> operation, specify
/// the <code>organizationEventDetailFilters</code> object in the request. Depending on the
/// AWS Health event type, note the following differences:</p>
/// <ul>
/// <li>
/// <p>To return event details for a public event, you must specify a null value for the
/// <code>awsAccountId</code> parameter. If you specify an account ID for a public
/// event, AWS Health returns an error message because public events aren't specific to
/// an account.</p>
/// </li>
/// <li>
/// <p>To return event details for an event that is specific to an account in your
/// organization,  you must specify the <code>awsAccountId</code> parameter in the
/// request. If you don't specify an account ID, AWS Health returns an error message
/// because the event is specific to an account in your organization. </p>
/// </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p>
/// <note>
/// <p>This operation doesn't support resource-level permissions. You can't use this operation to allow or deny access to specific AWS Health events. For more
/// information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>AWS Health User Guide</i>.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEventDetailsForOrganization {
    _private: (),
}
impl DescribeEventDetailsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeEventDetailsForOrganizationInput`](crate::input::DescribeEventDetailsForOrganizationInput)
    pub fn builder() -> crate::input::describe_event_details_for_organization_input::Builder {
        crate::input::describe_event_details_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEventDetailsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeEventDetailsForOrganizationOutput,
        crate::error::DescribeEventDetailsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_details_for_organization_error(response)
        } else {
            crate::operation_deser::parse_describe_event_details_for_organization_response(response)
        }
    }
}

/// <p> Returns information about events that meet the specified filter criteria. Events are
/// returned in a summary form and do not include the detailed description, any additional
/// metadata that depends on the event type, or any affected resources. To retrieve that
/// information, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetails.html">DescribeEventDetails</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operations.</p>
/// <p>If no filter criteria are specified, all events are returned. Results are sorted by
/// <code>lastModifiedTime</code>, starting with the most recent event.</p>
/// <note>
/// <ul>
/// <li>
/// <p>When you call the <code>DescribeEvents</code> operation and specify an entity
/// for the <code>entityValues</code> parameter, AWS Health might return public
/// events that aren't specific to that resource. For example, if you call
/// <code>DescribeEvents</code> and specify an ID for an Amazon Elastic Compute Cloud (Amazon EC2)
/// instance, AWS Health might return events that aren't specific to that resource or
/// service. To get events that are specific to a service, use the
/// <code>services</code> parameter in the <code>filter</code> object. For more
/// information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p>
/// </li>
/// <li>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </li>
/// </ul>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEvents {
    _private: (),
}
impl DescribeEvents {
    /// Creates a new builder-style object to manufacture [`DescribeEventsInput`](crate::input::DescribeEventsInput)
    pub fn builder() -> crate::input::describe_events_input::Builder {
        crate::input::describe_events_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEvents {
    type Output =
        std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_error(response)
        } else {
            crate::operation_deser::parse_describe_events_response(response)
        }
    }
}

/// <p>Returns information about events across your organization in AWS Organizations. You can use
/// the<code>filters</code> parameter to specify the events that you want to return. Events
/// are returned in a summary form and don't include the affected accounts, detailed
/// description, any additional metadata that depends on the event type, or any affected
/// resources. To retrieve that information, use the following operations:</p>
/// <ul>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedAccountsForOrganization.html">DescribeAffectedAccountsForOrganization</a>
/// </p>
/// </li>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a>
/// </p>
/// </li>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a>
/// </p>
/// </li>
/// </ul>
/// <p>If you don't specify a <code>filter</code>, the
/// <code>DescribeEventsForOrganizations</code> returns all events across your organization.
/// Results are sorted by <code>lastModifiedTime</code>, starting with the most recent event. </p>
/// <p>For more information about the different types of AWS Health events, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p>
/// <p>Before you can call this operation, you must first enable AWS Health to work with
/// AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's
/// management account.</p>
/// <note>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEventsForOrganization {
    _private: (),
}
impl DescribeEventsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeEventsForOrganizationInput`](crate::input::DescribeEventsForOrganizationInput)
    pub fn builder() -> crate::input::describe_events_for_organization_input::Builder {
        crate::input::describe_events_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEventsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeEventsForOrganizationOutput,
        crate::error::DescribeEventsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_for_organization_error(response)
        } else {
            crate::operation_deser::parse_describe_events_for_organization_response(response)
        }
    }
}

/// <p>Returns the event types that meet the specified filter criteria. You can use this API
/// operation to find information about the AWS Health event, such as the category, AWS
/// service, and event code. The metadata for each event appears in the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EventType.html">EventType</a> object. </p>
/// <p>If you don't specify a filter criteria, the API operation returns all event types, in no
/// particular order.  </p>
/// <note>
/// <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEventTypes {
    _private: (),
}
impl DescribeEventTypes {
    /// Creates a new builder-style object to manufacture [`DescribeEventTypesInput`](crate::input::DescribeEventTypesInput)
    pub fn builder() -> crate::input::describe_event_types_input::Builder {
        crate::input::describe_event_types_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeEventTypes {
    type Output = std::result::Result<
        crate::output::DescribeEventTypesOutput,
        crate::error::DescribeEventTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_types_error(response)
        } else {
            crate::operation_deser::parse_describe_event_types_response(response)
        }
    }
}

/// <p>This operation provides status information on enabling or disabling AWS Health to work
/// with your organization. To call this operation, you must sign in as an IAM user, assume
/// an IAM role, or sign in as the root user (not recommended) in the organization's
/// management account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeHealthServiceStatusForOrganization {
    _private: (),
}
impl DescribeHealthServiceStatusForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeHealthServiceStatusForOrganizationInput`](crate::input::DescribeHealthServiceStatusForOrganizationInput)
    pub fn builder() -> crate::input::describe_health_service_status_for_organization_input::Builder
    {
        crate::input::describe_health_service_status_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeHealthServiceStatusForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeHealthServiceStatusForOrganizationOutput,
        crate::error::DescribeHealthServiceStatusForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_health_service_status_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_health_service_status_for_organization_response(
                response,
            )
        }
    }
}

/// <p>Disables AWS Health from working with AWS Organizations. To call this operation, you must sign
/// in as an AWS Identity and Access Management (IAM) user, assume an IAM role, or sign in as the root user (not
/// recommended) in the organization's management account. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating
/// AWS Health events</a> in the
/// <i>AWS Health User Guide</i>.</p>
/// <p>This operation doesn't remove the service-linked role from the management account in your organization. You must use the IAM console, API, or AWS Command Line Interface (AWS CLI) to
/// remove the service-linked role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#delete-service-linked-role">Deleting a Service-Linked Role</a> in the
/// <i>IAM User Guide</i>.</p>
/// <note>
/// <p>You can also disable the organizational feature by using the Organizations <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_DisableAWSServiceAccess.html">DisableAWSServiceAccess</a> API operation. After you call this operation,
/// AWS Health stops aggregating events for all other AWS accounts in your organization.
/// If you call the AWS Health API operations for organizational view, AWS Health returns
/// an error. AWS Health continues to aggregate health events for your AWS
/// account.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisableHealthServiceAccessForOrganization {
    _private: (),
}
impl DisableHealthServiceAccessForOrganization {
    /// Creates a new builder-style object to manufacture [`DisableHealthServiceAccessForOrganizationInput`](crate::input::DisableHealthServiceAccessForOrganizationInput)
    pub fn builder() -> crate::input::disable_health_service_access_for_organization_input::Builder
    {
        crate::input::disable_health_service_access_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisableHealthServiceAccessForOrganization {
    type Output = std::result::Result<
        crate::output::DisableHealthServiceAccessForOrganizationOutput,
        crate::error::DisableHealthServiceAccessForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disable_health_service_access_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_disable_health_service_access_for_organization_response(
                response,
            )
        }
    }
}

/// <p>Enables AWS Health to work with AWS Organizations. You can use the organizational view feature
/// to aggregate events from all AWS accounts in your organization in a centralized location. </p>
/// <p>This operation also creates a service-linked role for the management account in the
/// organization. </p>
/// <note>
/// <p>To call this operation, you must meet the following requirements:</p>
/// <ul>
/// <li>
/// <p>You must have a Business or Enterprise Support plan from <a href="http://aws.amazon.com/premiumsupport/">AWS Support</a> to use the AWS Health
/// API. If you call the AWS Health API from an AWS account that doesn't have a
/// Business or Enterprise Support plan, you receive a
/// <code>SubscriptionRequiredException</code> error.</p>
/// </li>
/// <li>
/// <p>You must have permission to call this operation from the organization's
/// management account. For example IAM policies, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html">AWS Health
/// identity-based policy examples</a>.</p>
/// </li>
/// </ul>
/// </note>
/// <p>If you don't have the required support plan, you can instead use the AWS Health console
/// to enable the organizational view feature. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating
/// AWS Health events</a> in the <i>AWS Health User Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EnableHealthServiceAccessForOrganization {
    _private: (),
}
impl EnableHealthServiceAccessForOrganization {
    /// Creates a new builder-style object to manufacture [`EnableHealthServiceAccessForOrganizationInput`](crate::input::EnableHealthServiceAccessForOrganizationInput)
    pub fn builder() -> crate::input::enable_health_service_access_for_organization_input::Builder {
        crate::input::enable_health_service_access_for_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for EnableHealthServiceAccessForOrganization {
    type Output = std::result::Result<
        crate::output::EnableHealthServiceAccessForOrganizationOutput,
        crate::error::EnableHealthServiceAccessForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_enable_health_service_access_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_enable_health_service_access_for_organization_response(
                response,
            )
        }
    }
}
