use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::Resource;
use opentelemetry_sdk::trace::{Sampler, SdkTracerProvider, Tracer};
use opentelemetry::trace::TracerProvider;
use std::sync::OnceLock;
use std::time::Duration;

pub struct TempoTracerGuard {
    provider: SdkTracerProvider,
}

static SDK_TRACER: OnceLock<Tracer> = OnceLock::new();

pub fn get_sdk_tracer() -> Option<Tracer> {
    SDK_TRACER.get().cloned()
}

impl TempoTracerGuard {
    pub fn shutdown(self) {
        let _ = self.provider.shutdown();
    }
}
pub fn init_otel_tracing(service_name: &str, otlp_endpoint: &str) -> TempoTracerGuard {
    println!("Initialise TEMPO: {}", otlp_endpoint);
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Prepare resource with service.name
    let resource = Resource::builder_empty()
        .with_attributes([KeyValue::new("service.name", service_name.to_string())])
        .build();

    // Create exporter
    let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_protocol(Protocol::HttpBinary)
            .with_endpoint(otlp_endpoint)
            .with_timeout(Duration::from_secs(5))
            .build()
            .expect("build otlp http exporter");
    
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(1.0))))
        .build();

    let tracer = provider.tracer(service_name.to_string());
    let _ = SDK_TRACER.set(tracer);

    global::set_tracer_provider(provider.clone());

    TempoTracerGuard { provider }
}