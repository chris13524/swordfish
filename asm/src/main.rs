use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct LabelRef {
  label: String,
  addr: usize,
}

fn main() {
  let mut label_refs: Vec<LabelRef> = vec![];
  let mut labels: HashMap<String, u32> = HashMap::new();
  let mut out: Vec<u8> = vec![];

  parse("stdin", &mut io::stdin().lock().lines(), &mut label_refs, &mut labels, &mut out);

  for label_ref in label_refs {
    match labels.get(&label_ref.label) {
      None => panic!("label not found: {}", label_ref.label),
      Some(addr) => replace_at(&mut out, label_ref.addr, &addr.to_be_bytes()),
    }
  }

  io::stdout().lock().write(&out).unwrap();
}

fn replace_at(array: &mut [u8], index: usize, values: &[u8]) {
  for i in 0..values.len() {
    array[index + i] = values[i];
  }
}

fn parse(file: &str,
         lines: &mut dyn std::iter::Iterator<Item=io::Result<String>>,
         label_refs: &mut Vec<LabelRef>,
         labels: &mut HashMap<String, u32>,
         out: &mut Vec<u8>) {
  let mut line_number = 0;
  for line in lines {
    line_number += 1;

    let line = line.unwrap();
    if line.starts_with("#") { continue; }
    if line == "" { continue; }

    match line.find(' ') {
      None => {
        match line.as_str() {
          "HALT" => emit(out, 0, 0),
          "INP"  => emit(out, 1, 0),
          "OUT"  => emit(out, 2, 0),
          _ => panic!("unknown instruction `{}` on line {} of file {}", line, line_number, file),
        }
      },
      Some(index) => {
        let inst = &line[0..index];
        let param = &line[index+1..];
        match inst {
          "LOAD"    => emit_with_param(out, label_refs, 3,  param),
          "SAVE"    => emit_with_param(out, label_refs, 4,  param),
          "ADD"     => emit_with_param(out, label_refs, 5,  param),
          "SUB"     => emit_with_param(out, label_refs, 6,  param),
          "MUL"     => emit_with_param(out, label_refs, 7,  param),
          "DIV"     => emit_with_param(out, label_refs, 8,  param),
          "JMP"     => emit_with_param(out, label_refs, 9,  param),
          "IJMP"    => emit_with_param(out, label_refs, 10, param),
          "CJMP"    => emit_with_param(out, label_refs, 11, param),
          "LABEL"   => { labels.insert(param.to_string(), out.len() as u32); },
          "RAW"     => out.append(&mut std::fs::read(param).unwrap()),
          "INCLUDE" => {
            let file = std::fs::File::open(param).unwrap();
            parse(param, &mut io::BufReader::new(file).lines(), label_refs, labels, out);
          },
          _ => panic!("unknown instruction: {}", line),
        }
      },
    }
  }
}

fn emit(out: &mut Vec<u8>, inst: u32, param: u32) {
  out.extend_from_slice(&inst.to_be_bytes());
  out.extend_from_slice(&param.to_be_bytes());
}

fn emit_with_param(out: &mut Vec<u8>, label_refs: &mut Vec<LabelRef>, inst: u32, param: &str) {
  let address = parse_address(param, label_refs, out.len());
  emit(out, inst, address);
}

fn parse_address(value: &str, label_refs: &mut Vec<LabelRef>, ip: usize) -> u32 {
  if value.starts_with("0x") {
    return u32::from_str_radix(value.trim_start_matches("0x"), 16).unwrap();
  }

  if value.starts_with("0b") {
    return u32::from_str_radix(value.trim_start_matches("0b"), 2).unwrap();
  }

  if value.starts_with("0o") {
    return u32::from_str_radix(value.trim_start_matches("0o"), 8).unwrap();
  }

  return match value.parse() {
    Ok(val) => val,
    Err(_) => {
      label_refs.push(LabelRef {
        label: value.to_string(),
        addr: ip + 4,
      });
      0
    },
  }
}
