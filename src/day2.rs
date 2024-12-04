
use crate::BUF_SIZE;
use cortex_m::iprintln;
use cortex_m_semihosting::syscall;

extern crate alloc;
use alloc::vec::Vec;

pub fn run(stim: &mut cortex_m::peripheral::itm::Stim) {
    let sample_path = r"C:\Users\Mike_\Documents\Rust\aoc24\sample_data\day2.txt";
    let real_path = r"C:\Users\Mike_\Documents\Rust\aoc24\real_data\day2.txt";

    {
        let sample_result = read_from_system(sample_path);
        iprintln!(stim,"Sample result for part 1: {}",sample_result);
        if sample_result != 2 {
            iprintln!(stim,"Sample result for part 1 is not equal to the target of 2");
            panic!()
        }

        let sample_p2 = read_from_system2(sample_path);
        iprintln!(stim, "Sample result for part 2: {}",sample_p2);
        if sample_p2 != 4 {
            iprintln!(stim, "Sample result for part 2 is not equal to the target of 4");
            panic!()
        }
    }

    // let pt1_res = read_from_system(real_path);
    // iprintln!(stim,"Part 1 result: {}",pt1_res);
    let p2_res = read_from_system2(real_path);
    iprintln!(stim,"Part 2 result: {}",p2_res);
}

fn diff(line: &Vec<u8>) -> Vec<i8> {
    line.windows(2).map(|w| {
        w[1].checked_signed_diff(w[0]).unwrap()
    }).collect::<Vec<i8>>()
}

fn is_safe(line: &Vec<u8>) -> bool {
    let d = diff(&line);
    orig_safe(&d)
}

fn orig_safe(diff: &Vec<i8>) -> bool {
    let all_pos = diff.iter().all(|i| *i > 0);
    let all_neg = diff.iter().all(|i| *i < 0);
    let all_in_tol = diff.iter().all(|i| i.abs() <= 3);

    (all_pos || all_neg) && all_in_tol
}
 fn is_safe_2(line:Vec<u8>) -> bool {
    let d = diff(&line);
    // If it was already safe it's still safe
    if orig_safe(&d) {return true};
    // If all the directions are consistent and the tolerance is out, then it's never safe
    // This isn't true - if the last or first values are the ones breaking the tolerance then it can be safe
    // Don't need to actually check the tolerance because if the tolerance were okay it would have already returned true
    if d.iter().all(|i| *i > 0) || d.iter().all(|i| *i < 0) {
        let mut t1 = line.clone();
        let mut t2 = line.clone();
        t1.pop();
        t2.remove(0);
        let t1_safe = is_safe(&t1);
        let t2_safe = is_safe(&t2);
        return t1_safe || t2_safe;
    };
    // If two numbers are the same, try removing one of them
    if let Some(i) = d.iter().position(|i| i == &0) {
        let mut t1 = line.clone();
        t1.remove(i);
        let t1_safe = is_safe(&t1);
        return t1_safe;
    }
    // If one direction is inconsistent, remove it and check against the original safety func
    // Beware edge cases - which way do you remove? Try both and check?
    let num_pos: usize = d.iter().fold(0, |acc, n| {
        if n > &0 {
            acc + 1
        } else {
            acc
        }
    });
    let num_neg: usize = d.iter().fold(0, |acc,n| {
        if n < &0 {
            acc + 1
        } else {
            acc
        }
    });

    // If there are more than one instance of both directions - it's not safe
    if num_pos > 1 && num_neg > 1 { return false };

    if num_pos > num_neg {
        // Overall positive, find the negative direction, remove it, check against the original safety
        if let Some(pos) = d.iter().position(|x| x < &0) {
            let mut t1 = line.clone();
            let mut t2 = line.clone();
            t1.remove(pos);
            t2.remove(pos+1);
            let t1_safe = is_safe(&t1);
            let t2_safe = is_safe(&t2);
            return t1_safe || t2_safe;
        }
    } else {
        if let Some(pos) = d.iter().position(|x| x > &0) {
            let mut t1 = line.clone();
            let mut t2 = line.clone();
            t1.remove(pos);
            t2.remove(pos+1);
            let t1_safe = is_safe(&t1);
            let t2_safe = is_safe(&t2);
            return t1_safe || t2_safe;
        }
    }

    false
}

fn read_from_system(path: &str) -> usize {
    let mut read = 0;
    let mut res = 0;
    let mut report_buf: Vec<u8> = Vec::with_capacity(8);
    let mut num_buf:Vec<u8> = Vec::with_capacity(3);
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(), BUF_SIZE);
            for c in buf {
                if c == b'\n' {
                    // Process Report into report buffer
                    let num = core::str::from_utf8(&num_buf).unwrap().parse::<u8>().unwrap();
                    num_buf.clear();
                    report_buf.push(num);
                    let r = report_buf.clone();
                    if is_safe(&r) {
                        res +=1;
                    }
                    report_buf.clear();
                } else if c.is_ascii_whitespace() {
                    // Process number into report
                    let num = core::str::from_utf8(&num_buf).unwrap().parse::<u8>().unwrap();
                    num_buf.clear();
                    report_buf.push(num);
                } else if c != 0 {
                    // Add to buf
                    num_buf.push(c);
                }
            }
        }
        syscall!(CLOSE,fh);
    }
    res
}

fn read_from_system2(path: &str) -> usize {
    let mut read = 0;
    let mut res = 0;
    let mut report_buf: Vec<u8> = Vec::with_capacity(8);
    let mut num_buf:Vec<u8> = Vec::with_capacity(3);
    unsafe {
        let fh = syscall!(OPEN,path.as_ptr(),0,path.len());
        while read < BUF_SIZE {
            let buf: [u8;BUF_SIZE] = [0;BUF_SIZE];
            read = syscall!(READ,fh,buf.as_ptr(), BUF_SIZE);
            for c in buf {
                if c == b'\n' {
                    // Process Report into report buffer
                    let num = core::str::from_utf8(&num_buf).unwrap().parse::<u8>().unwrap();
                    num_buf.clear();
                    report_buf.push(num);
                    let r = report_buf.clone();
                    if is_safe_2(r) {
                        res +=1;
                    }
                    report_buf.clear();
                } else if c.is_ascii_whitespace() {
                    // Process number into report
                    let num = core::str::from_utf8(&num_buf).unwrap().parse::<u8>().unwrap();
                    num_buf.clear();
                    report_buf.push(num);
                } else if c != 0 {
                    // Add to buf
                    num_buf.push(c);
                }
            }
        }
        syscall!(CLOSE,fh);
    }
    res
}