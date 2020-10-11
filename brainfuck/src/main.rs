use std::io::prelude::*;
use std::fs;
use std::path;
use std::env;
use std::num::Wrapping;
use std::io::stdin;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let code = read_code(args.get(1));
    let valid = check_code(&code[..]);

    if !valid {
        panic!("Can not read code");
    }

    run(&code[..]);
}

fn read_code(file: Option<&String>) -> Vec<u8>{
    let code_file = match file {
        Some(f) => f,
        None => panic!("File required")
    };

    let path = path::Path::new(code_file);

    if !path.is_file() {
        panic!("{} is not a file", code_file)
    }

    fs::read(path).unwrap()
}

fn check_code(code: &[u8]) -> bool {
    let mut depth = 0;

    for instruction in code {
        match *instruction as char {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => {}
        }

        if depth < 0 {
            return false
        }
    }

    depth == 0
}

fn run(code: &[u8]) {
    const MEMORY_SIZE: usize = 256;
    const WRAPPING_ONE_USIZE:Wrapping<usize> = Wrapping(1);
    const WRAPPING_ONE_U8:Wrapping<u8> = Wrapping(1);
    const WRAPPING_MEMORY_POINTER_MASK:Wrapping<usize> = Wrapping(MEMORY_SIZE - 1);
    let mut memory = [Wrapping(0); MEMORY_SIZE];
    let mut memory_pointer = Wrapping(0usize);
    let mut instruction_pointer = 0;
    let mut stack = Vec::new();

    while instruction_pointer < code.len() {
        let instruction = code[instruction_pointer] as char;
        match instruction {
            '>' => memory_pointer = (memory_pointer + WRAPPING_ONE_USIZE) & WRAPPING_MEMORY_POINTER_MASK,
            '<' => memory_pointer = (memory_pointer - WRAPPING_ONE_USIZE) & WRAPPING_MEMORY_POINTER_MASK,
            '+' => memory[memory_pointer.0] = memory[memory_pointer.0] + WRAPPING_ONE_U8,
            '-' => memory[memory_pointer.0] -= WRAPPING_ONE_U8,
            ',' => memory[memory_pointer.0] = Wrapping(stdin().bytes().next().unwrap_or(Ok(0)).unwrap()),
            '.' => print!("{}", memory[memory_pointer.0].0 as char),
            '[' => {
                if memory[memory_pointer.0].0 == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        instruction_pointer += 1;
                        match code[instruction_pointer] as char {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                    }
                } else {
                    stack.push(instruction_pointer);
                }
            },
            ']' => {
                if memory[memory_pointer.0].0 == 0 {
                    stack.pop();
                } else {
                    instruction_pointer = *stack.last().unwrap();
                }
            },
            _ => {}
        }
        instruction_pointer+=1;
    }
}