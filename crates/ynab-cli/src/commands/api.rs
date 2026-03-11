use anyhow::Result;
use ynab_client::YnabClient;

use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    method: &str,
    path: &str,
    body: Option<&str>,
    out: &OutputConfig<'_>,
    dry_run: bool,
) -> Result<()> {
    let body_value = match body {
        Some(b) => Some(serde_json::from_str::<serde_json::Value>(b)?),
        None => None,
    };

    if dry_run {
        output::output(
            &client.dry_run_request(method, path, body_value.as_ref()),
            out,
        )?;
        return Ok(());
    }

    let result = client
        .raw_request(method, path, body_value.as_ref())
        .await?;
    output::output(&result, out)?;
    Ok(())
}
