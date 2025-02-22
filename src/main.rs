use clap::Parser;
use awscloud_sso_cred_helper::AwsSsoWorkflow;
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
    
    // println!("Account ID: {}", credential.account_id);
    // println!("Role Name: {}", credential.role_name);
    // println!("Access Key ID: {}", credential.access_key_id);
    // println!("Secret Access Key: {}", credential.secret_access_key);
    // println!("Session Token: {}", credential.session_token);

    Ok(())
}
