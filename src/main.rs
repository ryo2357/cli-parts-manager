use clap::{Parser, Subcommand};

pub const PARTS_MASTER_FILE_PATH:&str = "parts_master.yaml";
pub const CSV_MASTER_FILE_PATH:&str = "parts_master.csv";
mod models;
mod controllers;

#[derive(Parser)]
#[command(name = "cli-parts-manager")]
#[command(about = "parts manager cli app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(help = "Path to the file to add")]
        file_path: String,
    },
    Check {
        #[arg(help = "Path to the list file")]
        file_path1: String,
        #[arg(help = "Path to the output file")]
        file_path2: String,
    },
    Convert
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { file_path }) => {
            controllers::add_parts_to_master::add_parts(file_path, PARTS_MASTER_FILE_PATH)?;
        }
        Some(Commands::Check { file_path1, file_path2 }) => {
            controllers::check_model_list::check_model_list(file_path1, file_path2, PARTS_MASTER_FILE_PATH)?;
        }
        // Some(Commands::Completion { shell }) => {
        //     let shell: &str = shell;
        //     match shell {
        //         "bash" => generate(Bash, &mut Cli::command(), "myapp", &mut io::stdout()),
        //         _ => eprintln!("Unsupported shell"),
        //     }
        // }
        Some(Commands::Convert) => {
            controllers::convert_to_csv::convert_to_csv(PARTS_MASTER_FILE_PATH, CSV_MASTER_FILE_PATH)?;
        }
        None => {}
    }

    Ok(())
}
