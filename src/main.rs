use awscloud_sso_cred_helper::AwsSsoWorkflow;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    start_url: Option<String>,

    #[arg(long)]
    region: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut workflow = AwsSsoWorkflow {
        start_url: args.start_url.unwrap_or_else(|| "".to_string()),
        region: args.region.unwrap_or_else(|| "".to_string()),
    };

    let _ = workflow.run_workflow().await?;

    Ok(())
}
