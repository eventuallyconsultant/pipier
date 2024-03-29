use axum::{extract::State, response::Html, routing::post, Router};
use http::Uri;
use reqwest::Client;
use serde_json::Value;

mod error;
mod jq;
mod parsing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  pretty_env_logger::try_init().ok();

  let client = Client::new();

  let app = Router::new().route("/*args", post(handler)).with_state(client);

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
  println!("listening on {}", listener.local_addr()?);
  axum::serve(listener, app).await?;

  Ok(())
}

async fn handler(State(client): State<Client>, uri: Uri, body: String) -> Result<Html<&'static str>, error::HttpError> {
  tracing::trace!("uri: {uri:?}");
  tracing::trace!("body: {body}");
  let mut json: Value = serde_json::from_str(&body)?;
  let commands = parsing::parse_args(&uri)?;

  for command in commands {
    match command {
      parsing::Command::Jq(jq_query) => {
        tracing::info!("Running jq query: {jq_query} on {json}.");
        json = jq::jq(json, &jq_query)?;
        tracing::info!("Result is {json}.");
      }
      parsing::Command::Target(target) => {
        tracing::info!("Sending to {json} to {target}.");
        client.post(target).json(&json).send().await?;
      }
    }
  }

  // todo : return the reqwest response when there is one
  Ok(Html("Ok"))
}
