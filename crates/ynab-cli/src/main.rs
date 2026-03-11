mod cli;
mod commands;
mod mcp;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use ynab_client::{YnabClient, auth};

use cli::{Cli, Command};
use output::OutputConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        // Auth doesn't need a client
        Command::Auth { command } => {
            commands::auth::run(command)?;
        }

        // Schema doesn't need a client
        Command::Schema { resource_method } => {
            commands::schema::run(resource_method)?;
        }

        // Shell completions
        Command::Completions { shell } => {
            clap_complete::generate(*shell, &mut Cli::command(), "ynab", &mut std::io::stdout());
        }

        // MCP server - resolves its own auth
        Command::Mcp => {
            let token = auth::resolve_token(cli.token.as_deref())?;
            let client = YnabClient::new(token)?;
            let server = mcp::YnabMcpServer::new(client);
            server.serve_stdio().await?;
        }

        // All other commands need an authenticated client
        _ => {
            let token = auth::resolve_token(cli.token.as_deref())?;
            let client = YnabClient::new(token)?;
            let plan_id = cli.plan_id.as_deref();
            let dry_run = cli.dry_run;

            let out = OutputConfig {
                format: &cli.output_format,
                dollars: cli.dollars,
                fields: cli.fields.as_deref(),
                output_path: cli.output.as_deref(),
            };

            match &cli.command {
                Command::User { command } => {
                    commands::user::run(&client, command, &out, dry_run).await?;
                }
                Command::Plans { command } => {
                    commands::plans::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::Accounts { command } => {
                    commands::accounts::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::Transactions { command } => {
                    commands::transactions::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::Categories { command } => {
                    commands::categories::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::Payees { command } => {
                    commands::payees::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::PayeeLocations { command } => {
                    commands::payee_locations::run(&client, command, &out, plan_id, dry_run)
                        .await?;
                }
                Command::Months { command } => {
                    commands::months::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::Scheduled { command } => {
                    commands::scheduled::run(&client, command, &out, plan_id, dry_run).await?;
                }
                Command::MoneyMovements { command } => {
                    commands::money_movements::run(&client, command, &out, plan_id, dry_run)
                        .await?;
                }
                Command::Api { method, path, body } => {
                    commands::api::run(&client, method, path, body.as_deref(), &out, dry_run)
                        .await?;
                }
                // Already handled above
                Command::Auth { .. }
                | Command::Schema { .. }
                | Command::Mcp
                | Command::Completions { .. } => unreachable!(),
            }
        }
    }

    Ok(())
}
