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
    pub fn claim_devices_by_claim_code(&self) -> fluent_builders::ClaimDevicesByClaimCode<C> {
        fluent_builders::ClaimDevicesByClaimCode::new(self.handle.clone())
    }
    pub fn describe_device(&self) -> fluent_builders::DescribeDevice<C> {
        fluent_builders::DescribeDevice::new(self.handle.clone())
    }
    pub fn finalize_device_claim(&self) -> fluent_builders::FinalizeDeviceClaim<C> {
        fluent_builders::FinalizeDeviceClaim::new(self.handle.clone())
    }
    pub fn get_device_methods(&self) -> fluent_builders::GetDeviceMethods<C> {
        fluent_builders::GetDeviceMethods::new(self.handle.clone())
    }
    pub fn initiate_device_claim(&self) -> fluent_builders::InitiateDeviceClaim<C> {
        fluent_builders::InitiateDeviceClaim::new(self.handle.clone())
    }
    pub fn invoke_device_method(&self) -> fluent_builders::InvokeDeviceMethod<C> {
        fluent_builders::InvokeDeviceMethod::new(self.handle.clone())
    }
    pub fn list_device_events(&self) -> fluent_builders::ListDeviceEvents<C> {
        fluent_builders::ListDeviceEvents::new(self.handle.clone())
    }
    pub fn list_devices(&self) -> fluent_builders::ListDevices<C> {
        fluent_builders::ListDevices::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn unclaim_device(&self) -> fluent_builders::UnclaimDevice<C> {
        fluent_builders::UnclaimDevice::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    pub fn update_device_state(&self) -> fluent_builders::UpdateDeviceState<C> {
        fluent_builders::UpdateDeviceState::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct ClaimDevicesByClaimCode<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::claim_devices_by_claim_code_input::Builder,
    }
    impl<C> ClaimDevicesByClaimCode<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ClaimDevicesByClaimCodeOutput,
            smithy_http::result::SdkError<crate::error::ClaimDevicesByClaimCodeError>,
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
        /// <p>The claim code, starting with "C-", as provided by the device manufacturer.</p>
        pub fn claim_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.claim_code(input);
            self
        }
        pub fn set_claim_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_claim_code(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeDevice<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_device_input::Builder,
    }
    impl<C> DescribeDevice<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeDeviceOutput,
            smithy_http::result::SdkError<crate::error::DescribeDeviceError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct FinalizeDeviceClaim<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::finalize_device_claim_input::Builder,
    }
    impl<C> FinalizeDeviceClaim<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::FinalizeDeviceClaimOutput,
            smithy_http::result::SdkError<crate::error::FinalizeDeviceClaimError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>A collection of key/value pairs defining the resource tags. For example, {
        /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
        /// Tagging Strategies</a>.</p><p>
        /// </p>
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
    pub struct GetDeviceMethods<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_device_methods_input::Builder,
    }
    impl<C> GetDeviceMethods<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDeviceMethodsOutput,
            smithy_http::result::SdkError<crate::error::GetDeviceMethodsError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct InitiateDeviceClaim<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::initiate_device_claim_input::Builder,
    }
    impl<C> InitiateDeviceClaim<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InitiateDeviceClaimOutput,
            smithy_http::result::SdkError<crate::error::InitiateDeviceClaimError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct InvokeDeviceMethod<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::invoke_device_method_input::Builder,
    }
    impl<C> InvokeDeviceMethod<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InvokeDeviceMethodOutput,
            smithy_http::result::SdkError<crate::error::InvokeDeviceMethodError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>The device method to invoke.</p>
        pub fn device_method(mut self, input: crate::model::DeviceMethod) -> Self {
            self.inner = self.inner.device_method(input);
            self
        }
        pub fn set_device_method(
            mut self,
            input: std::option::Option<crate::model::DeviceMethod>,
        ) -> Self {
            self.inner = self.inner.set_device_method(input);
            self
        }
        /// <p>A JSON encoded string containing the device method request parameters.</p>
        pub fn device_method_parameters(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_method_parameters(input);
            self
        }
        pub fn set_device_method_parameters(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_method_parameters(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListDeviceEvents<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_device_events_input::Builder,
    }
    impl<C> ListDeviceEvents<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListDeviceEventsOutput,
            smithy_http::result::SdkError<crate::error::ListDeviceEventsError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>The start date for the device event query, in ISO8061 format. For example,
        /// 2018-03-28T15:45:12.880Z
        /// </p>
        pub fn from_time_stamp(mut self, input: smithy_types::Instant) -> Self {
            self.inner = self.inner.from_time_stamp(input);
            self
        }
        pub fn set_from_time_stamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.inner = self.inner.set_from_time_stamp(input);
            self
        }
        /// <p>The maximum number of results to return per request. If not set, a default value of
        /// 100 is used.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The end date for the device event query, in ISO8061 format. For example,
        /// 2018-03-28T15:45:12.880Z
        /// </p>
        pub fn to_time_stamp(mut self, input: smithy_types::Instant) -> Self {
            self.inner = self.inner.to_time_stamp(input);
            self
        }
        pub fn set_to_time_stamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.inner = self.inner.set_to_time_stamp(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListDevices<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_devices_input::Builder,
    }
    impl<C> ListDevices<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListDevicesOutput,
            smithy_http::result::SdkError<crate::error::ListDevicesError>,
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
        /// <p>The type of the device, such as "button".</p>
        pub fn device_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_type(input);
            self
        }
        pub fn set_device_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_type(input);
            self
        }
        /// <p>The maximum number of results to return per request. If not set, a default value of
        /// 100 is used.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
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
        /// <p>A collection of key/value pairs defining the resource tags. For example, {
        /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
        /// Tagging Strategies</a>.</p><p>
        /// </p>
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
    pub struct UnclaimDevice<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::unclaim_device_input::Builder,
    }
    impl<C> UnclaimDevice<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UnclaimDeviceOutput,
            smithy_http::result::SdkError<crate::error::UnclaimDeviceError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
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
        /// <p>A collections of tag keys. For example, {"key1","key2"}</p>
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
    pub struct UpdateDeviceState<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::update_device_state_input::Builder,
    }
    impl<C> UpdateDeviceState<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateDeviceStateOutput,
            smithy_http::result::SdkError<crate::error::UpdateDeviceStateError>,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(input);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>If true, the device is enabled. If false, the device is
        /// disabled.</p>
        pub fn enabled(mut self, input: bool) -> Self {
            self.inner = self.inner.enabled(input);
            self
        }
        pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_enabled(input);
            self
        }
    }
}
