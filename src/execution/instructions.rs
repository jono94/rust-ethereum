

use std::collections::HashMap;
use std::fmt;

use super::program_context::{ ProgramContext, ProgramError };
use super::types::{ u256 };

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OpCode {
    // 0x00: Stop and Arithmetic Operations
    Stop = 0x00,
    Add = 0x01,
    Mul = 0x02,
    Sub = 0x03,
    Div = 0x04,
    Sdiv = 0x05,
    Mod = 0x06,
    Smod = 0x07,
    AddMod = 0x08,
    MulMod = 0x09,
    Exp = 0x0a,
    SignExtend = 0x0b,
    // 0x10: Comparison and Bitwise Logic Operations
    Lt = 0x10,
    Gt = 0x11,
    Slt = 0x12,
    Sgt = 0x13,
    r#Eq = 0x14,
    IsZero = 0x15,
    And = 0x16,
    Or = 0x17,
    Xor = 0x18,
    Not = 0x19,
    Byte = 0x1a,
    Shl = 0x1b,
    Shr = 0x1c,
    Sar = 0x1d,
    // 0x20: KECCAK256
    Keccak256 = 0x20,
    // 0x30: Environmental Information
    Address = 0x30,
    Balance = 0x31,
    Origin = 0x32,
    Caller = 0x33,
    CallValue = 0x34,
    CallDataLoad = 0x35,
    CallDataSize = 0x36,
    CallDataCopy = 0x37,
    CodeSize = 0x38,
    CodeCopy = 0x39,
    GasPrice = 0x3a,
    ExtCodeSize = 0x3b,
    ExtCodeCopy = 0x3c,
    ReturnDataSize = 0x3d,
    ReturnDataCopy = 0x3e,
    ExtCodeHash = 0x3f,
    // 0x40: Block Information
    BlockHash = 0x40,
    Coinbase = 0x41,
    Timestamp = 0x42,
    Number = 0x43,
    Difficulty = 0x44,
    GasLimit = 0x45,
    ChainId = 0x46,
    SelfBalance = 0x47,
    // 0x50: Stack, Memory, Storage and Flow Operations
    Pop = 0x50,
    MLoad = 0x51,
    MStore = 0x52,
    MStore8 = 0x53,
    SLoad = 0x54,
    SStore = 0x55,
    Jump = 0x56,
    JumpI = 0x57,
    PC = 0x58,
    MSize = 0x59,
    Gas = 0x5a,
    JumpDest = 0x5b,
    // 0x60 and 0x70: Push Operations
    Push1 = 0x60,
    Push2 = 0x61,
    Push3 = 0x62,
    Push4 = 0x63,
    Push5 = 0x64,
    Push6 = 0x65,
    Push7 = 0x66,
    Push8 = 0x67,
    Push9 = 0x68,
    Push10 = 0x69,
    Push11 = 0x6a,
    Push12 = 0x6b,
    Push13 = 0x6c,
    Push14 = 0x6d,
    Push15 = 0x6e,
    Push16 = 0x6f,
    Push17 = 0x70,
    Push18 = 0x71,
    Push19 = 0x72,
    Push20 = 0x73,
    Push21 = 0x74,
    Push22 = 0x75,
    Push23 = 0x76,
    Push24 = 0x77,
    Push25 = 0x78,
    Push26 = 0x79,
    Push27 = 0x7a,
    Push28 = 0x7b,
    Push29 = 0x7c,
    Push30 = 0x7d,
    Push31 = 0x7e,
    Push32 = 0x7f,
    // 0x80: Duplication Operations
    Dup1 = 0x80,
    Dup2 = 0x81,
    Dup3 = 0x82,
    Dup4 = 0x83,
    Dup5 = 0x84,
    Dup6 = 0x85,
    Dup7 = 0x86,
    Dup8 = 0x87,
    Dup9 = 0x88,
    Dup10 = 0x89,
    Dup11 = 0x8a,
    Dup12 = 0x8b,
    Dup13 = 0x8c,
    Dup14 = 0x8d,
    Dup15 = 0x8e,
    Dup16 = 0x8f,
    // 0x90: Exchange Operations
    Swap1 = 0x90,
    Swap2 = 0x91,
    Swap3 = 0x92,
    Swap4 = 0x93,
    Swap5 = 0x94,
    Swap6 = 0x95,
    Swap7 = 0x96,
    Swap8 = 0x97,
    Swap9 = 0x98,
    Swap10 = 0x99,
    Swap11 = 0x9a,
    Swap12 = 0x9b,
    Swap13 = 0x9c,
    Swap14 = 0x9d,
    Swap15 = 0x9e,
    Swap16 = 0x9f,
    // 0xa0: Logging Operations
    Log0 = 0xa0,
    Log1 = 0xa1,
    Log2 = 0xa2,
    Log3 = 0xa3,
    Log4 = 0xa4,
    // 0xf0: System Operations
    Create = 0xf0,
    Call = 0xf1,
    CallCode = 0xf2,
    Return = 0xf3,
    DelegateCall = 0xf4,
    Create2 = 0xf5,
    StaticCall = 0xfa,
    Revert = 0xfd,
    Invalid = 0xfe,
    SelfDestruct = 0xff,
}

lazy_static! {
    pub static ref Instructions: HashMap<u8, Instruction> = HashMap::from([
        // 0x00: Stop and Arithmetic Operations
        (OpCode::Stop as u8, Instruction { value: OpCode::Stop as u8, mnemonic: "STOP", stack_items_removed: 0, stack_items_added: 0, rom_items_used: 0, execute: stop }),
        (OpCode::Add as u8, Instruction { value: OpCode::Add as u8, mnemonic: "ADD", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: add }),
        (OpCode::Mul as u8, Instruction { value: OpCode::Mul as u8, mnemonic: "MUL", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: mul }),
        (OpCode::Sub as u8, Instruction { value: OpCode::Sub as u8, mnemonic: "SUB", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: sub }),
        (OpCode::Div as u8, Instruction { value: OpCode::Div as u8, mnemonic: "DIV", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Sdiv as u8, Instruction { value: OpCode::Sdiv as u8, mnemonic: "SDIV", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Mod as u8, Instruction { value: OpCode::Mod as u8, mnemonic: "MOD", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: f_mod }),
        (OpCode::Smod as u8, Instruction { value: OpCode::Smod as u8, mnemonic: "SMOD", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::AddMod as u8, Instruction { value: OpCode::AddMod as u8, mnemonic: "ADDMOD", stack_items_removed: 3, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::MulMod as u8, Instruction { value: OpCode::MulMod as u8, mnemonic: "MULMOD", stack_items_removed: 3, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Exp as u8, Instruction { value: OpCode::Exp as u8, mnemonic: "EXP", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::SignExtend as u8, Instruction { value: OpCode::SignExtend as u8, mnemonic: "SIGNEXTEND", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        // 0x10: Comparison and Bitwise Logic Operations
        (OpCode::Lt as u8, Instruction { value: OpCode::Lt as u8, mnemonic: "LT", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Gt as u8, Instruction { value: OpCode::Gt as u8, mnemonic: "GT", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Slt as u8, Instruction { value: OpCode::Slt as u8, mnemonic: "SLT", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Sgt as u8, Instruction { value: OpCode::Sgt as u8, mnemonic: "SGT", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Eq as u8, Instruction { value: OpCode::Eq as u8, mnemonic: "EQ", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::IsZero as u8, Instruction { value: OpCode::IsZero as u8, mnemonic: "ISZERO", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::And as u8, Instruction { value: OpCode::And as u8, mnemonic: "AND", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Or as u8, Instruction { value: OpCode::Or as u8, mnemonic: "OR", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Xor as u8, Instruction { value: OpCode::Xor as u8, mnemonic: "XOR", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Not as u8, Instruction { value: OpCode::Not as u8, mnemonic: "NOT", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Byte as u8, Instruction { value: OpCode::Byte as u8, mnemonic: "BYTE", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Shl as u8, Instruction { value: OpCode::Shl as u8, mnemonic: "SHL", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Shr as u8, Instruction { value: OpCode::Shr as u8, mnemonic: "SHR", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Sar as u8, Instruction { value: OpCode::Sar as u8, mnemonic: "SAR", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        // 0x20: KECCAK256
        (OpCode::Keccak256 as u8, Instruction { value: OpCode::Keccak256 as u8, mnemonic: "KECCAK256", stack_items_removed: 2, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        // 0x30: Environmental Information
        (OpCode::Address as u8, Instruction { value: OpCode::Address as u8, mnemonic: "ADDRESS", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Balance as u8, Instruction { value: OpCode::Balance as u8, mnemonic: "BALANCE", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Origin as u8, Instruction { value: OpCode::Origin as u8, mnemonic: "ORIGIN", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Caller as u8, Instruction { value: OpCode::Caller as u8, mnemonic: "CALLER", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CallValue as u8, Instruction { value: OpCode::CallValue as u8, mnemonic: "CALLVALUE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CallDataLoad as u8, Instruction { value: OpCode::CallDataLoad as u8, mnemonic: "CALLDATALOAD", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CallDataSize as u8, Instruction { value: OpCode::CallDataSize as u8, mnemonic: "CALLDATASIZE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CallDataCopy as u8, Instruction { value: OpCode::CallDataCopy as u8, mnemonic: "CALLDATACOPY", stack_items_removed: 3, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::CodeSize as u8, Instruction { value: OpCode::CodeSize as u8, mnemonic: "CODESIZE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CodeCopy as u8, Instruction { value: OpCode::CodeCopy as u8, mnemonic: "CODECOPY", stack_items_removed: 3, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::GasPrice as u8, Instruction { value: OpCode::GasPrice as u8, mnemonic: "GASPRICE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::ExtCodeSize as u8, Instruction { value: OpCode::ExtCodeSize as u8, mnemonic: "EXTCODESIZE", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::ExtCodeCopy as u8, Instruction { value: OpCode::ExtCodeCopy as u8, mnemonic: "EXTCODECOPY", stack_items_removed: 4, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::ReturnDataSize as u8, Instruction { value: OpCode::ReturnDataSize as u8, mnemonic: "RETURNDATASIZE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::ReturnDataCopy as u8, Instruction { value: OpCode::ReturnDataCopy as u8, mnemonic: "RETURNDATACOPY", stack_items_removed: 3, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::ExtCodeHash as u8, Instruction { value: OpCode::ExtCodeHash as u8, mnemonic: "EXTCODEHASH", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        // 0x40: Block Information
        (OpCode::BlockHash as u8, Instruction { value: OpCode::BlockHash as u8, mnemonic: "BLOCKHASH", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Coinbase as u8, Instruction { value: OpCode::Coinbase as u8, mnemonic: "COINBASE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Timestamp as u8, Instruction { value: OpCode::Timestamp as u8, mnemonic: "TIMESTAMP", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Number as u8, Instruction { value: OpCode::Number as u8, mnemonic: "NUMBER", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Difficulty as u8, Instruction { value: OpCode::Difficulty as u8, mnemonic: "DIFFICULTY", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::GasLimit as u8, Instruction { value: OpCode::GasLimit as u8, mnemonic: "GASLIMIT", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::ChainId as u8, Instruction { value: OpCode::ChainId as u8, mnemonic: "CHAINID", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::SelfBalance as u8, Instruction { value: OpCode::SelfBalance as u8, mnemonic: "SELFBALANCE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        // 0x50: Stack, Memory, Storage and Flow Operations
        (OpCode::Pop as u8, Instruction { value: OpCode::Pop as u8, mnemonic: "POP", stack_items_removed: 1, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::MLoad as u8, Instruction { value: OpCode::MLoad as u8, mnemonic: "MLOAD", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::MStore as u8, Instruction { value: OpCode::MStore as u8, mnemonic: "MSTORE", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::MStore8 as u8, Instruction { value: OpCode::MStore8 as u8, mnemonic: "MSTORE8", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::SLoad as u8, Instruction { value: OpCode::SLoad as u8, mnemonic: "SLOAD", stack_items_removed: 1, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::SStore as u8, Instruction { value: OpCode::SStore as u8, mnemonic: "SSTORE", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Jump as u8, Instruction { value: OpCode::Jump as u8, mnemonic: "JUMP", stack_items_removed: 1, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::JumpI as u8, Instruction { value: OpCode::JumpI as u8, mnemonic: "JUMPI", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::PC as u8, Instruction { value: OpCode::PC as u8, mnemonic: "PC", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::MSize as u8, Instruction { value: OpCode::MSize as u8, mnemonic: "MSIZE", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Gas as u8, Instruction { value: OpCode::Gas as u8, mnemonic: "GAS", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::JumpDest as u8, Instruction { value: OpCode::JumpDest as u8, mnemonic: "JUMPDEST", stack_items_removed: 0, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        // 0x60 and 0x70: Push Operations
        (OpCode::Push1 as u8, Instruction { value: OpCode::Push1 as u8, mnemonic: "PUSH1", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 1, execute: push }),
        (OpCode::Push2 as u8, Instruction { value: OpCode::Push2 as u8, mnemonic: "PUSH2", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 2, execute: push }),
        (OpCode::Push3 as u8, Instruction { value: OpCode::Push3 as u8, mnemonic: "PUSH3", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 3, execute: push }),
        (OpCode::Push4 as u8, Instruction { value: OpCode::Push4 as u8, mnemonic: "PUSH4", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 4, execute: push }),
        (OpCode::Push5 as u8, Instruction { value: OpCode::Push5 as u8, mnemonic: "PUSH5", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 5, execute: push }),
        (OpCode::Push6 as u8, Instruction { value: OpCode::Push6 as u8, mnemonic: "PUSH6", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 6, execute: push }),
        (OpCode::Push7 as u8, Instruction { value: OpCode::Push7 as u8, mnemonic: "PUSH7", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 7, execute: push }),
        (OpCode::Push8 as u8, Instruction { value: OpCode::Push8 as u8, mnemonic: "PUSH8", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 8, execute: push }),
        (OpCode::Push9 as u8, Instruction { value: OpCode::Push9 as u8, mnemonic: "PUSH9", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 9, execute: push }),
        (OpCode::Push10 as u8, Instruction { value: OpCode::Push10 as u8, mnemonic: "PUSH10", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 10, execute: push }),
        (OpCode::Push11 as u8, Instruction { value: OpCode::Push11 as u8, mnemonic: "PUSH11", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 11, execute: push }),
        (OpCode::Push12 as u8, Instruction { value: OpCode::Push12 as u8, mnemonic: "PUSH12", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 12, execute: push }),
        (OpCode::Push13 as u8, Instruction { value: OpCode::Push13 as u8, mnemonic: "PUSH13", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 13, execute: push }),
        (OpCode::Push14 as u8, Instruction { value: OpCode::Push14 as u8, mnemonic: "PUSH14", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 14, execute: push }),
        (OpCode::Push15 as u8, Instruction { value: OpCode::Push15 as u8, mnemonic: "PUSH15", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 15, execute: push }),
        (OpCode::Push16 as u8, Instruction { value: OpCode::Push16 as u8, mnemonic: "PUSH16", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 16, execute: push }),
        (OpCode::Push17 as u8, Instruction { value: OpCode::Push17 as u8, mnemonic: "PUSH17", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 17, execute: push }),
        (OpCode::Push18 as u8, Instruction { value: OpCode::Push18 as u8, mnemonic: "PUSH18", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 18, execute: push }),
        (OpCode::Push19 as u8, Instruction { value: OpCode::Push19 as u8, mnemonic: "PUSH19", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 19, execute: push }),
        (OpCode::Push20 as u8, Instruction { value: OpCode::Push20 as u8, mnemonic: "PUSH20", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 20, execute: push }),
        (OpCode::Push21 as u8, Instruction { value: OpCode::Push21 as u8, mnemonic: "PUSH21", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 21, execute: push }),
        (OpCode::Push22 as u8, Instruction { value: OpCode::Push22 as u8, mnemonic: "PUSH22", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 22, execute: push }),
        (OpCode::Push23 as u8, Instruction { value: OpCode::Push23 as u8, mnemonic: "PUSH23", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 23, execute: push }),
        (OpCode::Push24 as u8, Instruction { value: OpCode::Push24 as u8, mnemonic: "PUSH24", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 24, execute: push }),
        (OpCode::Push25 as u8, Instruction { value: OpCode::Push25 as u8, mnemonic: "PUSH25", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 25, execute: push }),
        (OpCode::Push26 as u8, Instruction { value: OpCode::Push26 as u8, mnemonic: "PUSH26", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 26, execute: push }),
        (OpCode::Push27 as u8, Instruction { value: OpCode::Push27 as u8, mnemonic: "PUSH27", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 27, execute: push }),
        (OpCode::Push28 as u8, Instruction { value: OpCode::Push28 as u8, mnemonic: "PUSH28", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 28, execute: push }),
        (OpCode::Push29 as u8, Instruction { value: OpCode::Push29 as u8, mnemonic: "PUSH29", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 29, execute: push }),
        (OpCode::Push30 as u8, Instruction { value: OpCode::Push30 as u8, mnemonic: "PUSH30", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 30, execute: push }),
        (OpCode::Push31 as u8, Instruction { value: OpCode::Push31 as u8, mnemonic: "PUSH31", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 31, execute: push }),
        (OpCode::Push32 as u8, Instruction { value: OpCode::Push32 as u8, mnemonic: "PUSH32", stack_items_removed: 0, stack_items_added: 1, rom_items_used: 32, execute: push }),
        // 0x80: Duplication Operations
        (OpCode::Dup1 as u8, Instruction { value: OpCode::Dup1 as u8, mnemonic: "DUP1", stack_items_removed: 1, stack_items_added: 2, rom_items_used: 0, execute: todo }),
        (OpCode::Dup2 as u8, Instruction { value: OpCode::Dup2 as u8, mnemonic: "DUP2", stack_items_removed: 2, stack_items_added: 3, rom_items_used: 0, execute: todo }),
        (OpCode::Dup3 as u8, Instruction { value: OpCode::Dup3 as u8, mnemonic: "DUP3", stack_items_removed: 3, stack_items_added: 4, rom_items_used: 0, execute: todo }),
        (OpCode::Dup4 as u8, Instruction { value: OpCode::Dup4 as u8, mnemonic: "DUP4", stack_items_removed: 4, stack_items_added: 5, rom_items_used: 0, execute: todo }),
        (OpCode::Dup5 as u8, Instruction { value: OpCode::Dup5 as u8, mnemonic: "DUP5", stack_items_removed: 5, stack_items_added: 6, rom_items_used: 0, execute: todo }),
        (OpCode::Dup6 as u8, Instruction { value: OpCode::Dup6 as u8, mnemonic: "DUP6", stack_items_removed: 6, stack_items_added: 7, rom_items_used: 0, execute: todo }),
        (OpCode::Dup7 as u8, Instruction { value: OpCode::Dup7 as u8, mnemonic: "DUP7", stack_items_removed: 7, stack_items_added: 8, rom_items_used: 0, execute: todo }),
        (OpCode::Dup8 as u8, Instruction { value: OpCode::Dup8 as u8, mnemonic: "DUP8", stack_items_removed: 8, stack_items_added: 9, rom_items_used: 0, execute: todo }),
        (OpCode::Dup9 as u8, Instruction { value: OpCode::Dup9 as u8, mnemonic: "DUP9", stack_items_removed: 9, stack_items_added: 10, rom_items_used: 0, execute: todo }),
        (OpCode::Dup10 as u8, Instruction { value: OpCode::Dup10 as u8, mnemonic: "DUP10", stack_items_removed: 10, stack_items_added: 11, rom_items_used: 0, execute: todo }),
        (OpCode::Dup11 as u8, Instruction { value: OpCode::Dup11 as u8, mnemonic: "DUP11", stack_items_removed: 11, stack_items_added: 12, rom_items_used: 0, execute: todo }),
        (OpCode::Dup12 as u8, Instruction { value: OpCode::Dup12 as u8, mnemonic: "DUP12", stack_items_removed: 12, stack_items_added: 13, rom_items_used: 0, execute: todo }),
        (OpCode::Dup13 as u8, Instruction { value: OpCode::Dup13 as u8, mnemonic: "DUP13", stack_items_removed: 13, stack_items_added: 14, rom_items_used: 0, execute: todo }),
        (OpCode::Dup14 as u8, Instruction { value: OpCode::Dup14 as u8, mnemonic: "DUP14", stack_items_removed: 14, stack_items_added: 15, rom_items_used: 0, execute: todo }),
        (OpCode::Dup15 as u8, Instruction { value: OpCode::Dup15 as u8, mnemonic: "DUP15", stack_items_removed: 15, stack_items_added: 16, rom_items_used: 0, execute: todo }),
        (OpCode::Dup16 as u8, Instruction { value: OpCode::Dup16 as u8, mnemonic: "DUP16", stack_items_removed: 16, stack_items_added: 17, rom_items_used: 0, execute: todo }),
        // 0x90: Exchange Operations
        (OpCode::Swap1 as u8, Instruction { value: OpCode::Swap1 as u8, mnemonic: "SWAP1", stack_items_removed: 2, stack_items_added: 2, rom_items_used: 0, execute: todo }),
        (OpCode::Swap2 as u8, Instruction { value: OpCode::Swap2 as u8, mnemonic: "SWAP2", stack_items_removed: 3, stack_items_added: 3, rom_items_used: 0, execute: todo }),
        (OpCode::Swap3 as u8, Instruction { value: OpCode::Swap3 as u8, mnemonic: "SWAP3", stack_items_removed: 4, stack_items_added: 4, rom_items_used: 0, execute: todo }),
        (OpCode::Swap4 as u8, Instruction { value: OpCode::Swap4 as u8, mnemonic: "SWAP4", stack_items_removed: 5, stack_items_added: 5, rom_items_used: 0, execute: todo }),
        (OpCode::Swap5 as u8, Instruction { value: OpCode::Swap5 as u8, mnemonic: "SWAP5", stack_items_removed: 6, stack_items_added: 6, rom_items_used: 0, execute: todo }),
        (OpCode::Swap6 as u8, Instruction { value: OpCode::Swap6 as u8, mnemonic: "SWAP6", stack_items_removed: 7, stack_items_added: 7, rom_items_used: 0, execute: todo }),
        (OpCode::Swap7 as u8, Instruction { value: OpCode::Swap7 as u8, mnemonic: "SWAP7", stack_items_removed: 8, stack_items_added: 8, rom_items_used: 0, execute: todo }),
        (OpCode::Swap8 as u8, Instruction { value: OpCode::Swap8 as u8, mnemonic: "SWAP8", stack_items_removed: 9, stack_items_added: 9, rom_items_used: 0, execute: todo }),
        (OpCode::Swap9 as u8, Instruction { value: OpCode::Swap9 as u8, mnemonic: "SWAP9", stack_items_removed: 10, stack_items_added: 10, rom_items_used: 0, execute: todo }),
        (OpCode::Swap10 as u8, Instruction { value: OpCode::Swap10 as u8, mnemonic: "SWAP10", stack_items_removed: 11, stack_items_added: 11, rom_items_used: 0, execute: todo }),
        (OpCode::Swap11 as u8, Instruction { value: OpCode::Swap11 as u8, mnemonic: "SWAP11", stack_items_removed: 12, stack_items_added: 12, rom_items_used: 0, execute: todo }),
        (OpCode::Swap12 as u8, Instruction { value: OpCode::Swap12 as u8, mnemonic: "SWAP12", stack_items_removed: 13, stack_items_added: 13, rom_items_used: 0, execute: todo }),
        (OpCode::Swap13 as u8, Instruction { value: OpCode::Swap13 as u8, mnemonic: "SWAP13", stack_items_removed: 14, stack_items_added: 14, rom_items_used: 0, execute: todo }),
        (OpCode::Swap14 as u8, Instruction { value: OpCode::Swap14 as u8, mnemonic: "SWAP14", stack_items_removed: 15, stack_items_added: 15, rom_items_used: 0, execute: todo }),
        (OpCode::Swap15 as u8, Instruction { value: OpCode::Swap15 as u8, mnemonic: "SWAP15", stack_items_removed: 16, stack_items_added: 16, rom_items_used: 0, execute: todo }),
        (OpCode::Swap16 as u8, Instruction { value: OpCode::Swap16 as u8, mnemonic: "SWAP16", stack_items_removed: 17, stack_items_added: 17, rom_items_used: 0, execute: todo }),
        // 0xa0: Logging Operations
        (OpCode::Log0 as u8, Instruction { value: OpCode::Log0 as u8, mnemonic: "LOG0", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Log1 as u8, Instruction { value: OpCode::Log1 as u8, mnemonic: "LOG1", stack_items_removed: 3, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Log2 as u8, Instruction { value: OpCode::Log2 as u8, mnemonic: "LOG2", stack_items_removed: 4, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Log3 as u8, Instruction { value: OpCode::Log3 as u8, mnemonic: "LOG3", stack_items_removed: 5, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Log4 as u8, Instruction { value: OpCode::Log4 as u8, mnemonic: "LOG4", stack_items_removed: 6, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        // 0xf0: System Operations
        (OpCode::Create as u8, Instruction { value: OpCode::Create as u8, mnemonic: "CREATE", stack_items_removed: 3, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Call as u8, Instruction { value: OpCode::Call as u8, mnemonic: "CALL", stack_items_removed: 7, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::CallCode as u8, Instruction { value: OpCode::CallCode as u8, mnemonic: "CALLCODE", stack_items_removed: 7, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Return as u8, Instruction { value: OpCode::Return as u8, mnemonic: "RETURN", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::DelegateCall as u8, Instruction { value: OpCode::DelegateCall as u8, mnemonic: "DELEGATECALL", stack_items_removed: 6, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Create2 as u8, Instruction { value: OpCode::Create2 as u8, mnemonic: "CREATE2", stack_items_removed: 4, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::StaticCall as u8, Instruction { value: OpCode::StaticCall as u8, mnemonic: "STATICCALL", stack_items_removed: 6, stack_items_added: 1, rom_items_used: 0, execute: todo }),
        (OpCode::Revert as u8, Instruction { value: OpCode::Revert as u8, mnemonic: "REVERT", stack_items_removed: 2, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::Invalid as u8, Instruction { value: OpCode::Invalid as u8, mnemonic: "INVALID", stack_items_removed: 0, stack_items_added: 0, rom_items_used: 0, execute: todo }),
        (OpCode::SelfDestruct as u8, Instruction { value: OpCode::SelfDestruct as u8, mnemonic: "SELFDESTRUCT", stack_items_removed: 1, stack_items_added: 0, rom_items_used: 0, execute: todo }),
    ]);
}



pub struct Instruction {
    pub value: u8,
    pub mnemonic: &'static str,
    pub stack_items_removed: u8, // delta
    pub stack_items_added: u8, // alpha
    pub rom_items_used: u8,
    //description: &str[100],
    pub execute: fn(opcode: u8, &mut ProgramContext) -> Result<(), ProgramError>,
}

impl Instruction {
    pub fn execute(&self, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
        //println!("Executing {}", self.value);
        (self.execute)(self.value, program_context)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "4: Instruction {{ value: {} }}", self.value)
    }
}

fn todo(opcode: u8, _program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    println!("TODO: Implement opcode: {}", opcode);
    Ok(())
}


// 0x00: Stop and Arithmetic Operations
fn stop(_opcode: u8, _program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    Err(ProgramError::Stopped)
}

fn add(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    program_context.stack.push(a + b);
    Ok(())
}

fn mul(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    program_context.stack.push(a * b);
    Ok(())
}

fn sub(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    program_context.stack.push(a - b);
    Ok(())
}

/*
fn div(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    let mut res = u256::from_u128s(0, 0);
    if b != 0 {
        res = a / b;
    }
    program_context.stack.push(res);
    Ok(())
}
*/

// TODO: sdiv

fn f_mod(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    let mut res = u256::zero();
    if b != u256::zero() {
        res = a % b;
    }
    program_context.stack.push(res);
    Ok(())
}

// TODO: smod, addmod, mulmod, exp, signextend

// 0x10: Comparison and Bitwise Logic Operations
fn lt(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    let mut res = u256::zero();
    if a < b {
        res = u256::from_u128s(0, 1);
    }
    program_context.stack.push(res);
    Ok(())
}

fn gt(_opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let a = program_context.stack.pop();
    let b = program_context.stack.pop();
    let mut res = u256::zero();
    if a > b {
        res = u256::from_u128s(0, 1);
    }
    program_context.stack.push(res);
    Ok(())
}


//
fn push(opcode: u8, program_context: &mut ProgramContext) -> Result<(), ProgramError> {
    let mut push_num = opcode + 1 - (OpCode::Push1 as u8); // Get number of pushes to make based upon opcode offset from push1
    let mut data: u256 = u256::zero();
    while push_num > 0 { // This could be much more efficient if a slice is returned instead...
         let byte_data: u256 = u256::from_u8(program_context.rom.next_byte()?);
         //data += byte_data << ((push_num-1) * 8);
         push_num -= 1;
         //println!("{:32x}", data);
    }
    program_context.stack.push(data);
    Ok(())
}
