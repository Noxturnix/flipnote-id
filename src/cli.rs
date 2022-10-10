use clap::{ArgAction, Parser, ValueEnum, ValueHint};

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(value_enum)]
    pub action: Actions,

    /// Flipnote Studio option file
    #[arg(default_value = "option.bin")]
    #[arg(long = "file")]
    #[arg(short = 'f')]
    #[arg(value_hint = ValueHint::FilePath)]
    pub file: String,

    /// Flipnote Studio ID (use with `set` action)
    #[arg(long = "id")]
    #[arg(short = 'i')]
    #[arg(value_hint = ValueHint::FilePath)]
    #[arg(required_if_eq("action", "set"))]
    pub fsid: Option<String>,

    /// Don't backup the original file when setting FSID (used by `set` action)
    #[arg(action = ArgAction::SetTrue)]
    #[arg(long = "no-backup")]
    #[arg(short = 'd')]
    pub no_backup: bool,
}

#[derive(ValueEnum, Clone)]
pub enum Actions {
    /// Set FSID and compute checksum
    Set,
    /// Extract FSID
    Extract,
    /// Check FSID and verify checksum
    Check,
}
