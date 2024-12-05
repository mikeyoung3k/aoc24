// use crate::BUF_SIZE;
use cortex_m::iprintln;
use cortex_m_semihosting::{syscall,hprintln};
use core::cmp::Ordering;

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use core::iter::FromIterator;

const BUF_SIZE: usize = 1024;

pub fn run(stim: &mut cortex_m::peripheral::itm::Stim) {
    let sample_path = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day5.txt";
    let real_path = r"C:\Users\Mike_\Documents\Rust\aoc24\real_data\day5.txt";
    {
        let sample_1 = pt1(sample_path);
        iprintln!(stim,"Sample pt1 {}",sample_1);
        if sample_1 != 143 {
            iprintln!(stim,"Sample pt1 does not match target of 143");
            panic!();
        }
        let sample_2 = pt2(sample_path);
        iprintln!(stim, "Sample pt2 {}",sample_2);
        if sample_2 != 123 {
            iprintln!(stim,"Sample pt2 does not match target of 123");
            panic!();
        }
    }
    // let p1 = pt1(real_path);
    // iprintln!(stim,"Pt1 {}",p1);
    let p2 = pt2(real_path);
    iprintln!(stim,"Pt2 {}",p2);
}

fn pt1(path: &str) -> u32 {
    // Build a map of rules
    // For each line, check the map of rules.
    // Can probably hold all rules in memory, but not all lines
    let mut read = 0;
    let mut rules_buf: Vec<u8> = Vec::with_capacity(6000);
    let mut partial_line_buf: Vec<u8> = Vec::with_capacity(128);
    let mut whole_line_buf:Vec<u8> = Vec::with_capacity(128);
    let mut newline_count = 0;
    let mut is_rule = true;
    let mut res = 0;
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while is_rule {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            for b in buf.iter() {
                if *b == b'\n' {
                    newline_count += 1;
                } else {
                    newline_count = 0;
                } 
                if newline_count == 2 {
                    is_rule = false
                }
                if b != &0 && is_rule {
                    rules_buf.push(*b);
                } else if b != &0 {
                    whole_line_buf.push(*b);
                }
            }
        }
        // hprintln!("Left over initial lines: {:?}",core::str::from_utf8(&whole_line_buf).unwrap());
        let rules_map = lines_to_rules(rules_buf);
        while read < BUF_SIZE {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            partial_line_buf.iter().for_each(|b| whole_line_buf.push(*b));
            partial_line_buf.clear();
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            buf.iter().for_each(|b| {
                if b != &0 {
                    whole_line_buf.push(*b);
                }
            });
            let mut last_newline = 0;
            whole_line_buf.iter().enumerate().for_each(|(i,b)| {
                if *b == b'\n' {
                    last_newline = i;
                }
            });
            // hprintln!("Whole line buf: {:?}",core::str::from_utf8(&whole_line_buf).unwrap());
            partial_line_buf = whole_line_buf.split_off(last_newline);
            // hprintln!("Partial line buf: {:?}",core::str::from_utf8(&partial_line_buf).unwrap());
            res += parse_lines(&rules_map,&whole_line_buf);
            whole_line_buf.clear();
        }
    }
    res
}

fn pt2(path: &str) -> u32 {
    // Build a map of rules
    // For each line, check the map of rules.
    // Can probably hold all rules in memory, but not all lines
    let mut read = 0;
    let mut rules_buf: Vec<u8> = Vec::with_capacity(6000);
    let mut partial_line_buf: Vec<u8> = Vec::with_capacity(128);
    let mut whole_line_buf:Vec<u8> = Vec::with_capacity(128);
    let mut newline_count = 0;
    let mut is_rule = true;
    let mut res = 0;
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while is_rule {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            for b in buf.iter() {
                if *b == b'\n' {
                    newline_count += 1;
                } else {
                    newline_count = 0;
                } 
                if newline_count == 2 {
                    is_rule = false
                }
                if b != &0 && is_rule {
                    rules_buf.push(*b);
                } else if b != &0 {
                    whole_line_buf.push(*b);
                }
            }
        }
        // hprintln!("Left over initial lines: {:?}",core::str::from_utf8(&whole_line_buf).unwrap());
        let rules_map = lines_to_rules(rules_buf);
        while read < BUF_SIZE {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            partial_line_buf.iter().for_each(|b| whole_line_buf.push(*b));
            partial_line_buf.clear();
            read = syscall!(READ,fh,buf.as_ptr(),BUF_SIZE);
            buf.iter().for_each(|b| {
                if b != &0 {
                    whole_line_buf.push(*b);
                }
            });
            let mut last_newline = 0;
            whole_line_buf.iter().enumerate().for_each(|(i,b)| {
                if *b == b'\n' {
                    last_newline = i;
                }
            });
            // hprintln!("Whole line buf: {:?}",core::str::from_utf8(&whole_line_buf).unwrap());
            partial_line_buf = whole_line_buf.split_off(last_newline);
            // hprintln!("Partial line buf: {:?}",core::str::from_utf8(&partial_line_buf).unwrap());
            res += correct_lines(&rules_map,&whole_line_buf);
            whole_line_buf.clear();
        }
    }
    res
}

fn correct_lines(rules_map: &BTreeMap<u8,Vec<u8>>,whole_line_buf: &Vec<u8>) -> u32 {
    let mut res: u32 = 0;
    let buf_str = core::str::from_utf8(&whole_line_buf).unwrap();
    for line in buf_str.lines() {
        if line.is_empty() {
            continue
        }
        let mut nums = line.split(',').map(|n| {
            if let Ok(num) = n.parse::<u8>() {
                num
            } else {
                hprintln!("Failed to parse number: {}, in line {}", n, line);
                panic!();
            }
        }).collect::<Vec<u8>>();
        let line_map: BTreeMap<u8,usize> = BTreeMap::from_iter(nums.iter().enumerate().map(|(i,n)| (*n,i)));
        let mut valid = false;
        let valid_line = nums.iter().all(|n| {
            let mut valid_num = true;
            if let Some(v) = rules_map.get(n) {
                // v is a vec of all the numbers that must come after n if they exist
                let n_i = line_map.get(n).unwrap();
                for m in v.iter() {
                    if let Some(j) = line_map.get(m) {
                        if j < n_i {
                            valid_num = false;
                        }
                    }
                }
            }
            valid_num
        });
        if !valid_line {
            res += order_correction(&rules_map,&mut nums);
        }
    }

    res
}

fn order_correction(rule_map: &BTreeMap<u8,Vec<u8>>, nums: &mut Vec<u8>) -> u32 {
    // hprintln!("Numbers: {:?}", nums);
    // hprintln!("Rules map: {:?}",rule_map);
    nums.sort_by(|a,b| {
        if let Some(rules) = rule_map.get(a) {
            if rules.contains(b) {
                return Ordering::Less;
            }
        }

        if let Some(rules) = rule_map.get(b) {
            if rules.contains(a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
    nums[nums.len()/2] as u32
}

fn parse_lines(rules_map: &BTreeMap<u8,Vec<u8>>,whole_line_buf: &Vec<u8>) -> u32 {
    let mut res: u32 = 0;
    let buf_str = core::str::from_utf8(&whole_line_buf).unwrap();
    for line in buf_str.lines() {
        if line.is_empty() {
            continue
        } 
        let nums = line.split(',').map(|n| {
            if let Ok(num) = n.parse::<u8>() {
                num
            } else {
                hprintln!("Failed to parse number: {}, in line {}", n, line);
                panic!();
            }
        }).collect::<Vec<u8>>();
        let line_map: BTreeMap<u8,usize> = BTreeMap::from_iter(nums.iter().enumerate().map(|(i,n)| (*n,i)));
        let mut valid = false;
        let valid_line = nums.iter().all(|n| {
            let mut valid_num = true;
            if let Some(v) = rules_map.get(n) {
                // v is a vec of all the numbers that must come after n if they exist
                let n_i = line_map.get(n).unwrap();
                for m in v.iter() {
                    if let Some(j) = line_map.get(m) {
                        if j < n_i {
                            valid_num = false;
                        }
                    }
                }
            }
            valid_num
        });
        if valid_line {
            res += nums[nums.len()/2] as u32;
        }
    }

    res
}

fn lines_to_rules(buf: Vec<u8>) -> BTreeMap<u8,Vec<u8>> {
    let mut map = BTreeMap::new();
    let mystr = core::str::from_utf8(&buf).unwrap();
    for line in mystr.lines() {
        let mut splt = line.split('|');
        let left = splt.next().unwrap().parse::<u8>().unwrap();
        let right = splt.next().unwrap().parse::<u8>().unwrap();
        let e = map.entry(left).or_insert(Vec::new());
        e.push(right);
    }
    map
}