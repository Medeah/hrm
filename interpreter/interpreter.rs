use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {

    let s = env::args().nth(1).expect ("first args should be file name");
    println!("Running {}", s);

    let mut f = File::open(s).expect("Failed to open file");
    let mut program = Vec::new();
    f.read_to_end(&mut program).expect("Failed to read file");
    let program = program;

    println!("The bytes: {:?}", program);


    let mut instruction_pointer: u8 = 0;
    let mut in_hand: i8 = 0;
    let mut board: [i8; 25] = [0; 25];
    let mut input = vec![0, 25, 28, 4, 90, 72, 43, 22, 18, 99, 100, 40, 16, 57, 6, 80, 37, 85, 89, 88];
    loop {
        //println!("ip: {}", instruction_pointer);
        match program[instruction_pointer as usize] {
            // INBOX
            105 => {
                in_hand = input.pop().expect("no more input");
                instruction_pointer += 1;
            },
            // OUTBOX
            111 => {
                 println!("{}", in_hand);
                 instruction_pointer += 1;
            },
            // COPYFROM
            102 => {
                let arg = program[(instruction_pointer+1) as usize];
                in_hand = board[arg as usize];
                instruction_pointer += 2;
            },
            // COPYFROM Indirect
            103 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                in_hand = board[index as usize];
                instruction_pointer += 2;
            },
            // COPYTO
            116 => {
                let arg = program[(instruction_pointer+1) as usize];
                board[arg as usize] = in_hand;
                instruction_pointer += 2;
            },
            // COPYTO Indirect
            99 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                board[index as usize] = in_hand;
                instruction_pointer += 2;
            },
            // ADD
            97 => {
                 let arg = program[(instruction_pointer+1) as usize];
                 in_hand += board[arg as usize];
                 instruction_pointer += 2;
            },
            // ADD Indirect
            98 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                in_hand += board[index as usize];
                instruction_pointer += 2;
            },
            // SUB
            115 => {
                let arg = program[(instruction_pointer+1) as usize];
                in_hand -= board[arg as usize];
                instruction_pointer += 2;
            },
            // SUB Indirect
            107 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                in_hand -= board[index as usize];
                instruction_pointer += 2;
            }
            // BUMPUP
            117 => {
                let arg = program[(instruction_pointer+1) as usize];
                board[arg as usize] += 1;
                in_hand = board[arg as usize];
                instruction_pointer += 2;
            },
            // BUMPUP Indirect
            104 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                board[index as usize] += 1;
                in_hand = board[index as usize];
                instruction_pointer += 2;
            },
            // BUMPDN
            100 => {
                let arg = program[(instruction_pointer+1) as usize];
                board[arg as usize] -= 1;
                in_hand = board[arg as usize];
                instruction_pointer += 2;
            },
            // BUMPDN Indirect
            101 => {
                let arg = program[(instruction_pointer+1) as usize];
                let index = board[arg as usize];
                board[index as usize] -= 1;
                in_hand = board[index as usize];
                instruction_pointer += 2;
            },
            // JUMP
            106 => {
                let arg = program[(instruction_pointer+1) as usize];
                instruction_pointer = arg;
            },
            // JUMPZ
            122 => {
                let arg = program[(instruction_pointer+1) as usize];
                if in_hand == 0 {
                    instruction_pointer = arg;
                } else {
                    instruction_pointer += 2;
                }
            },
            // JUMPN
            110 => {
                let arg = program[(instruction_pointer+1) as usize];
                if in_hand < 0 {
                    instruction_pointer = arg;
                } else {
                    instruction_pointer += 2;
                }
            },
            _ => {
                println!("unknown opcode: {}", program[instruction_pointer as usize]);
                std::process::exit(-1);
            }
        }
    }
}
