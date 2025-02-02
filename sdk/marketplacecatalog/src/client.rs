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
    pub fn cancel_change_set(&self) -> fluent_builders::CancelChangeSet<C> {
        fluent_builders::CancelChangeSet::new(self.handle.clone())
    }
    pub fn describe_change_set(&self) -> fluent_builders::DescribeChangeSet<C> {
        fluent_builders::DescribeChangeSet::new(self.handle.clone())
    }
    pub fn describe_entity(&self) -> fluent_builders::DescribeEntity<C> {
        fluent_builders::DescribeEntity::new(self.handle.clone())
    }
    pub fn list_change_sets(&self) -> fluent_builders::ListChangeSets<C> {
        fluent_builders::ListChangeSets::new(self.handle.clone())
    }
    pub fn list_entities(&self) -> fluent_builders::ListEntities<C> {
        fluent_builders::ListEntities::new(self.handle.clone())
    }
    pub fn start_change_set(&self) -> fluent_builders::StartChangeSet<C> {
        fluent_builders::StartChangeSet::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CancelChangeSet<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::cancel_change_set_input::Builder,
    }
    impl<C> CancelChangeSet<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CancelChangeSetOutput,
            smithy_http::result::SdkError<crate::error::CancelChangeSetError>,
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
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>.</p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique identifier of the <code>StartChangeSet</code> request that you
        /// want to cancel.</p>
        pub fn change_set_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_id(input);
            self
        }
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeChangeSet<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_change_set_input::Builder,
    }
    impl<C> DescribeChangeSet<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeChangeSetOutput,
            smithy_http::result::SdkError<crate::error::DescribeChangeSetError>,
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
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique identifier for the <code>StartChangeSet</code> request that you
        /// want to describe the details for.</p>
        pub fn change_set_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_id(input);
            self
        }
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeEntity<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_entity_input::Builder,
    }
    impl<C> DescribeEntity<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeEntityOutput,
            smithy_http::result::SdkError<crate::error::DescribeEntityError>,
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
        /// <p>Required. The catalog related to the request. Fixed value:
        /// <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Required. The unique ID of the entity to describe.</p>
        pub fn entity_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.entity_id(input);
            self
        }
        pub fn set_entity_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_entity_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListChangeSets<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_change_sets_input::Builder,
    }
    impl<C> ListChangeSets<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListChangeSetsOutput,
            smithy_http::result::SdkError<crate::error::ListChangeSetsError>,
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
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>An array of filter objects.</p>
        pub fn filter_list(mut self, inp: impl Into<crate::model::Filter>) -> Self {
            self.inner = self.inner.filter_list(inp);
            self
        }
        pub fn set_filter_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filter_list(input);
            self
        }
        /// <p>An object that contains two attributes, <code>SortBy</code> and
        /// <code>SortOrder</code>.</p>
        pub fn sort(mut self, input: crate::model::Sort) -> Self {
            self.inner = self.inner.sort(input);
            self
        }
        pub fn set_sort(mut self, input: std::option::Option<crate::model::Sort>) -> Self {
            self.inner = self.inner.set_sort(input);
            self
        }
        /// <p>The maximum number of results returned by a single call. This value must be provided
        /// in the next call to retrieve the next set of results. By default, this value is
        /// 20.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token value retrieved from a previous call to access the next page of
        /// results.</p>
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
    pub struct ListEntities<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_entities_input::Builder,
    }
    impl<C> ListEntities<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListEntitiesOutput,
            smithy_http::result::SdkError<crate::error::ListEntitiesError>,
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
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>The type of entities to retrieve.</p>
        pub fn entity_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.entity_type(input);
            self
        }
        pub fn set_entity_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_entity_type(input);
            self
        }
        /// <p>An array of filter objects. Each filter object contains two attributes,
        /// <code>filterName</code> and <code>filterValues</code>.</p>
        pub fn filter_list(mut self, inp: impl Into<crate::model::Filter>) -> Self {
            self.inner = self.inner.filter_list(inp);
            self
        }
        pub fn set_filter_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filter_list(input);
            self
        }
        /// <p>An object that contains two attributes, <code>SortBy</code> and
        /// <code>SortOrder</code>.</p>
        pub fn sort(mut self, input: crate::model::Sort) -> Self {
            self.inner = self.inner.sort(input);
            self
        }
        pub fn set_sort(mut self, input: std::option::Option<crate::model::Sort>) -> Self {
            self.inner = self.inner.set_sort(input);
            self
        }
        /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>Specifies the upper limit of the elements on a single page. If a value isn't provided,
        /// the default value is 20.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartChangeSet<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::start_change_set_input::Builder,
    }
    impl<C> StartChangeSet<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartChangeSetOutput,
            smithy_http::result::SdkError<crate::error::StartChangeSetError>,
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
        /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code>
        /// </p>
        pub fn catalog(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.catalog(input);
            self
        }
        pub fn set_catalog(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_catalog(input);
            self
        }
        /// <p>Array of <code>change</code> object.</p>
        pub fn change_set(mut self, inp: impl Into<crate::model::Change>) -> Self {
            self.inner = self.inner.change_set(inp);
            self
        }
        pub fn set_change_set(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Change>>,
        ) -> Self {
            self.inner = self.inner.set_change_set(input);
            self
        }
        /// <p>Optional case sensitive string of up to 100 ASCII characters. The change set name can
        /// be used to filter the list of change sets. </p>
        pub fn change_set_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.change_set_name(input);
            self
        }
        pub fn set_change_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_change_set_name(input);
            self
        }
        /// <p>A unique token to identify the request to ensure idempotency.</p>
        pub fn client_request_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_request_token(input);
            self
        }
        pub fn set_client_request_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_request_token(input);
            self
        }
    }
}
