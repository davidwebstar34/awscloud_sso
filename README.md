# AWS(Unofficial) Cloud SSO 

## Installation

### Using `curl` (Recommended)

To install the latest binary release from GitHub:

```sh
curl -L -o awscloud_sso https://github.com/davidwebstar34/awscloud_sso/releases/latest/download/awscloud_sso-x86_64-apple-darwin
chmod +x awscloud_sso
sudo mv awscloud_sso /usr/local/bin/
```

### macOS: Allow Running the Binary

On macOS, you may need to explicitly allow the binary:

```sh
xattr -d com.apple.quarantine /usr/local/bin/awscloud_sso
```

## Usage

Once installed, run the following command to start the interactive AWS SSO workflow:

```sh
awscloud_sso
```

### Example Commands

- **Authenticate and Select a Role**

  ```sh
  awscloud_sso
  ```

  This command will prompt you to log in and select an AWS account and role.

- **Specify a Start URL and Region**

  ```sh
  awscloud_sso --start-url https://your.awsapps.com/start --region us-west-2
  ```

  This allows you to bypass the interactive prompt by providing the SSO start URL and AWS region.

- **Display Help**
  ```sh
  awscloud_sso --help
  ```
  Shows available command-line options.

## How It Works

1. The tool initiates an AWS SSO authentication flow.
2. The user logs in via a browser to authorize the CLI session.
3. Available AWS accounts and roles are retrieved.
4. The user selects an account and role.
5. Temporary AWS credentials are fetched and stored in `~/.aws/credentials`.
6. The credentials can be used with AWS CLI or SDKs.

## `awscloud_sso_credential_helper` Library

This tool is built on top of the `awscloud_sso_credential_helper` library, which provides the core logic for AWS SSO authentication and credential retrieval. Developers can integrate this library into their Rust projects to automate AWS SSO credential handling.

To use it in a Rust project, add the dependency to your `Cargo.toml`:

```toml
[dependencies]
awscloud_sso_credential_helper = "1.2.0"
```

Example usage in Rust:

```rust
use awscloud_sso_credential_helper::AwsSsoWorkflow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workflow = AwsSsoWorkflow::default();
    let credential = workflow.run_workflow().await?;

    println!("Account ID: {}", credential.account_id);
    println!("Role Name: {}", credential.role_name);
    println!("Access Key ID: {}", credential.access_key_id);
    println!("Secret Access Key: {}", credential.secret_access_key);
    println!("Session Token: {}", credential.session_token);
    Ok(())
}
```

## Integration with AWS CLI

After authenticating with `awscloud_sso`, you can run AWS CLI commands seamlessly:

```sh
aws s3 ls
```

## Troubleshooting

- **Expired credentials**: Run `awscloud_sso` again to refresh credentials.
- **Browser login issues**: Manually visit the login URL printed in the terminal.
- **macOS Security Warning**: If you get a warning when running the binary, allow it using:
  ```sh
  xattr -d com.apple.quarantine /usr/local/bin/awscloud_sso
  ```

---

🚀 **Start using `awscloud_sso` today to simplify AWS authentication!**
