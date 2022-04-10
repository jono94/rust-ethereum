
use super::instructions::Instructions;
use super::types::u256;

#[derive(Debug)]
pub enum ProgramError {
    Stopped,
    ROMOutOfBoundsError(ROMOutOfBoundsError),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ProgramError::Stopped => write!(f, "Recieved STOP opcode"),
            ProgramError::ROMOutOfBoundsError(err) => write!(f, "{}", err),
        }
    }
}

pub struct ProgramContext {
    pub rom: Rom,
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
}

impl ProgramContext {
    pub fn new(rom: Rom) -> ProgramContext {
        ProgramContext { rom, stack: Stack::new(), memory: Memory::new(), storage: Storage::new() }
    }
}

// UTILS START
use std::{fmt::Write, num::ParseIntError};

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

// UTILS END

// ROM START
use std::fmt;

#[derive(Debug, Clone)]
pub struct ROMOutOfBoundsError {
    index: usize,
    max_rom_index: usize,
}

impl fmt::Display for ROMOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index {}, not in range [0, {}]", self.index, self.max_rom_index)
    }
}

pub struct Rom {
    rom: Vec<u8>,
    pc: u128, // Not sure what size this should be, I guess it could be infinite...
    size: usize,
}

impl Rom {
    pub fn new(rom: Vec<u8>) -> Rom {
        let size: usize = rom.len();
        Rom { rom, pc: 0, size }
    }

    pub fn from_string(s: &str) -> Rom {
        let rom = decode_hex(s).unwrap();
        let size: usize = rom.len();
        Rom { rom, pc: 0, size }
    }

    pub fn next_byte(&mut self) -> Result<u8, ProgramError> {
        let pc: usize = self.pc as usize;
        if pc < self.size {
            self.pc += 1;
            return Ok(self.rom[pc]);
        }
        Err(ProgramError::ROMOutOfBoundsError(ROMOutOfBoundsError { index: pc, max_rom_index: self.size - 1 }))
    }

    pub fn disassemble(&mut self) -> Result<(), ProgramError> {
        loop {
            let mut line: String = String::new();
            let opcode = &self.next_byte()?;
            if let Some(instruction) = &Instructions.get(&opcode) {
                line.push_str(format!("  {:6}", instruction.mnemonic).as_str());
                let mut rom_args = instruction.rom_items_used;
                while rom_args > 0 {
                    line.push_str(format!("  {:#04x}", self.next_byte()?).as_str());
                    rom_args -= 1;
                }
            } else {
                line.push_str(format!("  {:#04x}", opcode).as_str());
            }
            println!("{}", line);
        }
        Ok(())
    }
}

// ROM END

// STACK START

// TODO change to u256
pub struct Stack {
    stack: Vec<u256>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: vec![] }
    }

    pub fn push(&mut self, value: u256) {
        if self.stack.len() > 1023 {
            // Next item will result in 1024 to 1025
            println!("TODO: Implement stack overflow");
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> u256 {
        if self.stack.len() < 1 {
            // Next item will results in 0 to -1
            println!("TODO: Implement stack underflow");
        }
        self.stack.pop().unwrap()
    }
}

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: Vec::new() }
    }
}

// TODO: Change to u256
pub struct Storage {
    storage: Vec<u128>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage { storage: Vec::new() }
    }
}
