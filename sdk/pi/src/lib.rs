#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <fullname>Amazon RDS Performance Insights</fullname>
//! <p>Amazon RDS Performance Insights enables you to monitor and explore different dimensions of
//! database load based on data captured from a running DB instance. The guide provides detailed
//! information about Performance Insights data types, parameters and errors.</p>
//! <p>When Performance Insights is enabled, the Amazon RDS Performance Insights API provides visibility into the performance of your DB instance. Amazon
//! CloudWatch provides the authoritative source for AWS service-vended monitoring metrics.  Performance Insights offers a domain-specific
//! view of DB load. </p>
//! <p>DB load is measured as Average Active Sessions. Performance Insights provides the data to API consumers as a two-dimensional
//! time-series dataset. The time dimension provides DB load data for each time point in the queried time range. Each time
//! point decomposes overall load in relation to the requested dimensions, measured at that time point. Examples include
//! SQL, Wait event, User, and Host.</p>
//! <ul>
//! <li>
//! <p>To learn more about Performance Insights and Amazon Aurora DB instances, go to the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_PerfInsights.html">Amazon Aurora User Guide</a>.</p>
//! </li>
//! <li>
//! <p>To learn more about Performance Insights and Amazon RDS DB instances, go to the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Amazon RDS User Guide</a>.</p>
//! </li>
//! </ul>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("pi", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
