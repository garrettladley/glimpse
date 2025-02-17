use crate::config::Config;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "glimpse",
    about = "A blazingly fast tool for peeking at codebases",
    version
)]
pub struct Cli {
    /// Directory to analyze
    #[arg(value_parser = validate_path, default_value = ".")]
    pub path: PathBuf,

    /// Additional patterns to include (e.g. "*.rs,*.go")
    #[arg(short, long, value_delimiter = ',')]
    pub include: Option<Vec<String>>,

    /// Additional patterns to exclude
    #[arg(short, long, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,

    /// Maximum file size in bytes
    #[arg(short, long)]
    pub max_size: Option<u64>,

    /// Maximum directory depth
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// Output format (tree, files, or both)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output file path (optional)
    #[arg(short = 'f', long)]
    pub file: Option<PathBuf>,

    /// Print to stdout instead
    #[arg(short, long)]
    pub print: bool,

    /// Number of threads for parallel processing
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// Show hidden files and directories
    #[arg(short = 'H', long)]
    pub hidden: bool,

    /// Don't respect .gitignore files
    #[arg(long)]
    pub no_ignore: bool,

    /// Token Count
    #[arg(long)]
    pub tokens: bool,
}

impl Cli {
    pub fn parse_with_config(config: &Config) -> anyhow::Result<Self> {
        let mut cli = Self::parse();

        // Apply config defaults if CLI args aren't specified
        cli.max_size = cli.max_size.or(Some(config.max_size));
        cli.max_depth = cli.max_depth.or(Some(config.max_depth));
        cli.output = cli.output.or(Some(config.default_output_format.clone()));

        // Merge excludes from config and CLI
        if let Some(mut excludes) = cli.exclude.take() {
            excludes.extend(config.default_excludes.clone());
            cli.exclude = Some(excludes);
        } else {
            cli.exclude = Some(config.default_excludes.clone());
        }

        Ok(cli)
    }
}

fn validate_path(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("Path '{}' does not exist", path));
    }
    if !path_buf.is_dir() {
        return Err(format!("Path '{}' is not a directory", path));
    }
    Ok(path_buf)
}
