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
    pub fn create_application(&self) -> fluent_builders::CreateApplication<C> {
        fluent_builders::CreateApplication::new(self.handle.clone())
    }
    pub fn delete_application(&self) -> fluent_builders::DeleteApplication<C> {
        fluent_builders::DeleteApplication::new(self.handle.clone())
    }
    pub fn describe_application(&self) -> fluent_builders::DescribeApplication<C> {
        fluent_builders::DescribeApplication::new(self.handle.clone())
    }
    pub fn list_applications(&self) -> fluent_builders::ListApplications<C> {
        fluent_builders::ListApplications::new(self.handle.clone())
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
    pub fn update_application(&self) -> fluent_builders::UpdateApplication<C> {
        fluent_builders::UpdateApplication::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateApplication<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_application_input::Builder,
    }
    impl<C> CreateApplication<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateApplicationOutput,
            smithy_http::result::SdkError<crate::error::CreateApplicationError>,
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
        /// <p>The name of the web application.</p>
        pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_name(input);
            self
        }
        pub fn set_application_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_name(input);
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_description(input);
            self
        }
        pub fn set_application_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_description(input);
            self
        }
        /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request.
        /// Don't reuse this client token if a new idempotent request is required.</p>
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_token(input);
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_token(input);
            self
        }
        /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p>
        /// <note>
        /// <p>The name of the role must be in the form <code>AWSIotFleetHub_<i>random_string</i>
        /// </code>.</p>
        /// </note>
        pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.role_arn(input);
            self
        }
        pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_role_arn(input);
            self
        }
        /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
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
    pub struct DeleteApplication<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_application_input::Builder,
    }
    impl<C> DeleteApplication<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteApplicationOutput,
            smithy_http::result::SdkError<crate::error::DeleteApplicationError>,
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
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_id(input);
            self
        }
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_id(input);
            self
        }
        /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request.
        /// Don't reuse this client token if a new idempotent request is required.</p>
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_token(input);
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeApplication<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_application_input::Builder,
    }
    impl<C> DescribeApplication<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeApplicationOutput,
            smithy_http::result::SdkError<crate::error::DescribeApplicationError>,
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
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_id(input);
            self
        }
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListApplications<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_applications_input::Builder,
    }
    impl<C> ListApplications<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListApplicationsOutput,
            smithy_http::result::SdkError<crate::error::ListApplicationsError>,
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
        /// <p>A token used to get the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
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
        /// <p>The ARN of the resource.</p>
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
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>The new or modified tags for the resource.</p>
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
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>A list of the keys of the tags to be removed from the resource.</p>
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
    pub struct UpdateApplication<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::update_application_input::Builder,
    }
    impl<C> UpdateApplication<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateApplicationOutput,
            smithy_http::result::SdkError<crate::error::UpdateApplicationError>,
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
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_id(input);
            self
        }
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_id(input);
            self
        }
        /// <p>The name of the web application.</p>
        pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_name(input);
            self
        }
        pub fn set_application_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_name(input);
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_description(input);
            self
        }
        pub fn set_application_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_description(input);
            self
        }
        /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request.
        /// Don't reuse this client token if a new idempotent request is required.</p>
        pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_token(input);
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_token(input);
            self
        }
    }
}
