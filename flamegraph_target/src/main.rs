//! Allow the flamgraphs to be generate for gtlf2 files with slow load times.
//!
use std::path::PathBuf;

use asset_importer_rs_core::{AiImporterExt, default_file_loader};
use asset_importer_rs_gltf::Gltf2Importer;
use clap::Parser;
use log::error;
use log::info;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    fname: Option<PathBuf>,
}

fn main() {
    console_log::init_with_level(log::Level::Error).expect("error initializing log");
    let cli = Cli::parse();

    if let Some(path) = cli.fname.as_deref() {
        println!("filename: {}", path.display());
        let importer = Gltf2Importer;
        let scene = importer.read_file(path, default_file_loader).unwrap();
        info!("scene");
        info!("{scene:#?}");
        println!("parsing complete.")
    } else {
        error!("failed to load file.")
    }
}
