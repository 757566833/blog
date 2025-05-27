use std::sync::OnceLock;
use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider, Resource};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

fn get_resource(server_name: &str) -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();
    let server_name_owned = server_name.to_string();
    RESOURCE
        .get_or_init(|| {
            Resource::builder()
                .with_service_name(server_name_owned.clone())
                .build()
        })
        .clone()
}

fn init_logs(server_url:&str,server_name: &str) -> SdkLoggerProvider {
    let exporter = LogExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/logs",server_url))
        .with_protocol(Protocol::HttpBinary)
        .build()
        .expect("Failed to create log exporter");

    SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource(server_name))
        .build()
}

fn init_traces(server_url:&str,server_name: &str) -> SdkTracerProvider {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/traces",server_url))
        .with_protocol(Protocol::HttpBinary) //can be changed to `Protocol::HttpJson` to export in JSON format
        .build()
        .expect("Failed to create trace exporter");

    SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource(server_name))
        .build()
}
fn init_metrics(server_url:&str,server_name: &str) -> SdkMeterProvider {
    let exporter = MetricExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/metrics",server_url))
        .with_protocol(Protocol::HttpBinary) //can be changed to `Protocol::HttpJson` to export in JSON format
        .build()
        .expect("Failed to create metric exporter");

    SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(get_resource(server_name))
        .build()
}

pub fn init_opentelemetry(server_url:&str,server_name:&str)->(SdkLoggerProvider,SdkTracerProvider,SdkMeterProvider) {
    let logger_provider = init_logs(server_url,server_name);

    // Create a new OpenTelemetryTracingBridge using the above LoggerProvider.
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // For the OpenTelemetry layer, add a tracing filter to filter events from
    // OpenTelemetry and its dependent crates (opentelemetry-otlp uses crates
    // like reqwest/tonic etc.) from being sent back to OTel itself, thus
    // preventing infinite telemetry generation. The filter levels are set as
    // follows:
    // - Allow `info` level and above by default.
    // - Restrict `opentelemetry`, `hyper`, `tonic`, and `reqwest` completely.
    // Note: This will also drop events from crates like `tonic` etc. even when
    // they are used outside the OTLP Exporter. For more details, see:
    // https://github.com/open-telemetry/opentelemetry-rust/issues/761
    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("opentelemetry=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = otel_layer.with_filter(filter_otel);
  // 只在 debug 模式下启用 fmt 层（终端打印）
    #[cfg(debug_assertions)]{
    // Create a new tracing::Fmt layer to print the logs to stdout. It has a
    // default filter of `info` level and above, and `info` and above for logs
    // from OpenTelemetry crates. The filter levels can be customized as needed.
    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=info".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    // Initialize the tracing subscriber with the OpenTelemetry layer and the
    // Fmt layer.
    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();
    }
    #[cfg(not(debug_assertions))]
    {
        // release 构建，不加 fmt_layer
        tracing_subscriber::registry()
            .with(otel_layer)
            .init();
    }

    // At this point Logs (OTel Logs and Fmt Logs) are initialized, which will
    // allow internal-logs from Tracing/Metrics initializer to be captured.

    let tracer_provider = init_traces(server_url,server_name);
    // Set the global tracer provider using a clone of the tracer_provider.
    // Setting global tracer provider is required if other parts of the application
    // uses global::tracer() or global::tracer_with_version() to get a tracer.
    // Cloning simply creates a new reference to the same tracer provider. It is
    // important to hold on to the tracer_provider here, so as to invoke
    // shutdown on it when application ends.
    global::set_tracer_provider(tracer_provider.clone());

    let meter_provider = init_metrics(server_url,server_name);
    // Set the global meter provider using a clone of the meter_provider.
    // Setting global meter provider is required if other parts of the application
    // uses global::meter() or global::meter_with_version() to get a meter.
    // Cloning simply creates a new reference to the same meter provider. It is
    // important to hold on to the meter_provider here, so as to invoke
    // shutdown on it when application ends.
    global::set_meter_provider(meter_provider.clone());

    return (logger_provider, tracer_provider, meter_provider);

    // let common_scope_attributes = vec![KeyValue::new("scope-key", "scope-value")];
    // let scope = InstrumentationScope::builder("gateway")
    //     .with_version("0.0.1")
    //     .with_attributes(common_scope_attributes)
    //     .build();

    // let tracer = global::tracer_with_scope(scope.clone());
    // let meter = global::meter_with_scope(scope);

    // let counter = meter
    //     .u64_counter("test_counter")
    //     .with_description("a simple counter for demo purposes.")
    //     .with_unit("my_unit")
    //     .build();
    
}
