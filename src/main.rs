use awscloud_sso_cred_helper::AwsSsoWorkflow;
use clap::Parser;
use std::error::Error;

/// `Args` represents the structure to parse command-line arguments for this CLI.
///
/// # Fields
///
/// * `start_url` - An optional field specifying the start URL for the AWS SSO session.
/// * `region` - An optional field specifying the AWS region for the session.
/// * `command` - An optional subcommand that determines the action to perform.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    start_url: Option<String>,

    #[arg(long)]
    region: Option<String>,

    #[clap(subcommand)]
    command: Option<SubCommands>,
}

/// SubCommands defines the two available commands for the CLI:
///
/// - `Refresh`: A command to refresh the AWS SSO credentials. This command takes no additional arguments.
/// - `Start`: A command to start an AWS SSO session.
///    - `start_url` (String): The start URL for the AWS SSO session.
///    - `region` (String): The AWS region for the session.
#[derive(Parser, Debug)]
enum SubCommands {
    Refresh,
    Start {
        #[arg(
            short,
            long,
            help = "the start url of aws sso: usually something <domain>.awsapps.com/start"
        )]
        start_url: String,

        #[arg(short, long)]
        region: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut workflow = AwsSsoWorkflow {
        start_url: format!(
            "{}{}",
            args.start_url.unwrap_or_else(|| "".to_string()),
            ".awsapps.com/start"
        ),
        region: args.region.unwrap_or_else(|| "".to_string()),
    };

    let _ = workflow.run_workflow().await?;

    Ok(())
}
