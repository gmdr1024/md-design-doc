#![warn(rust_2018_idioms)]

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, App as ClapApp, Arg};

use md_designer::app::App;
use md_designer::rule::Rule;

fn main() -> Result<()> {
    // setup clap
    let clap = ClapApp::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("path")
                .required(true)
                .help("input file path (.md)"),
        )
        .arg(
            Arg::with_name("conf_path")
                .required(true)
                .help("config file path (.yml)"),
        )
        .get_matches();

    let path = Path::new(clap.value_of("path").unwrap());
    let input_text = fs::read_to_string(&path)?;
    let cfg_text = fs::read_to_string(clap.value_of("conf_path").unwrap())?;

    let rule = Rule::marshal(&cfg_text)?;

    let app = App::new(
        path.file_stem()
            .with_context(|| "Input file path is malformed")?
            .to_str()
            .unwrap(),
        &input_text,
        rule,
    )?;

    app.export_excel()?;

    Ok(())
}
