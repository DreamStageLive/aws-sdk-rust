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
    pub fn send_command(&self) -> fluent_builders::SendCommand<C> {
        fluent_builders::SendCommand::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct SendCommand<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::send_command_input::Builder,
    }
    impl<C> SendCommand<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::SendCommandOutput,
            smithy_http::result::SdkError<crate::error::SendCommandError>,
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
        /// <p>Specifies the session token for the current command. A session token is constant
        /// throughout the life of the session.</p>
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This
        /// <code>SessionToken</code> is required for every subsequent command that is issued during
        /// the current session.</p>
        pub fn session_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_token(inp);
            self
        }
        pub fn set_session_token(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_token(inp);
            self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the
        /// response.</p>
        pub fn start_session(mut self, inp: crate::model::StartSessionRequest) -> Self {
            self.inner = self.inner.start_session(inp);
            self
        }
        pub fn set_start_session(
            mut self,
            inp: std::option::Option<crate::model::StartSessionRequest>,
        ) -> Self {
            self.inner = self.inner.set_start_session(inp);
            self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn start_transaction(mut self, inp: crate::model::StartTransactionRequest) -> Self {
            self.inner = self.inner.start_transaction(inp);
            self
        }
        pub fn set_start_transaction(
            mut self,
            inp: std::option::Option<crate::model::StartTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_start_transaction(inp);
            self
        }
        /// <p>Command to end the current session.</p>
        pub fn end_session(mut self, inp: crate::model::EndSessionRequest) -> Self {
            self.inner = self.inner.end_session(inp);
            self
        }
        pub fn set_end_session(
            mut self,
            inp: std::option::Option<crate::model::EndSessionRequest>,
        ) -> Self {
            self.inner = self.inner.set_end_session(inp);
            self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn commit_transaction(mut self, inp: crate::model::CommitTransactionRequest) -> Self {
            self.inner = self.inner.commit_transaction(inp);
            self
        }
        pub fn set_commit_transaction(
            mut self,
            inp: std::option::Option<crate::model::CommitTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_commit_transaction(inp);
            self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn abort_transaction(mut self, inp: crate::model::AbortTransactionRequest) -> Self {
            self.inner = self.inner.abort_transaction(inp);
            self
        }
        pub fn set_abort_transaction(
            mut self,
            inp: std::option::Option<crate::model::AbortTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_abort_transaction(inp);
            self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn execute_statement(mut self, inp: crate::model::ExecuteStatementRequest) -> Self {
            self.inner = self.inner.execute_statement(inp);
            self
        }
        pub fn set_execute_statement(
            mut self,
            inp: std::option::Option<crate::model::ExecuteStatementRequest>,
        ) -> Self {
            self.inner = self.inner.set_execute_statement(inp);
            self
        }
        /// <p>Command to fetch a page.</p>
        pub fn fetch_page(mut self, inp: crate::model::FetchPageRequest) -> Self {
            self.inner = self.inner.fetch_page(inp);
            self
        }
        pub fn set_fetch_page(
            mut self,
            inp: std::option::Option<crate::model::FetchPageRequest>,
        ) -> Self {
            self.inner = self.inner.set_fetch_page(inp);
            self
        }
    }
}