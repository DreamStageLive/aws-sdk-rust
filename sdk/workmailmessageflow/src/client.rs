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
    pub fn get_raw_message_content(&self) -> fluent_builders::GetRawMessageContent<C> {
        fluent_builders::GetRawMessageContent::new(self.handle.clone())
    }
    pub fn put_raw_message_content(&self) -> fluent_builders::PutRawMessageContent<C> {
        fluent_builders::PutRawMessageContent::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct GetRawMessageContent<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_raw_message_content_input::Builder,
    }
    impl<C> GetRawMessageContent<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRawMessageContentOutput,
            smithy_http::result::SdkError<crate::error::GetRawMessageContentError>,
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
        /// <p>The identifier of the email message to retrieve.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_id(input);
            self
        }
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_message_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutRawMessageContent<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::put_raw_message_content_input::Builder,
    }
    impl<C> PutRawMessageContent<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutRawMessageContentOutput,
            smithy_http::result::SdkError<crate::error::PutRawMessageContentError>,
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
        /// <p>The identifier of the email message being updated.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_id(input);
            self
        }
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_message_id(input);
            self
        }
        /// <p>Describes the raw message content of the updated email message.</p>
        pub fn content(mut self, input: crate::model::RawMessageContent) -> Self {
            self.inner = self.inner.content(input);
            self
        }
        pub fn set_content(
            mut self,
            input: std::option::Option<crate::model::RawMessageContent>,
        ) -> Self {
            self.inner = self.inner.set_content(input);
            self
        }
    }
}
