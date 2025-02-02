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
    pub fn create_changeset(&self) -> fluent_builders::CreateChangeset<C> {
        fluent_builders::CreateChangeset::new(self.handle.clone())
    }
    pub fn get_programmatic_access_credentials(
        &self,
    ) -> fluent_builders::GetProgrammaticAccessCredentials<C> {
        fluent_builders::GetProgrammaticAccessCredentials::new(self.handle.clone())
    }
    pub fn get_working_location(&self) -> fluent_builders::GetWorkingLocation<C> {
        fluent_builders::GetWorkingLocation::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateChangeset<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_changeset_input::Builder,
    }
    impl<C> CreateChangeset<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateChangesetOutput,
            smithy_http::result::SdkError<crate::error::CreateChangesetError>,
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
        /// <p>The unique identifier for the FinSpace dataset in which the changeset will be
        /// created.</p>
        pub fn dataset_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_id(input);
            self
        }
        pub fn set_dataset_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_id(input);
            self
        }
        /// <p>Option to indicate how a changeset will be applied to a dataset.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>REPLACE</code> - Changeset will be considered as a replacement to all prior
        /// loaded changesets.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>APPEND</code> - Changeset will be considered as an addition to the end of all
        /// prior loaded changesets.</p>
        /// </li>
        /// </ul>
        pub fn change_type(mut self, input: crate::model::ChangeType) -> Self {
            self.inner = self.inner.change_type(input);
            self
        }
        pub fn set_change_type(
            mut self,
            input: std::option::Option<crate::model::ChangeType>,
        ) -> Self {
            self.inner = self.inner.set_change_type(input);
            self
        }
        /// <p>Type of the data source from which the files to create the changeset will be
        /// sourced.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>S3</code> - Amazon S3.</p>
        /// </li>
        /// </ul>
        pub fn source_type(mut self, input: crate::model::SourceType) -> Self {
            self.inner = self.inner.source_type(input);
            self
        }
        pub fn set_source_type(
            mut self,
            input: std::option::Option<crate::model::SourceType>,
        ) -> Self {
            self.inner = self.inner.set_source_type(input);
            self
        }
        /// <p>Source path from which the files to create the changeset will be sourced.</p>
        pub fn source_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.source_params(k, v);
            self
        }
        pub fn set_source_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_source_params(input);
            self
        }
        /// <p>Format type of the input files being loaded into the changeset.</p>
        pub fn format_type(mut self, input: crate::model::FormatType) -> Self {
            self.inner = self.inner.format_type(input);
            self
        }
        pub fn set_format_type(
            mut self,
            input: std::option::Option<crate::model::FormatType>,
        ) -> Self {
            self.inner = self.inner.set_format_type(input);
            self
        }
        /// <p>Options that define the structure of the source file(s).</p>
        pub fn format_params(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.format_params(k, v);
            self
        }
        pub fn set_format_params(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_format_params(input);
            self
        }
        /// <p>Metadata tags to apply to this changeset.</p>
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
    pub struct GetProgrammaticAccessCredentials<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_programmatic_access_credentials_input::Builder,
    }
    impl<C> GetProgrammaticAccessCredentials<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetProgrammaticAccessCredentialsOutput,
            smithy_http::result::SdkError<crate::error::GetProgrammaticAccessCredentialsError>,
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
        /// <p>The time duration in which the credentials remain valid. </p>
        pub fn duration_in_minutes(mut self, input: i64) -> Self {
            self.inner = self.inner.duration_in_minutes(input);
            self
        }
        pub fn set_duration_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_duration_in_minutes(input);
            self
        }
        /// <p>The habanero environment identifier.</p>
        pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_id(input);
            self
        }
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetWorkingLocation<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_working_location_input::Builder,
    }
    impl<C> GetWorkingLocation<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetWorkingLocationOutput,
            smithy_http::result::SdkError<crate::error::GetWorkingLocationError>,
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
        /// <p>Specify the type of the working location.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>SAGEMAKER</code> - Use the Amazon S3 location as a temporary location to store data content when
        /// working with FinSpace Notebooks that run on SageMaker studio.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>INGESTION</code> - Use the Amazon S3 location as a staging location to copy your
        /// data content and then use the location with the changeset creation operation.</p>
        /// </li>
        /// </ul>
        pub fn location_type(mut self, input: crate::model::LocationType) -> Self {
            self.inner = self.inner.location_type(input);
            self
        }
        pub fn set_location_type(
            mut self,
            input: std::option::Option<crate::model::LocationType>,
        ) -> Self {
            self.inner = self.inner.set_location_type(input);
            self
        }
    }
}
