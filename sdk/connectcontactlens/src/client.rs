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
    pub fn list_realtime_contact_analysis_segments(
        &self,
    ) -> fluent_builders::ListRealtimeContactAnalysisSegments<C> {
        fluent_builders::ListRealtimeContactAnalysisSegments::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct ListRealtimeContactAnalysisSegments<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_realtime_contact_analysis_segments_input::Builder,
    }
    impl<C> ListRealtimeContactAnalysisSegments<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListRealtimeContactAnalysisSegmentsOutput,
            smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError>,
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
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input);
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn contact_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.contact_id(input);
            self
        }
        pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_contact_id(input);
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
}
