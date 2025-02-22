awscloud_sso

awscloud_sso is a command-line tool that provides seamless authentication for AWS accounts using AWS SSO. It builds on top of the awscloud_sso_credential_helper library and simplifies retrieving and managing AWS credentials for multiple accounts and roles.

Features

Interactive selection of AWS SSO accounts and roles

Automatic fetching and storing of temporary credentials

Integration with AWS CLI and SDKs

Multi-platform support: Linux, macOS, Windows(Not supported)

Installation

You can install awscloud_sso using cargo:

cargo install awscloud_sso

Alternatively, download the latest release from GitHub Releases.

Usage

Once installed, run the following command to start the interactive AWS SSO workflow:

awscloud_sso

Example Commands

Authenticate and Select a Role

awscloud_sso

This command will prompt you to log in and select an AWS account and role.

Specify a Start URL and Region

awscloud_sso --start-url https://your.awsapps.com/start --region us-west-2

This allows you to bypass the interactive prompt by providing the SSO start URL and AWS region.

Display Help

awscloud_sso --help

Shows available command-line options.

How It Works

The tool initiates an AWS SSO authentication flow.

The user logs in via a browser to authorize the CLI session.

Available AWS accounts and roles are retrieved.

The user selects an account and role.

Temporary AWS credentials are fetched and stored in ~/.aws/credentials.

The credentials can be used with AWS CLI or SDKs.

Configuration

The tool reads existing AWS configurations and supports additional environment variables:

export AWS_SSO_START_URL="https://your.awsapps.com/start"
export AWS_REGION="us-west-2"

Integration with AWS CLI

After authenticating with awscloud_sso, you can run AWS CLI commands seamlessly:

aws s3 ls

Troubleshooting

Expired credentials: Run awscloud_sso again to refresh credentials.

Missing dependencies: Ensure Rust is installed (rustc --version).

Browser login issues: Manually visit the login URL printed in the terminal.

License

awscloud_sso is licensed under the MIT

🚀 Start using awscloud_sso today to simplify and secure AWS authentication!
