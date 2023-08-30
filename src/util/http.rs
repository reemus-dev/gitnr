use anyhow::Context;
use once_cell::sync::Lazy;
use std::sync::Arc;
use ureq::{Agent, AgentBuilder};

static AGENT: Lazy<Agent> = Lazy::new(|| {
    AgentBuilder::new()
        .tls_connector(Arc::new(
            native_tls::TlsConnector::new()
                .with_context(|| "Failed to get native TLS implementation for HTTP requests")
                .unwrap(),
        ))
        .build()
});

pub fn http() -> Agent {
    AGENT.clone()
}
