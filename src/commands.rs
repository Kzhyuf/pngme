use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::anyhow;
use clap::{App, Arg, arg, ArgMatches, SubCommand};
use crate::{Error, Result};
use crate::chunk::Chunk;
use crate::png::Png;

pub struct Args{
    matches: ArgMatches,
}

impl Args {
    fn new() -> Self {
        Args{
            matches: App::new("pngme")
                .version("0.1")
                .author("unknown")
                .about("personal implementation of pngme")
                .subcommand(SubCommand::with_name("encode")
                    .about("encode info into png, <file path> <chunk type> <message> [output file]")
                    .args([
                        arg!(<file_path> "file path"),
                        arg!(<chunk_type> "chunk type"),
                        arg!(<message> "message"),
                        arg!([output_file] "output file")
                    ]))
                .subcommand(SubCommand::with_name("decode")
                    .about("decode from a png file with specified chunk type, <file path>, <chunk type>")
                    .args([
                        arg!(<file_path> "file path"),
                        arg!(<chunk_type> "chunk type")
                    ]))
                .subcommand(SubCommand::with_name("remove")
                    .about("remove chunk type from a png file, <file path> <chunk type>")
                    .args([
                        arg!(<file_path> "file path"),
                        arg!(<chunk_type> "chunk type"),
                    ]))
                .subcommand(SubCommand::with_name("print")
                    .about("print file info")
                    .arg(arg!(<file_path> "file path")))
                .get_matches(),
        }
    }

    pub fn parse() -> Result<()>{
        let args = Self::new();
        match args.matches.subcommand() {
            Some(("encode", sub_cmd)) => Self::handle_encode(sub_cmd),
            Some(("decode", sub_cmd)) => Self::handle_decode(sub_cmd),
            Some(("remove", sub_cmd)) => Self::handle_remove_chunk_type(sub_cmd),
            Some(("print", sub_cmd)) => Self::handle_print(sub_cmd),
            _ => {
                Err(anyhow!("command not found"))
            },
        }
    }


    fn handle_encode(matches: &ArgMatches) -> Result<()> {
        let path_buf = PathBuf::from(matches.get_one::<String>("file_path").unwrap());
        let chunk_type = matches.get_one::<String>("chunk_type").unwrap();
        let msg:&str = matches.get_one::<String>("message").unwrap();
        let mut png = Png::from_file(&path_buf).unwrap();
        png.append_chunk(Chunk::new(chunk_type.parse()?, msg.as_bytes().to_vec()));

        if let Some(output_file) = matches.get_one::<String>("output_file") {
            let mut file = File::create(output_file)?;
            file.write_all(png.as_bytes().as_slice())?;
        }else{
            let mut file = File::create(&path_buf)?;
            file.write_all(png.as_bytes().as_slice())?;
        }
        println!("message encoded");
        Ok(())
    }

    fn handle_decode(matches: &ArgMatches) -> Result<()> {
        let path_buf = PathBuf::from(matches.get_one::<String>("file_path").unwrap());
        let png = Png::from_file(path_buf).unwrap();

        let chunk_type = matches.get_one::<String>("chunk_type").unwrap();
        if let Some(chunk) = png.get_chunk(chunk_type) {
            println!("chunk data: {}", chunk.data_as_string()?);
        }else {
            println!("no such chunk type");
        }
        Ok(())
    }

    fn handle_remove_chunk_type(matches: &ArgMatches) -> Result<()> {
        let path_buf = PathBuf::from(matches.get_one::<String>("file_path").unwrap());
        let mut png = Png::from_file(path_buf).unwrap();

        let chunk_type = matches.get_one::<String>("chunk_type").unwrap();
        png.remove_chunk(chunk_type)?;
        println!("specified chunk type removed");
        Ok(())
    }

    fn handle_print(matches: &ArgMatches) -> Result<()> {
        let path_buf = PathBuf::from(matches.get_one::<String>("file_path").unwrap());
        let png = Png::from_file(path_buf)?;
        png.chunks().iter()
            .for_each(
                |chunk|
                    println!("{:?}", chunk.chunk_type())
            );
        Ok(())
    }
}