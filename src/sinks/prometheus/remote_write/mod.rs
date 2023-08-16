use snafu::prelude::*;

use crate::sinks::util::buffer::metrics::MetricNormalize;

mod config;
mod request_builder;
mod service;
mod sink;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "prometheus-integration-tests"))]
mod integration_tests;

pub use config::RemoteWriteConfig;

/// Supported compression types for Prometheus Remote Write.
#[configurable_component]
#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
#[serde(rename_all = "lowercase")]
pub(super) enum Compression {
    /// Snappy.
    #[derivative(Default)]
    Snappy,

    /// Gzip.
    Gzip,

    /// Zstandard.
    Zstd,
}

#[derive(Debug, Snafu)]
enum Errors {
    #[snafu(display(r#"Prometheus remote_write sink cannot accept "set" metrics"#))]
    SetMetricInvalid,
    #[snafu(display("aws.region required when AWS authentication is in use"))]
    AwsRegionRequired,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct PartitionKey {
    tenant_id: Option<String>,
}

#[derive(Default)]
pub struct PrometheusMetricNormalize;

impl MetricNormalize for PrometheusMetricNormalize {
    fn normalize(&mut self, state: &mut MetricSet, metric: Metric) -> Option<Metric> {
        state.make_absolute(metric)
    }
}
