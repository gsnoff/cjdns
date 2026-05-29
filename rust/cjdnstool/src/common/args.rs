use cjdns::admin::Opts;
use clap::Args;
use const_format::formatcp;
use env_logger::Env;
use log::{LevelFilter, SetLoggerError};

use super::ansi::{BOLD, BOLD_UNDERLINE, CLEAR, UNDERLINE};

const ENV_LOG_FILTER: &str = "CJDNS_LOG";
const ENV_LOG_FILTER_ALIGN: &str = "      ";
const ENV_LOG_STYLE: &str = "CJDNS_LOG_STYLE";

// Macros from `const_format` currently don't support alignment.
const AFTER_HELP: &str = formatcp!(
    "\
{BOLD_UNDERLINE}Environment Variables:{CLEAR}
  {BOLD}{ENV_LOG_FILTER}{CLEAR}{ENV_LOG_FILTER_ALIGN}  \
  Log filters to apply, in env_logger syntax
  {BOLD}{ENV_LOG_STYLE}{CLEAR}  \
  Write style to apply for logging"
);

const AFTER_LONG_HELP: &str = formatcp!(
    r#"{AFTER_HELP}

  For detailed log filter and style syntax, refer to:
  {UNDERLINE}https://docs.rs/env_logger/0.11/env_logger/#enabling-logging{CLEAR}

  Note: If the current directory or any of its parents contain a file named ".env",
  the NAME=value entries inside will be automatically parsed and applied
  if the corresponding variables are not already present in the environment."#
);

#[derive(Args)]
#[command(after_help = AFTER_HELP, after_long_help = AFTER_LONG_HELP)]
pub struct CommonArgs {
    /// Increase logging verbosity (-v for info, -vv for debug, -vvv for trace)
    ///
    /// Logging verbosity can also be controlled via the CJDNS_LOG environment variable (see below).
    ///
    /// Note: The command line argument takes precedence over the environment variable.
    #[arg(short = 'v', long, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    verbose: u8,

    /// Decrease logging verbosity (-q for error, -qq to silence default level logging)
    #[arg(short = 'q', long, action = clap::ArgAction::Count)]
    quiet: u8,

    /// Remote IP address (either IPv4 or IPv6).
    #[arg(short = 'a', long, value_name = "IP")]
    address: Option<String>,

    /// Remote UDP port.
    #[arg(short = 'p', long, value_name = "PORT")]
    port: Option<u16>,

    /// Connection password for cjdns instance.
    #[arg(short = 'P', long, value_name = "PASSWORD")]
    password: Option<String>,

    /// Path to config file (~/.cjdnsadmin used by default).
    #[arg(short = 'c', long, value_name = "PATH")]
    cjdnsadmin: Option<String>,
}

impl CommonArgs {
    pub fn init_logger(&self) -> Result<(), SetLoggerError> {
        let mut builder = env_logger::Builder::from_env(
            Env::new()
                .filter_or(ENV_LOG_FILTER, "warn")
                .write_style(ENV_LOG_STYLE),
        );
        if self.verbose > 0 {
            use LevelFilter::*;
            builder.filter_level(match self.verbose {
                1 => Info,
                2 => Debug,
                _ => Trace, // 3 or higher
            });
        } else if self.quiet > 0 {
            use LevelFilter::*;
            builder.filter_level(match self.quiet {
                1 => Error,
                _ => Off, // 2 or higher
            });
        }
        builder.try_init()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn as_anon(self) -> Opts {
        Opts {
            addr: self.address,
            port: self.port,
            password: None,
            config_file_path: self.cjdnsadmin,
            anon: true,
        }
    }

    pub fn with_auth(self) -> Opts {
        Opts {
            addr: self.address,
            port: self.port,
            password: Some(self.password.unwrap_or_else(|| "NONE".to_owned())),
            config_file_path: self.cjdnsadmin,
            anon: false,
        }
    }
}
