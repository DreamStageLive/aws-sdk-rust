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
    pub fn create_home_region_control(&self) -> fluent_builders::CreateHomeRegionControl<C> {
        fluent_builders::CreateHomeRegionControl::new(self.handle.clone())
    }
    pub fn describe_home_region_controls(&self) -> fluent_builders::DescribeHomeRegionControls<C> {
        fluent_builders::DescribeHomeRegionControls::new(self.handle.clone())
    }
    pub fn get_home_region(&self) -> fluent_builders::GetHomeRegion<C> {
        fluent_builders::GetHomeRegion::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateHomeRegionControl<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_home_region_control_input::Builder,
    }
    impl<C> CreateHomeRegionControl<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateHomeRegionControlOutput,
            smithy_http::result::SdkError<crate::error::CreateHomeRegionControlError>,
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
        /// <p>The name of the home region of the calling account.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.home_region(input);
            self
        }
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_home_region(input);
            self
        }
        /// <p>The account for which this command sets up a home region control. The <code>Target</code>
        /// is always of type <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.inner = self.inner.target(input);
            self
        }
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.inner = self.inner.set_target(input);
            self
        }
        /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether
        /// the caller has permission to make the call.</p>
        pub fn dry_run(mut self, input: bool) -> Self {
            self.inner = self.inner.dry_run(input);
            self
        }
        pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_dry_run(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeHomeRegionControls<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_home_region_controls_input::Builder,
    }
    impl<C> DescribeHomeRegionControls<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeHomeRegionControlsOutput,
            smithy_http::result::SdkError<crate::error::DescribeHomeRegionControlsError>,
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
        /// <p>The <code>ControlID</code> is a unique identifier string of your
        /// <code>HomeRegionControl</code> object.</p>
        pub fn control_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.control_id(input);
            self
        }
        pub fn set_control_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_control_id(input);
            self
        }
        /// <p>The name of the home region you'd like to view.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.home_region(input);
            self
        }
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_home_region(input);
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which
        /// is always of type <code>ACCOUNT</code>. It applies the home region to the current
        /// <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.inner = self.inner.target(input);
            self
        }
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.inner = self.inner.set_target(input);
            self
        }
        /// <p>The maximum number of filtering results to display per page. </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available.
        /// To retrieve the next page of results, make the call again using the returned token in
        /// <code>NextToken</code>.</p>
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
    pub struct GetHomeRegion<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_home_region_input::Builder,
    }
    impl<C> GetHomeRegion<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetHomeRegionOutput,
            smithy_http::result::SdkError<crate::error::GetHomeRegionError>,
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
}
