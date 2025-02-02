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
    pub fn create_lifecycle_policy(&self) -> fluent_builders::CreateLifecyclePolicy<C> {
        fluent_builders::CreateLifecyclePolicy::new(self.handle.clone())
    }
    pub fn delete_lifecycle_policy(&self) -> fluent_builders::DeleteLifecyclePolicy<C> {
        fluent_builders::DeleteLifecyclePolicy::new(self.handle.clone())
    }
    pub fn get_lifecycle_policies(&self) -> fluent_builders::GetLifecyclePolicies<C> {
        fluent_builders::GetLifecyclePolicies::new(self.handle.clone())
    }
    pub fn get_lifecycle_policy(&self) -> fluent_builders::GetLifecyclePolicy<C> {
        fluent_builders::GetLifecyclePolicy::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    pub fn update_lifecycle_policy(&self) -> fluent_builders::UpdateLifecyclePolicy<C> {
        fluent_builders::UpdateLifecyclePolicy::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateLifecyclePolicy<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_lifecycle_policy_input::Builder,
    }
    impl<C> CreateLifecyclePolicy<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::CreateLifecyclePolicyError>,
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
        /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
        /// the lifecycle policy.</p>
        pub fn execution_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.execution_role_arn(input);
            self
        }
        pub fn set_execution_role_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_execution_role_arn(input);
            self
        }
        /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are
        /// supported.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(input);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The desired activation state of the lifecycle policy after creation.</p>
        pub fn state(mut self, input: crate::model::SettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(input);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::SettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// <p>The configuration details of the lifecycle policy.</p>
        pub fn policy_details(mut self, input: crate::model::PolicyDetails) -> Self {
            self.inner = self.inner.policy_details(input);
            self
        }
        pub fn set_policy_details(
            mut self,
            input: std::option::Option<crate::model::PolicyDetails>,
        ) -> Self {
            self.inner = self.inner.set_policy_details(input);
            self
        }
        /// <p>The tags to apply to the lifecycle policy during creation.</p>
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
    pub struct DeleteLifecyclePolicy<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_lifecycle_policy_input::Builder,
    }
    impl<C> DeleteLifecyclePolicy<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::DeleteLifecyclePolicyError>,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(input);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetLifecyclePolicies<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_lifecycle_policies_input::Builder,
    }
    impl<C> GetLifecyclePolicies<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetLifecyclePoliciesOutput,
            smithy_http::result::SdkError<crate::error::GetLifecyclePoliciesError>,
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
        /// <p>The identifiers of the data lifecycle policies.</p>
        pub fn policy_ids(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_ids(inp);
            self
        }
        pub fn set_policy_ids(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_policy_ids(input);
            self
        }
        /// <p>The activation state.</p>
        pub fn state(mut self, input: crate::model::GettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(input);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::GettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// <p>The resource type.</p>
        pub fn resource_types(mut self, inp: impl Into<crate::model::ResourceTypeValues>) -> Self {
            self.inner = self.inner.resource_types(inp);
            self
        }
        pub fn set_resource_types(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResourceTypeValues>>,
        ) -> Self {
            self.inner = self.inner.set_resource_types(input);
            self
        }
        /// <p>The target tag for a policy.</p>
        /// <p>Tags are strings in the format <code>key=value</code>.</p>
        pub fn target_tags(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_tags(inp);
            self
        }
        pub fn set_target_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_target_tags(input);
            self
        }
        /// <p>The tags to add to objects created by the policy.</p>
        /// <p>Tags are strings in the format <code>key=value</code>.</p>
        /// <p>These user-defined tags are added in addition to the AWS-added lifecycle tags.</p>
        pub fn tags_to_add(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tags_to_add(inp);
            self
        }
        pub fn set_tags_to_add(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tags_to_add(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetLifecyclePolicy<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_lifecycle_policy_input::Builder,
    }
    impl<C> GetLifecyclePolicy<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::GetLifecyclePolicyError>,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(input);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C> ListTagsForResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListTagsForResourceOutput,
            smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C> TagResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::TagResourceOutput,
            smithy_http::result::SdkError<crate::error::TagResourceError>,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>One or more tags.</p>
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
    pub struct UntagResource<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C> UntagResource<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UntagResourceOutput,
            smithy_http::result::SdkError<crate::error::UntagResourceError>,
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
        /// <p>The Amazon Resource Name (ARN) of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>The tag keys.</p>
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
    #[derive(std::fmt::Debug)]
    pub struct UpdateLifecyclePolicy<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::update_lifecycle_policy_input::Builder,
    }
    impl<C> UpdateLifecyclePolicy<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateLifecyclePolicyOutput,
            smithy_http::result::SdkError<crate::error::UpdateLifecyclePolicyError>,
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
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.policy_id(input);
            self
        }
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_policy_id(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
        /// the lifecycle policy.</p>
        pub fn execution_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.execution_role_arn(input);
            self
        }
        pub fn set_execution_role_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_execution_role_arn(input);
            self
        }
        /// <p>The desired activation state of the lifecycle policy after creation.</p>
        pub fn state(mut self, input: crate::model::SettablePolicyStateValues) -> Self {
            self.inner = self.inner.state(input);
            self
        }
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::SettablePolicyStateValues>,
        ) -> Self {
            self.inner = self.inner.set_state(input);
            self
        }
        /// <p>A description of the lifecycle policy.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(input);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The configuration of the lifecycle policy. You cannot update the policy type or the
        /// resource type.</p>
        pub fn policy_details(mut self, input: crate::model::PolicyDetails) -> Self {
            self.inner = self.inner.policy_details(input);
            self
        }
        pub fn set_policy_details(
            mut self,
            input: std::option::Option<crate::model::PolicyDetails>,
        ) -> Self {
            self.inner = self.inner.set_policy_details(input);
            self
        }
    }
}
