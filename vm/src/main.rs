use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut ram = load(&buffer);
    execute(&mut ram, &mut io::stdin().lock(), &mut io::stdout().lock());
}

fn load(buffer: &[u8]) -> Vec<u64> {
    if buffer.len() % 8 != 0 {
        panic!("buffer does not have a byte count that is a multiple of 8");
    }

    let count = buffer.len() / 8;
    let mut ram: Vec<u64> = Vec::with_capacity(count);
    for i in 0..count {
      ram.push(u64::from_be_bytes([
        buffer[i*8 + 0],
        buffer[i*8 + 1],
        buffer[i*8 + 2],
        buffer[i*8 + 3],
        buffer[i*8 + 4],
        buffer[i*8 + 5],
        buffer[i*8 + 6],
        buffer[i*8 + 7],
      ]));
    }

    return ram;
}

const HALT: u32 = 0;
const INP: u32 = 1;
const OUT: u32 = 2;
const LOAD: u32 = 3;
const SAVE: u32 = 4;
const ADD: u32 = 5;
const SUB: u32 = 6;
const MUL: u32 = 7;
const DIV: u32 = 8;
const JMP: u32 = 9;
const IJMP: u32 = 10;
const CJMP: u32 = 11;

fn execute(ram: &mut [u64], input: &mut impl Read, output: &mut impl Write) {
  if ram.len() >= u32::MAX as usize { panic!("ram size is greather than or equal to 2^32"); }

  let mut ip: u32 = 0;
  let mut ac: u64 = 0;
  loop {
    //eprintln!("ip: {}", ip);
    //eprintln!("ac: {}", ac);

    let inst = ram[ip as usize];
    //eprintln!("inst: {}", inst);
    let opcode = (inst >> 32) as u32;
    //eprintln!("opcode: {}", opcode);
    let addr = inst as u32 / 8;

    match opcode {
      HALT => return,
      INP => {
          let mut buff = [0u8; 8];
          if input.read_exact(&mut buff).is_err() { return; }
          ac = u64::from_be_bytes(buff);
      },
      OUT  => { output.write(&u64::to_be_bytes(ac)).unwrap(); },
      LOAD => ac  = ram[addr as usize],
      SAVE =>       ram[addr as usize] = ac,
      ADD  => ac += ram[addr as usize],
      SUB  => ac -= ram[addr as usize],
      MUL  => ac *= ram[addr as usize],
      DIV  => ac /= ram[addr as usize],
      JMP  => { ip = addr; continue; },
      IJMP => { ip = ram[addr as usize] as u32; continue; },
      CJMP => if ac == 0 { ip = addr; continue; },
      x => panic!("unexpected opcode {}", x),
    }

    ip += 1;
    if ip as usize >= ram.len() { return; }
  }
}
