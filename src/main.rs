
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

mod execution;
use crate::execution::instructions::{ Instructions };
use crate::execution::program_context::{ ProgramContext, Rom, ROMOutOfBoundsError };

use clap::{ Parser, Subcommand };

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Disassemble {
        #[clap(short, long, parse(from_os_str))]
        filename: PathBuf,
    },
    Run {
        #[clap(short, long, parse(from_os_str))]
        filename: PathBuf,
    },
}

fn run(filename: &PathBuf) {

    let mut rom = load_rom_from_file(filename);
    let mut program_context: ProgramContext = ProgramContext::new(rom);

    loop {
        match program_context.rom.next_byte() {
            Err(err) => {
                println!("{}", err);
                break
            },
            Ok(opcode) => {
                match Instructions.get(&opcode) {
                    Some(instruction) => {
                        println!("{}: {:?}", opcode, instruction);
                        instruction.execute(&mut program_context);
                    },
                    None => println!("This should raise an exception. OpCode missing {}", opcode)
                }
            }
        }
    }
}

fn load_rom_from_file(filename: &PathBuf) -> Rom {
    let mut file = match File::open(&filename.as_path()) {
        Err(err) => panic!("Failed to open file: {}, {}", filename.as_path().display(), err),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => panic!("Failed to read file: {}, {}", filename.as_path().display(), err),
        Ok(_) => println!("Contents: {}", contents),
    }

    Rom::from_string(&contents)
}

fn disassemble(filename: &PathBuf) {
    println!("Decompiling {}", filename.as_path().display());
    let mut prog = load_rom_from_file(filename);
    match prog.disassemble() {
        Err(err) => println!("{}", err),
        Ok(_) => {}
    }
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Disassemble { filename } => {
            disassemble(filename);
        },
        Commands::Run { filename } => {
            run(filename);
        }
    }
}
