// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn describe_report_creation(&self) -> fluent_builders::DescribeReportCreation<C> {
        fluent_builders::DescribeReportCreation::new(self.handle.clone())
    }
    pub fn get_compliance_summary(&self) -> fluent_builders::GetComplianceSummary<C> {
        fluent_builders::GetComplianceSummary::new(self.handle.clone())
    }
    pub fn get_resources(&self) -> fluent_builders::GetResources<C> {
        fluent_builders::GetResources::new(self.handle.clone())
    }
    pub fn get_tag_keys(&self) -> fluent_builders::GetTagKeys<C> {
        fluent_builders::GetTagKeys::new(self.handle.clone())
    }
    pub fn get_tag_values(&self) -> fluent_builders::GetTagValues<C> {
        fluent_builders::GetTagValues::new(self.handle.clone())
    }
    pub fn start_report_creation(&self) -> fluent_builders::StartReportCreation<C> {
        fluent_builders::StartReportCreation::new(self.handle.clone())
    }
    pub fn tag_resources(&self) -> fluent_builders::TagResources<C> {
        fluent_builders::TagResources::new(self.handle.clone())
    }
    pub fn untag_resources(&self) -> fluent_builders::UntagResources<C> {
        fluent_builders::UntagResources::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DescribeReportCreation<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_report_creation_input::Builder,
    }
    impl<C> DescribeReportCreation<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeReportCreationOutput,
            smithy_http::result::SdkError<crate::error::DescribeReportCreationError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetComplianceSummary<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_compliance_summary_input::Builder,
    }
    impl<C> GetComplianceSummary<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetComplianceSummaryOutput,
            smithy_http::result::SdkError<crate::error::GetComplianceSummaryError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies target identifiers (usually, specific account IDs) to limit the output by.
        /// If you use this parameter, the count of returned noncompliant resources includes only
        /// resources with the specified target IDs.</p>
        pub fn target_id_filters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_id_filters(inp);
            self
        }
        pub fn set_target_id_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_target_id_filters(input);
            self
        }
        /// <p>Specifies a list of AWS Regions to limit the output by. If you use this parameter,
        /// the count of returned noncompliant resources includes only resources in the specified
        /// Regions.</p>
        pub fn region_filters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.region_filters(inp);
            self
        }
        pub fn set_region_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_region_filters(input);
            self
        }
        /// <p>Specifies that you want the response to include information for only resources of the
        /// specified types. The format of each resource type is
        /// <code>service[:resourceType]</code>. For example, specifying a resource type of
        /// <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances).
        /// Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p>
        /// <p>The string for each service name and resource type is the same as that embedded in a
        /// resource's Amazon Resource Name (ARN). Consult the <i>AWS General
        /// Reference</i> for the following:</p>
        /// <ul>
        /// <li>
        /// <p>For a list of service name strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p>
        /// </li>
        /// <li>
        /// <p>For resource type strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example
        /// ARNs</a>.</p>
        /// </li>
        /// <li>
        /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names
        /// (ARNs) and AWS Service Namespaces</a>.</p>
        /// </li>
        /// </ul>
        /// <p>You can specify multiple resource types by using a comma separated array. The array
        /// can include up to 100 items. Note that the length constraint requirement applies to each
        /// resource type filter. </p>
        pub fn resource_type_filters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_type_filters(inp);
            self
        }
        pub fn set_resource_type_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_resource_type_filters(input);
            self
        }
        /// <p>Specifies that you want the response to include information for only resources that
        /// have tags with the specified tag keys. If you use this parameter, the count of returned
        /// noncompliant resources includes only resources that have the specified tag keys.</p>
        pub fn tag_key_filters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_key_filters(inp);
            self
        }
        pub fn set_tag_key_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_key_filters(input);
            self
        }
        /// <p>Specifies a list of attributes to group the counts of noncompliant resources by. If
        /// supplied, the counts are sorted by those attributes.</p>
        pub fn group_by(mut self, inp: impl Into<crate::model::GroupByAttribute>) -> Self {
            self.inner = self.inner.group_by(inp);
            self
        }
        pub fn set_group_by(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::GroupByAttribute>>,
        ) -> Self {
            self.inner = self.inner.set_group_by(input);
            self
        }
        /// <p>Specifies the maximum number of results to be returned in each page. A
        /// query can return fewer than this maximum, even if there are more results still to return. You
        /// should always check the <code>PaginationToken</code> response value to see if there are more
        /// results. You can specify a minimum of 1 and a maximum value of 100.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>Specifies a <code>PaginationToken</code> response value from a
        /// previous request to indicate that you want the next page of results. Leave this parameter empty
        /// in your initial request.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.pagination_token(input);
            self
        }
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_pagination_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetResources<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_resources_input::Builder,
    }
    impl<C> GetResources<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetResourcesOutput,
            smithy_http::result::SdkError<crate::error::GetResourcesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies a <code>PaginationToken</code> response value from a
        /// previous request to indicate that you want the next page of results. Leave this parameter empty
        /// in your initial request.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.pagination_token(input);
            self
        }
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_pagination_token(input);
            self
        }
        /// <p>Specifies a list of TagFilters (keys and values) to restrict the output to only those
        /// resources that have the specified tag and, if included, the specified value. Each
        /// <code>TagFilter</code> must contain a key with values optional. A request can
        /// include up to 50 keys, and each key can include up to 20 values. </p>
        /// <p>Note the following when deciding how to use TagFilters:</p>
        /// <ul>
        /// <li>
        /// <p>If you <i>don't</i> specify a <code>TagFilter</code>, the
        /// response includes all resources that are currently tagged or ever had a tag.
        /// Resources that currently don't have tags are shown with an empty tag set, like
        /// this: <code>"Tags": []</code>.</p>
        /// </li>
        /// <li>
        /// <p>If you specify more than one filter in a single request, the response returns
        /// only those resources that satisfy all filters.</p>
        /// </li>
        /// <li>
        /// <p>If you specify a filter that contains more than one value for a key, the
        /// response returns resources that match any of the specified values for that
        /// key.</p>
        /// </li>
        /// <li>
        /// <p>If you don't specify any values for a key, the response returns resources that
        /// are tagged with that key and any or no value.</p>
        /// <p>For example, for the following filters: <code>filter1= {keyA,{value1}}</code>,
        /// <code>filter2={keyB,{value2,value3,value4}}</code>, <code>filter3=
        /// {keyC}</code>:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>GetResources({filter1})</code> returns resources tagged with
        /// <code>key1=value1</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>GetResources({filter2})</code> returns resources tagged with
        /// <code>key2=value2</code> or <code>key2=value3</code> or
        /// <code>key2=value4</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>GetResources({filter3})</code> returns resources tagged with any
        /// tag with the key <code>key3</code>, and with any or no value</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>GetResources({filter1,filter2,filter3})</code> returns resources
        /// tagged with <code>(key1=value1) and (key2=value2 or key2=value3 or
        /// key2=value4) and (key3, any or no value)</code>
        /// </p>
        /// </li>
        /// </ul>
        /// </li>
        /// </ul>
        pub fn tag_filters(mut self, inp: impl Into<crate::model::TagFilter>) -> Self {
            self.inner = self.inner.tag_filters(inp);
            self
        }
        pub fn set_tag_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::TagFilter>>,
        ) -> Self {
            self.inner = self.inner.set_tag_filters(input);
            self
        }
        /// <p>Specifies the maximum number of results to be returned in each page. A
        /// query can return fewer than this maximum, even if there are more results still to return. You
        /// should always check the <code>PaginationToken</code> response value to see if there are more
        /// results. You can specify a minimum of 1 and a maximum value of 100.</p>
        pub fn resources_per_page(mut self, input: i32) -> Self {
            self.inner = self.inner.resources_per_page(input);
            self
        }
        pub fn set_resources_per_page(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_resources_per_page(input);
            self
        }
        /// <p>AWS recommends using <code>ResourcesPerPage</code> instead of this parameter.</p>
        /// <p>A limit that restricts the number of tags (key and value pairs) returned by
        /// <code>GetResources</code> in paginated output. A resource with no tags is counted as
        /// having one tag (one key and value pair).</p>
        /// <p>
        /// <code>GetResources</code> does not split a resource and its associated tags across
        /// pages. If the specified <code>TagsPerPage</code> would cause such a break, a
        /// <code>PaginationToken</code> is returned in place of the affected resource and its
        /// tags. Use that token in another request to get the remaining data. For example, if you
        /// specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources
        /// with 10 tags each (meaning that each resource has 10 key and value pairs), the output
        /// will consist of three pages. The first page displays the first 10 resources, each with
        /// its 10 tags. The second page displays the next 10 resources, each with its 10 tags. The
        /// third page displays the remaining 2 resources, each with its 10 tags.</p>
        /// <p>You can set <code>TagsPerPage</code> to a minimum of 100 items up to a maximum of 500
        /// items.</p>
        pub fn tags_per_page(mut self, input: i32) -> Self {
            self.inner = self.inner.tags_per_page(input);
            self
        }
        pub fn set_tags_per_page(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_tags_per_page(input);
            self
        }
        /// <p>Specifies the resource types that you want included in the response. The format of
        /// each resource type is <code>service[:resourceType]</code>. For example, specifying a
        /// resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2
        /// instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2
        /// instances. </p>
        /// <p>The string for each service name and resource type is the same as that embedded in a
        /// resource's Amazon Resource Name (ARN). Consult the <i>AWS General
        /// Reference</i> for the following:</p>
        /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and
        /// AWS Service Namespaces</a>.</p>
        /// <p>You can specify multiple resource types by using an array. The array can include up to
        /// 100 items. Note that the length constraint requirement applies to each resource type
        /// filter. </p>
        pub fn resource_type_filters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_type_filters(inp);
            self
        }
        pub fn set_resource_type_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_resource_type_filters(input);
            self
        }
        /// <p>Specifies whether to include details regarding the compliance with the effective tag
        /// policy. Set this to <code>true</code> to determine whether resources are compliant with
        /// the tag policy and to get details.</p>
        pub fn include_compliance_details(mut self, input: bool) -> Self {
            self.inner = self.inner.include_compliance_details(input);
            self
        }
        pub fn set_include_compliance_details(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_compliance_details(input);
            self
        }
        /// <p>Specifies whether to exclude resources that are compliant with the tag policy. Set
        /// this to <code>true</code> if you are interested in retrieving information on
        /// noncompliant resources only.</p>
        /// <p>You can use this parameter only if the <code>IncludeComplianceDetails</code> parameter
        /// is also set to <code>true</code>.</p>
        pub fn exclude_compliant_resources(mut self, input: bool) -> Self {
            self.inner = self.inner.exclude_compliant_resources(input);
            self
        }
        pub fn set_exclude_compliant_resources(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_exclude_compliant_resources(input);
            self
        }
        /// <p>Specifies a list of ARNs of resources for which you want to retrieve tag data. You
        /// can't specify both this parameter and any of the pagination parameters
        /// (<code>ResourcesPerPage</code>, <code>TagsPerPage</code>,
        /// <code>PaginationToken</code>) in the same request. If you specify both, you get an
        /// <code>Invalid Parameter</code> exception.</p>
        /// <p>If a resource specified by this parameter doesn't exist, it doesn't generate an error;
        /// it simply isn't included in the response.</p>
        /// <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information,
        /// see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon
        /// Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS
        /// General Reference</i>.</p>
        pub fn resource_arn_list(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn_list(inp);
            self
        }
        pub fn set_resource_arn_list(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_resource_arn_list(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetTagKeys<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_tag_keys_input::Builder,
    }
    impl<C> GetTagKeys<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetTagKeysOutput,
            smithy_http::result::SdkError<crate::error::GetTagKeysError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies a <code>PaginationToken</code> response value from a
        /// previous request to indicate that you want the next page of results. Leave this parameter empty
        /// in your initial request.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.pagination_token(input);
            self
        }
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_pagination_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetTagValues<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_tag_values_input::Builder,
    }
    impl<C> GetTagValues<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetTagValuesOutput,
            smithy_http::result::SdkError<crate::error::GetTagValuesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies a <code>PaginationToken</code> response value from a
        /// previous request to indicate that you want the next page of results. Leave this parameter empty
        /// in your initial request.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.pagination_token(input);
            self
        }
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_pagination_token(input);
            self
        }
        /// <p>Specifies the tag key for which you want to list all existing values that are
        /// currently used in the specified AWS Region for the calling AWS account.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.key(input);
            self
        }
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_key(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartReportCreation<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::start_report_creation_input::Builder,
    }
    impl<C> StartReportCreation<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartReportCreationOutput,
            smithy_http::result::SdkError<crate::error::StartReportCreationError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the Amazon S3 bucket where the report will be stored; for example:</p>
        /// <p>
        /// <code>awsexamplebucket</code>
        /// </p>
        /// <p>For more information on S3 bucket requirements, including an example bucket policy,
        /// see the example S3 bucket policy on this page.</p>
        pub fn s3_bucket(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.s3_bucket(input);
            self
        }
        pub fn set_s3_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_s3_bucket(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResources<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::tag_resources_input::Builder,
    }
    impl<C> TagResources<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::TagResourcesOutput,
            smithy_http::result::SdkError<crate::error::TagResourcesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies the list of ARNs of the resources that you want to apply tags to.</p>
        /// <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information,
        /// see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon
        /// Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS
        /// General Reference</i>.</p>
        pub fn resource_arn_list(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn_list(inp);
            self
        }
        pub fn set_resource_arn_list(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_resource_arn_list(input);
            self
        }
        /// <p>Specifies a list of tags that you want to add to the specified resources. A tag
        /// consists of a key and a value that you define.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UntagResources<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::untag_resources_input::Builder,
    }
    impl<C> UntagResources<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UntagResourcesOutput,
            smithy_http::result::SdkError<crate::error::UntagResourcesError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Specifies a list of ARNs of the resources that you want to remove tags from.</p>
        /// <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information,
        /// see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon
        /// Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS
        /// General Reference</i>.</p>
        pub fn resource_arn_list(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn_list(inp);
            self
        }
        pub fn set_resource_arn_list(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_resource_arn_list(input);
            self
        }
        /// <p>Specifies a list of tag keys that you want to remove from the specified
        /// resources.</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
}