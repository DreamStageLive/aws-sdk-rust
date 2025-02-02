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
    pub fn close_tunnel(&self) -> fluent_builders::CloseTunnel<C> {
        fluent_builders::CloseTunnel::new(self.handle.clone())
    }
    pub fn describe_tunnel(&self) -> fluent_builders::DescribeTunnel<C> {
        fluent_builders::DescribeTunnel::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn list_tunnels(&self) -> fluent_builders::ListTunnels<C> {
        fluent_builders::ListTunnels::new(self.handle.clone())
    }
    pub fn open_tunnel(&self) -> fluent_builders::OpenTunnel<C> {
        fluent_builders::OpenTunnel::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CloseTunnel<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::close_tunnel_input::Builder,
    }
    impl<C> CloseTunnel<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CloseTunnelOutput,
            smithy_http::result::SdkError<crate::error::CloseTunnelError>,
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
        /// <p>The ID of the tunnel to close.</p>
        pub fn tunnel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tunnel_id(input);
            self
        }
        pub fn set_tunnel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_tunnel_id(input);
            self
        }
        /// <p>When set to true, AWS IoT Secure Tunneling deletes the tunnel data
        /// immediately.</p>
        pub fn delete(mut self, input: bool) -> Self {
            self.inner = self.inner.delete(input);
            self
        }
        pub fn set_delete(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_delete(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeTunnel<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_tunnel_input::Builder,
    }
    impl<C> DescribeTunnel<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeTunnelOutput,
            smithy_http::result::SdkError<crate::error::DescribeTunnelError>,
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
        /// <p>The tunnel to describe.</p>
        pub fn tunnel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tunnel_id(input);
            self
        }
        pub fn set_tunnel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_tunnel_id(input);
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
        /// <p>The resource ARN.</p>
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
    pub struct ListTunnels<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_tunnels_input::Builder,
    }
    impl<C> ListTunnels<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListTunnelsOutput,
            smithy_http::result::SdkError<crate::error::ListTunnelsError>,
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
        /// <p>The name of the IoT thing associated with the destination device.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(input);
            self
        }
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The maximum number of results to return at once.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>A token to retrieve the next set of results.</p>
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
    pub struct OpenTunnel<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::open_tunnel_input::Builder,
    }
    impl<C> OpenTunnel<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::OpenTunnelOutput,
            smithy_http::result::SdkError<crate::error::OpenTunnelError>,
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
        /// <p>A short text description of the tunnel. </p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(input);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>A collection of tag metadata.</p>
        pub fn tags(mut self, inp: impl Into<crate::model::Tag>) -> Self {
            self.inner = self.inner.tags(inp);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
        /// <p>The destination configuration for the OpenTunnel request.</p>
        pub fn destination_config(mut self, input: crate::model::DestinationConfig) -> Self {
            self.inner = self.inner.destination_config(input);
            self
        }
        pub fn set_destination_config(
            mut self,
            input: std::option::Option<crate::model::DestinationConfig>,
        ) -> Self {
            self.inner = self.inner.set_destination_config(input);
            self
        }
        /// <p>Timeout configuration for a tunnel.</p>
        pub fn timeout_config(mut self, input: crate::model::TimeoutConfig) -> Self {
            self.inner = self.inner.timeout_config(input);
            self
        }
        pub fn set_timeout_config(
            mut self,
            input: std::option::Option<crate::model::TimeoutConfig>,
        ) -> Self {
            self.inner = self.inner.set_timeout_config(input);
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
        /// <p>The tags for the resource.</p>
        pub fn tags(mut self, inp: impl Into<crate::model::Tag>) -> Self {
            self.inner = self.inner.tags(inp);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
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
        /// <p>The resource ARN.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// <p>The keys of the tags to remove.</p>
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
