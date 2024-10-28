use color_eyre::Result;
use doas_rs::cli::cmd;
use human_panic::setup_panic;
use pam_client2::conv_cli::Conversation;
use pam_client2::{Context, Flag};
use std::{os::unix::process::CommandExt, process::Command};
use users::get_user_by_name;

fn main() -> Result<()> {
    setup_panic!(); // <-- initialize human-panic
    let matches = cmd().get_matches();

    match matches.subcommand_name() {
        Some("doas") => {
            let doas_matches = matches.subcommand_matches("doas").unwrap();
            let commands = doas_matches
                .get_many::<String>("command")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            let default_user = format!("root");
            let user = doas_matches
                .get_one::<String>("user")
                .unwrap_or(&default_user);
            let uid = get_user_by_name(user).uid();

            // Authenticate session.
            let mut context = Context::new(
                "doas",              // Service name, decides which policy is used
                None,                // Optional preset user name
                Conversation::new(), // Handler for user interaction
            )
            .expect("Failed to initialize PAM context");

            context.set_user_prompt(Some("Password:"));
            context
                .authenticate(Flag::NONE)
                .expect("Authentication failed");
            context
                .acct_mgmt(Flag::NONE)
                .expect("Authentication failed");
            let mut session = context
                .open_session(Flag::NONE)
                .expect("Session opening failed");

            Command::new(commands[0])
                .args(&commands[1..])
                .env_clear()
                .envs(session.envlist().iter_tuples())
                .uid(uid)
                .exec();
        }
        Some("doasedit") => {
            todo!("doasedit");
        }
        Some("vidoas") => {
            todo!("vidoas");
        }
        _ => {
            cmd().print_help()?;
        }
    }
    Ok(())
}
