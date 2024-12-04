// use crate::BUF_SIZE;
use cortex_m::iprintln;
use cortex_m_semihosting::{syscall,hprintln};

extern crate alloc;
use alloc::vec::Vec;

const BUF_SIZE: usize = 1024;

pub fn run(stim: &mut cortex_m::peripheral::itm::Stim) {
    let sample_path = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day4.txt";
    let real_path = r"C:\Users\Mike_\Documents\Rust\aoc24\real_data\day4.txt";
   {
        let sample_p1 = pt1(sample_path,10);
        iprintln!(stim,"Sample pt1: {:?}",sample_p1);
        if sample_p1 != 18 {
            iprintln!(stim,"Sample p1 does not match target of 18");
            panic!();
        }
        let sample_p2 = pt2(sample_path,10);
        iprintln!(stim,"Sample pt2: {:?}",sample_p2);
        if sample_p2 != 9 {
            iprintln!(stim,"Sample p2 does not match target of 9");
            panic!();
        }
   }

//    let p1 = pt1(real_path,141);
//    iprintln!(stim,"Part 1 result: {}",p1);
    let p2 = pt2(real_path,141);
    iprintln!(stim,"Part 2: {}",p2);

}

fn pt1(path: &str,line_len:usize) -> u32 {
    let mut res = 0;
    let mut read = 0;
    let mut whole_line_buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    let mut partial_line_buf: Vec<u8> = Vec::with_capacity(line_len);
    let mut find_next_line = Vec::with_capacity(512);
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf:[u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            whole_line_buf.clear();
            whole_line_buf.append(&mut partial_line_buf);
            buf.iter().for_each(|b| {
                if b != &0 {
                    whole_line_buf.push(*b);
                }
            });
            let mut last_newline:usize = 0;
            whole_line_buf.iter().enumerate().for_each(|(i,b)| {
                if *b == b'\n' {
                    last_newline = i;
                }
            });
            partial_line_buf = whole_line_buf.split_off(last_newline);
            res += parse_buf(&whole_line_buf, &mut find_next_line);
        }
    }
    res
}

fn pt2(path: &str,line_len:usize) -> u32 {
    let mut res = 0;
    let mut read = 0;
    let mut whole_line_buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    let mut partial_line_buf: Vec<u8> = Vec::with_capacity(line_len);
    let mut a_line: [Vec<MAS>;2] = [Vec::with_capacity(140),Vec::with_capacity(140)];
    let mut end_line: [Vec<MAS>;2] = [Vec::with_capacity(140),Vec::with_capacity(140)];
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf:[u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            whole_line_buf.clear();
            whole_line_buf.append(&mut partial_line_buf);
            buf.iter().for_each(|b| {
                if b != &0 {
                    whole_line_buf.push(*b);
                }
            });
            let mut last_newline:usize = 0;
            whole_line_buf.iter().enumerate().for_each(|(i,b)| {
                if *b == b'\n' {
                    last_newline = i;
                }
            });
            partial_line_buf = whole_line_buf.split_off(last_newline);
            res += parse_buf2(&whole_line_buf, &mut a_line, &mut end_line);
        }
    }
    res
}

#[derive(Debug,Clone)]
struct NextLetter{
    pos: usize,
    from: char,
    next: char,
    direction: i8,
}

#[derive(Clone,Copy,Debug)]
struct MAS{
    A: usize,
    FinalLine: ((usize,char),(usize,char))
}

fn parse_buf2(buf: &Vec<u8>, a_line: &mut [Vec<MAS>;2], end_line: &mut [Vec<MAS>;2]) -> u32 {
    // Search the line for starter characters.
    // Pop onto the list to search the locations of the required next charactes (A, then S and M)
    let mut res = 0;
    let buf_str = core::str::from_utf8(buf).unwrap(); 
    let mut in_x = false;
    for line in buf_str.lines() {
        if line.is_empty() {
            continue
        }
        let m_idxs = line.match_indices('M').map(|(idx,_)| idx).collect::<Vec<usize>>();
        //hprintln!("M found at {:?}",m_idxs);
        let s_idxs = line.match_indices('S').map(|(idx,_)| idx).collect::<Vec<usize>>();
        //hprintln!("S found at {:?}",s_idxs);
        for id in m_idxs.iter() {
            //hprintln!("Checking M index {}",id);
            if m_idxs.contains(&(*id+2)) {
                //hprintln!("Pushing MM");
                a_line[1].push(MAS{A: id + 1, FinalLine: ((*id,'S'),(id+2,'S'))});
            }
            if s_idxs.contains(&(*id+2)) {
                //hprintln!("Pushing MS");
                a_line[1].push(MAS{A: id + 1, FinalLine: ((*id,'M'),(id+2,'S'))});
            }
        }
        for id in s_idxs.iter() {
            //hprintln!("Checking S index {}",id);
            if s_idxs.contains(&(*id+2)) {
                //hprintln!("Pushing SS") ;
                a_line[1].push(MAS{A: id + 1, FinalLine: ((*id,'M'),(id+2,'M'))});
            }
            if m_idxs.contains(&(*id+2)) {
                //hprintln!("Pushing SM");
                a_line[1].push(MAS{A: id + 1, FinalLine: ((*id,'S'),(id+2,'M'))});
            }
        }
        //hprintln!("Searching A line: {:?}",a_line[0]);

        for mid in a_line[0].iter() {
            if line.chars().nth(mid.A).unwrap() == 'A' {
                //hprintln!("Found an A line {:?}",mid);
                end_line[1].push(*mid);
            }
        }

        //hprintln!("Searching end line: {:?}",end_line[0]);

        for el in end_line[0].iter() {
            let mut i = line.chars();
            if i.nth(el.FinalLine.0.0).unwrap() == el.FinalLine.0.1 && i.nth(1).unwrap() == el.FinalLine.1.1 {
                //hprintln!("Found a final line: {:?}",el);
                res += 1;
            }
        }
        
        // At the end of the line, clear out each search line, and swap the next line in.
        a_line[0].clear();
        a_line.swap(0,1);
        end_line[0].clear();
        end_line.swap(0,1);
    }

    res
}

fn parse_buf(buf: &Vec<u8>,mut find_next_line: &mut Vec<NextLetter> ) -> u32 {
    let mut res = 0;
    let bufStr = core::str::from_utf8(buf).unwrap();
    let mut find_this_line = Vec::with_capacity(512);
    for line in bufStr.lines() {
        if line.is_empty() {
            continue;
        }
        core::mem::swap(&mut find_this_line,&mut find_next_line);
        find_next_line.clear();
        // Check for in line matches
        res += line.matches("XMAS").count() as u32;
        res += line.matches("SAMX").count() as u32;
        ////hprintln!("Parsing line {}",line);
        for (pos,char) in line.chars().enumerate() {
             match char{
                'X' => generate_new_options((pos,'X'), line.len(),&mut find_next_line),
                'S' => generate_new_options((pos,'S'),line.len(),&mut find_next_line),
                _ => {},
             };
        }
        ////hprintln!("Found the following starter characters: {:?}",next_line);
        for target  in find_this_line.iter() {
            let char_at_pos = line.chars().nth(target.pos);
            if char_at_pos.is_none() {hprintln!("Indexing into incorrect positions in line {} : {}. Target: {:?}",line,target.pos,target);panic!()}
            if char_at_pos.unwrap() == target.next {
                match target.next {
                    'X' => res += 1,
                    'S' => res += 1,
                    'M' => {
                        let next_pos = next_pos_gen(target.pos,target.direction);
                        let next_char = next_char_gen(target.next,target.from);
                        find_next_line.push(NextLetter{pos:next_pos,from:target.from,next:next_char,direction:target.direction});
                    },
                    'A' => {
                        let next_pos = next_pos_gen(target.pos,target.direction);
                        let next_char = next_char_gen(target.next,target.from);
                        find_next_line.push(NextLetter{pos:next_pos,from:target.from,next:next_char,direction:target.direction});
                    }
                    _ => {hprintln!("Unexpected match: {:?}",target.next);panic!()}
                }
            }
        }
    }
    res
}

fn generate_new_options(found: (usize,char),linelen: usize, find_next_line: &mut Vec<NextLetter>) {
    match found {
        (_,'X') => {
            find_next_line.push(NextLetter{pos:found.0, from: 'X', direction:0,next:'M'});
            if found.0 > 2 { // Can only do a diagonal backwards if there's space to fit it
            find_next_line.push(NextLetter{pos:found.0-1,from: 'X', direction:-1,next:'M'});
            }
            if found.0 < linelen - 3 { // Can only do a diagonal forward if there's space to fit it
            find_next_line.push(NextLetter{pos:found.0+1,from:'X',direction:1,next:'M'});
            }
        }
        (_,'S') => {
            find_next_line.push(NextLetter{pos:found.0, from: 'S', direction:0,next:'A'});
            if found.0 > 2 { // Can only do a diagonal backwards if there's space to fit it
            find_next_line.push(NextLetter{pos:found.0-1,from: 'S', direction:-1,next:'A'});
            }
            if found.0 < linelen - 3 { // Can only do a diagonal forward if there's space to fit it
            find_next_line.push(NextLetter{pos:found.0+1,from:'S',direction:1,next:'A'});
            }
        }
        _ => {}
    }
}

fn next_pos_gen(pos:usize,direction:i8) ->usize {
    match direction {
        -1 => pos -1,
        0 => pos,
        1 => pos +1,
        _ => {hprintln!("Unexpected direction {}",direction);panic!()}
    }
}

fn next_char_gen(target:char,from:char) -> char {
    match (target,from) {
        ('M','X') => 'A',
        ('M','S') => 'X',
        ('A','X') => 'S',
        ('A','S') => 'M',
        _ => {hprintln!("Unexpected target/from pair {:?}",(target,from));panic!()}
    }
}