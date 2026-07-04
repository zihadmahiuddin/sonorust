use js_sys::Function;
use once_cell::sync::OnceCell;
use tracing::{Event, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

// 1. A global storage for the callback (contained inside WASM module)
static LOG_CALLBACK: OnceCell<Function> = OnceCell::new();

pub fn set_log_callback(callback: Function) {
    let _ = LOG_CALLBACK.set(callback);
}

// 2. The Custom Layer
pub struct JsTracingLayer;

impl<S: Subscriber> Layer<S> for JsTracingLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if let Some(callback) = LOG_CALLBACK.get() {
            // Extract the log message
            let mut visitor = LogVisitor::default();
            event.record(&mut visitor);

            // Send to JS
            let level = event.metadata().level().to_string();
            let _ = callback.call2(
                &wasm_bindgen::JsValue::NULL,
                &level.into(),
                &visitor.message.into(),
            );
        }
    }
}

// 3. Helper to extract the message field from Tracing events
#[derive(Default)]
struct LogVisitor {
    message: String,
}

impl tracing::field::Visit for LogVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}
