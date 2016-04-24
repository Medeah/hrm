use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let s;
    match env::args().nth(1) {
        None => {
            println!("first args should be file name");
            std::process::exit(-1);
        },
        Some(name) => s = name,
    }
    println!("Running {}", s);
    let mut f = File::open(s).expect("Failed to open file");
    let mut buffer: [u8; 32] = [0; 32];
    f.read(&mut buffer).expect("Failed to read file");

    println!("The bytes: {:?}", buffer);


    let mut point: u8 = 0;
    let mut inhand: u8 = 0;
    let mut board: [u8; 25] = [0; 25];
    let mut input = vec![1, 2, 3, 4, 5];
    loop {
        match buffer[point as usize] {
            //INBOX
            105 => {
                inhand = input.pop().expect("no more input");
                point += 1;
            },
            // OUTBOX
            111 => {
                 println!("{}", inhand);
                 point += 1;
            },
            // //COPYFROM
            // 102 =>{
            //     accumulator += 1
            // },
            // //COPYFROM Indirect
            // 103 =>{
            //     accumulator += 1
            // },
            //COPYTO
            116 => {
                let arg = buffer[(point+1) as usize];
                board[arg as usize] = inhand;
                point += 2;
            },
            //COPYTO Indirect
            // 99 =>{
            //     accumulator += 1
            // },
            //ADD
            97 =>{
                 let arg = buffer[(point+1) as usize];
                 inhand += board[arg as usize];
                 point += 2;
            },
            // //ADD Indirect
            // 98 =>{
            //     accumulator += 1
            // },
            // //SUB
            // 115 =>{
            //     accumulator += 1
            // },
            // //SUB Indirect
            // 107 =>{
            //     accumulator += 1
            // }
            // //BUMPUP
            // 117 =>{
            //     accumulator += 1
            // },
            // //BUMPUP Indirect
            // 104 =>{
            //     accumulator += 1
            // },
            // //BUMPDN
            // 100 =>{
            //     accumulator += 1
            // },
            // //BUMPDN Indirect
            // 101 =>{
            //     accumulator += 1
            // },
            //JUMP
            106 =>{
                let arg = buffer[(point+1) as usize];
                point = arg;
            },
            // //JUMPZ
            // 122 =>{
            //     accumulator += 1
            // },
            // //JUMPN
            // 110 =>{
            //     accumulator += 1
            // }
            _ => { /* ignore everything else */ }
        }
    }
}
