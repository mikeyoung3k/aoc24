use crate::BUF_SIZE;
use cortex_m::iprintln;
use cortex_m_semihosting::syscall;

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::VecDeque;


pub fn run(stim: &mut cortex_m::peripheral::itm::Stim) {
    let sample_path = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day1.txt";
    let real_path = r"C:\Users\Mike_\Documents\Rust\aoc24\real_data\day1.txt";
    {
        let sample_p1_in = read_from_system(sample_path,8);
        iprintln!(stim,"Sample input: {:?}",sample_p1_in);
        let sample_p1_res = pt1(sample_p1_in);
        iprintln!(stim, "P1 res: {:?}", sample_p1_res);
        if sample_p1_res != 11 {
            iprintln!(stim, "Sample input for part 1 does not match target of 11!");
            panic!();
        }
    }
    {
        let sample_p2_in = read_from_system(sample_path,8);
        let sample_p2_res = pt2(sample_p2_in);
        iprintln!(stim,"Pt2 res: {:?}", sample_p2_res);
        if sample_p2_res != 31 {
            iprintln!(stim, "Sample result for part 2 does not match target of 31!");
            panic!();
        }
    }

    let real_data= read_from_system(real_path,1002);
    // let p1 = pt1(real_data);
    // iprintln!(stim,"Day 1 pt 1: {}",p1);
    let p2 = pt2(real_data);
    iprintln!(stim,"Day 1 pt 2: {}",p2);

}
    
fn pt1(input: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = input.0;
    let mut right = input.1;
    //hprintln!("Beginning to sort left vec");
    left.sort();
    //hprintln!("Beginning to sort right vec");
    right.sort();
    left.iter().zip(right.iter()).map(|(l,r)| l.abs_diff(*r)).sum()
}

fn pt2(input: (Vec<u32>,Vec<u32>)) -> u32 {
    let mut left = input.0;
    let mut right = input.1;
    left.sort();
    right.sort();
    left.iter().map(|n| {
        let v = right.iter().fold(0, |acc,r| {
            if r == n {
                acc + 1
            } else {
                acc
            }
        });
        n*v
    }).sum()
}

fn read_from_system(path: &str,capacity:usize) -> (Vec<u32>,Vec<u32>) {
    let mut read = 0;
    let mut left: Vec<u32> = Vec::with_capacity(capacity);
    let mut right: Vec<u32> = Vec::with_capacity(capacity);
    //hprintln!("Initialized two vectors of capacity {}",capacity);
    let mut line_buffer:VecDeque<u8> = VecDeque::with_capacity(BUF_SIZE*2);
    //hprintln!("Initialized vecdeque of capacity {}",BUF_SIZE*2);
    // Read into buffer, scan for double newline, then parse the previous items and sum them into a single value
    unsafe  {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ, fh, buf.as_ptr(), BUF_SIZE);
            // //hprintln!("Read: {}, Buffer: {:?}",read,core::str::from_utf8(&buf).unwrap());
            let (mut l,mut r) = parse_buf(buf, &mut line_buffer);
            //hprintln!("Parsed data and got vecs of length {}", l.len());
            left.append(&mut l);
            right.append(&mut r);
        }

        syscall!(CLOSE,fh);
    }

    return (left,right)
}

fn parse_buf(buf: [u8; BUF_SIZE], line_buffer: &mut VecDeque<u8>) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    //hprintln!("Initialized two empty vecs for temporary assignment");
    
    buf.iter().for_each(|c| {{
        if c != &0 {
            line_buffer.push_back(*c);
        }
    }});

    //hprintln!("Moved all of buffer into line_buffer");

    let mut single_line: Vec<u8> = Vec::with_capacity(50);
    //hprintln!("Initialized single line vec with capacity 50 bytes");
    while let Some(c) = line_buffer.pop_front() {
        if c == '\n' as u8 {
            let line_str = core::str::from_utf8(&single_line).unwrap();
            //hprintln!("Parsing line: {}", line_str);
            let mut line_parts = line_str.split_whitespace();
            left.push(line_parts.next().unwrap().parse::<u32>().unwrap());
            right.push(line_parts.next().unwrap().parse::<u32>().unwrap());
            single_line.clear();
        } else {
            single_line.push(c)
        }
    }
    line_buffer.clear();
    //hprintln!("Cleared line buffer");
    single_line.iter().for_each(|c| line_buffer.push_back(*c));

    return (left,right)

}