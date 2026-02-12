use clap::Subcommand;
use crate::application::config::AuthUseCase;

#[derive(Subcommand)]
#[derive(Debug)]
pub enum PasswordCommands {
    Set {
        password: String
    },
    Clear
}

impl PasswordCommands {

    pub async fn run(&self, auth_use_case: Box<dyn  AuthUseCase>) {
        match self {
            PasswordCommands::Set { password } => {
                println!("Password set");
                auth_use_case.set_password(Some(password.clone())).await;
            },
            PasswordCommands::Clear => {
                auth_use_case.set_password(None).await;
                println!("Password removed");
            }
        }
    }
}
