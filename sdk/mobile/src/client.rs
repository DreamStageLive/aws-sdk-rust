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
    pub fn create_project(&self) -> fluent_builders::CreateProject<C> {
        fluent_builders::CreateProject::new(self.handle.clone())
    }
    pub fn delete_project(&self) -> fluent_builders::DeleteProject<C> {
        fluent_builders::DeleteProject::new(self.handle.clone())
    }
    pub fn describe_bundle(&self) -> fluent_builders::DescribeBundle<C> {
        fluent_builders::DescribeBundle::new(self.handle.clone())
    }
    pub fn describe_project(&self) -> fluent_builders::DescribeProject<C> {
        fluent_builders::DescribeProject::new(self.handle.clone())
    }
    pub fn export_bundle(&self) -> fluent_builders::ExportBundle<C> {
        fluent_builders::ExportBundle::new(self.handle.clone())
    }
    pub fn export_project(&self) -> fluent_builders::ExportProject<C> {
        fluent_builders::ExportProject::new(self.handle.clone())
    }
    pub fn list_bundles(&self) -> fluent_builders::ListBundles<C> {
        fluent_builders::ListBundles::new(self.handle.clone())
    }
    pub fn list_projects(&self) -> fluent_builders::ListProjects<C> {
        fluent_builders::ListProjects::new(self.handle.clone())
    }
    pub fn update_project(&self) -> fluent_builders::UpdateProject<C> {
        fluent_builders::UpdateProject::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateProject<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::create_project_input::Builder,
    }
    impl<C> CreateProject<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateProjectOutput,
            smithy_http::result::SdkError<crate::error::CreateProjectError>,
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
        /// <p>
        /// Name of the project.
        /// </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(input);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>
        /// Default region where project resources should be created.
        /// </p>
        pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.region(input);
            self
        }
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_region(input);
            self
        }
        /// <p>
        /// ZIP or YAML file which contains configuration settings to be used when creating
        /// the project. This may be the contents of the file downloaded from the URL provided
        /// in an export project operation.
        /// </p>
        pub fn contents(mut self, input: smithy_types::Blob) -> Self {
            self.inner = self.inner.contents(input);
            self
        }
        pub fn set_contents(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_contents(input);
            self
        }
        /// <p>
        /// Unique identifier for an exported snapshot of project configuration. This
        /// snapshot identifier is included in the share URL when a project is exported.
        /// </p>
        pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(input);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteProject<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_project_input::Builder,
    }
    impl<C> DeleteProject<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteProjectOutput,
            smithy_http::result::SdkError<crate::error::DeleteProjectError>,
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
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(input);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeBundle<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_bundle_input::Builder,
    }
    impl<C> DescribeBundle<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeBundleOutput,
            smithy_http::result::SdkError<crate::error::DescribeBundleError>,
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
        /// <p>
        /// Unique bundle identifier.
        /// </p>
        pub fn bundle_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bundle_id(input);
            self
        }
        pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bundle_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeProject<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_project_input::Builder,
    }
    impl<C> DescribeProject<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeProjectOutput,
            smithy_http::result::SdkError<crate::error::DescribeProjectError>,
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
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(input);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
        /// <p>
        /// If set to true, causes AWS Mobile Hub to synchronize information from other services, e.g., update state of AWS CloudFormation stacks in the AWS Mobile Hub project.
        /// </p>
        pub fn sync_from_resources(mut self, input: bool) -> Self {
            self.inner = self.inner.sync_from_resources(input);
            self
        }
        pub fn set_sync_from_resources(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_sync_from_resources(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ExportBundle<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::export_bundle_input::Builder,
    }
    impl<C> ExportBundle<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ExportBundleOutput,
            smithy_http::result::SdkError<crate::error::ExportBundleError>,
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
        /// <p>
        /// Unique bundle identifier.
        /// </p>
        pub fn bundle_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bundle_id(input);
            self
        }
        pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bundle_id(input);
            self
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(input);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
        /// <p>
        /// Developer desktop or target application platform.
        /// </p>
        pub fn platform(mut self, input: crate::model::Platform) -> Self {
            self.inner = self.inner.platform(input);
            self
        }
        pub fn set_platform(mut self, input: std::option::Option<crate::model::Platform>) -> Self {
            self.inner = self.inner.set_platform(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ExportProject<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::export_project_input::Builder,
    }
    impl<C> ExportProject<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ExportProjectOutput,
            smithy_http::result::SdkError<crate::error::ExportProjectError>,
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
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(input);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListBundles<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_bundles_input::Builder,
    }
    impl<C> ListBundles<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListBundlesOutput,
            smithy_http::result::SdkError<crate::error::ListBundlesError>,
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
        /// <p>
        /// Maximum number of records to list in a single response.
        /// </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>
        /// Pagination token. Set to null to start listing bundles from start.
        /// If non-null pagination token is returned in a result, then pass its
        /// value in here in another request to list more bundles.
        /// </p>
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
    pub struct ListProjects<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_projects_input::Builder,
    }
    impl<C> ListProjects<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListProjectsOutput,
            smithy_http::result::SdkError<crate::error::ListProjectsError>,
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
        /// <p>
        /// Maximum number of records to list in a single response.
        /// </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>
        /// Pagination token. Set to null to start listing projects from start.
        /// If non-null pagination token is returned in a result, then pass its
        /// value in here in another request to list more projects.
        /// </p>
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
    pub struct UpdateProject<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::update_project_input::Builder,
    }
    impl<C> UpdateProject<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateProjectOutput,
            smithy_http::result::SdkError<crate::error::UpdateProjectError>,
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
        /// <p>
        /// ZIP or YAML file which contains project configuration to be updated. This should
        /// be the contents of the file downloaded from the URL provided in an export project
        /// operation.
        /// </p>
        pub fn contents(mut self, input: smithy_types::Blob) -> Self {
            self.inner = self.inner.contents(input);
            self
        }
        pub fn set_contents(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_contents(input);
            self
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(input);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
}
