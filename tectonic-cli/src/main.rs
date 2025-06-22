#![allow(clippy::needless_return)]
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use tectonic::{generate_workload, generate_workload_spec_schema};
use tracing::debug;
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate workload(s) from a file or folder of workload specifications.
    Generate {
        /// File or folder of workload spec files
        #[arg(short = 'w', long = "workload")]
        workload_path: String,

        /// Output folder for workloads. Defaults to the same directory as the workload spec file.
        #[arg(short = 'o', long = "output", required = false)]
        output: Option<String>,

        /// Do not write any files, just log what would be done.
        #[arg(long = "dry-run", required = false)]
        dry_run: bool,
    },
    /// Prints the JSON schema for IDE integration.
    Schema,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::new("debug"))
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    debug!("hi");

    match args.command {
        Command::Generate {
            dry_run: _,
            workload_path,
            output,
        } => invoke_generate(&workload_path, output.as_deref()),
        Command::Schema => invoke_schema(),
    }
}

/// Generate workload(s) from a file or folder of workload specifications.
fn invoke_generate(workload_path: &str, output: Option<&str>) -> Result<()> {
    let workload_path = PathBuf::from(workload_path);
    if !workload_path.exists() {
        bail!("File or folder does not exist {}", workload_path.display());
    }

    let output_path = if let Some(output) = output {
        // Directory that didn't exist.
        let output_path = PathBuf::from(output);
        if !output_path.exists() {
            fs::create_dir_all(&output_path)?;
        }
        output_path
    } else if workload_path.is_dir() {
        // Same directory as workload spec dir.
        workload_path.clone()
    } else {
        // Directory containing spec file.
        workload_path.parent().unwrap().to_path_buf()
    };

    if workload_path.is_dir() {
        for entry in WalkDir::new(&workload_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|file| {
                file.file_type().is_file()
                    && file
                        .path()
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.ends_with(".spec.json"))
                        .unwrap_or(false)
            })
        {
            let path = entry.path();
            println!("Generating workload for: {}", path.display());
            let contents = fs::read_to_string(path)?;

            let output_file = path
                .file_name()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.rsplitn(3, '.').collect::<Vec<_>>()[2]) // file.spec.json -> file
                .map(|stem| format!("{stem}.txt")) // file -> file.txt
                .unwrap_or_else(|| {
                    let filename = path.file_name().unwrap().to_string_lossy();
                    let basename = filename
                        .rsplit_once('.')
                        .map_or(filename.as_ref(), |(base, _)| base);
                    format!("{basename}.txt")
                });

            let mut output_file_path = output_path.clone();
            output_file_path.push(output_file);

            generate_workload(&contents, output_file_path)?;
        }
    } else if workload_path.is_file() {
        let contents = fs::read_to_string(&workload_path)?;

        let output_file = workload_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|stem| format!("{stem}.txt"))
            .unwrap_or_else(|| format!("{}.txt", workload_path.display()));

        let mut output_file_path = output_path.clone();
        output_file_path.push(output_file);

        generate_workload(&contents, output_file_path)?;
    } else {
        unreachable!("Path is neither a file nor a directory");
    };

    return Ok(());
}

/// Prints the json schema for IDE integration.
fn invoke_schema() -> Result<()> {
    let schema_str = generate_workload_spec_schema().context("Schema generation failed.")?;
    println!("{schema_str}");
    return Ok(());
}
