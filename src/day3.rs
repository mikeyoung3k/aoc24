use crate::BUF_SIZE;
use cortex_m::iprintln;
use cortex_m_semihosting::syscall;

extern crate alloc;
use alloc::vec::Vec;

pub fn run(stim: &mut cortex_m::peripheral::itm::Stim) {
    let sample_path = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day3.txt";
    let sample_path2 = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day3-2.txt";
    let real_path = r"C:\Users\Mike_\Documents\Rust\aoc24\real_data\day3.txt";
    {
        let res = pt1(sample_path);
        iprintln!(stim,"Sample part 1 result: {:?}",res);
        if res != 161 {
            iprintln!(stim,"Sample part 1 result does not match target of 161");
            panic!();
        }
    }
    {
        let res = pt2(sample_path2);
        iprintln!(stim,"Sample part 2 result {:?}",res);
        if res != 48 {
            iprintln!(stim,"Sample part 2 result does not match target of 48");
            panic!();
        }
    }
    let p1 = pt1(real_path);
    iprintln!(stim,"Part 1 result: {:?}",p1);
    let p2 = pt2(real_path);
    iprintln!(stim,"Part 2 result: {:?}", p2);
}

fn pt1(path: &str) -> u32 {
    let mut read = 0;
    let mut res = 0;
    let mut oldbuf: Vec<u8> = Vec::with_capacity(BUF_SIZE*2);
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf:[u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(), BUF_SIZE);
            // Entire buffer is guaranteed to be whole utf8 characters
            buf.iter().for_each(|b| {
                if b != &0 {
                    oldbuf.push(*b)
                }
            });
            res += parse_buf(&mut oldbuf,&mut true,true);
        }
    }

    res
}

fn pt2(path: &str) -> u32 {
    let mut read = 0;
    let mut res = 0;
    let mut oldbuf: Vec<u8> = Vec::with_capacity(BUF_SIZE*2);
    let mut enable_multiply = true;
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            // hprintln!("Reading from file");
            let buf:[u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(), BUF_SIZE);
            // Entire buffer is guaranteed to be whole utf8 characters
            buf.iter().for_each(|b| {
                if b != &0 {
                    oldbuf.push(*b)
                }
            });
            res += parse_buf(&mut oldbuf, &mut enable_multiply,false);
        }
    }

    res
}

#[derive(PartialEq,Debug)]
enum Instr{
    None,
    M,
    U,
    L, // Letter L
    LB, // Left Bracket
    LS(u32), // Left Side
    Com(u32), // Comma
    R((u32,u32)), // Right side -> Both sides really
    D,
    O,
    N,
    APS,
}
fn parse_buf(oldbuf: &mut Vec<u8>,enable_multiply: &mut bool,always_enable:bool) -> u32 {
    let mut res = 0;
    let buf_str = core::str::from_utf8(&oldbuf).unwrap();
    let mut current_instruction = Instr::None;
    let mut partialbuf: Vec<u8> = Vec::with_capacity(10 );
    for c in buf_str.chars(){
        if current_instruction == Instr::O && c != 'n' {partialbuf.clear()}; // Clear out the partial buffer in the event of a "do" that isn't part of a "don't". Must be cleared before pushing a new char in
        partialbuf.push(c as u8);
        match (c, current_instruction) {
            ('m',Instr::None) => current_instruction = Instr::M,
            ('u',Instr::M) => current_instruction = Instr::U,
            ('l',Instr::U) => current_instruction = Instr::L,
            ('(', Instr::L) => current_instruction = Instr::LB,
            (_,Instr::LB) if c.is_digit(10) => current_instruction = Instr::LS(c.to_digit(10).unwrap()),
            (_,Instr::LS(num)) if c.is_digit(10) => current_instruction = Instr::LS(num*10 + c.to_digit(10).unwrap()),
            (_,Instr::Com(num)) if c.is_digit(10) => current_instruction = Instr::R((num,c.to_digit(10).unwrap())),
            (_,Instr::R((ln,rn))) if c.is_digit(10) => current_instruction = Instr::R((ln,rn*10 + c.to_digit(10).unwrap())),
            (_,_) if c.is_digit(10) => {current_instruction = Instr::None; partialbuf.clear()},
            (',',Instr::LS(num)) => current_instruction = Instr::Com(num),
            (',',_) => {current_instruction = Instr::None; partialbuf.clear()},
            (')',Instr::R((ln,rn))) => {res += ln*rn * (*enable_multiply || always_enable) as u32; current_instruction = Instr::None; partialbuf.clear()},
            ('d',Instr::None) => current_instruction = Instr::D,
            ('o',Instr::D) => {*enable_multiply=true; current_instruction = Instr::O},
            ('n',Instr::O) => current_instruction = Instr::N,
            ('\'',Instr::N) => current_instruction = Instr::APS,
            ('t',Instr::APS) => {*enable_multiply = false; current_instruction = Instr::None; partialbuf.clear()}
            _ => {current_instruction = Instr::None; partialbuf.clear()}
        }
    }
    oldbuf.clear();
    oldbuf.append(&mut partialbuf);
    res
}
