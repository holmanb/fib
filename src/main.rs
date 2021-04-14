/* 
Demonstrates that memoized version of recursive fibonnaci is far faster than a 
naive recursive version. Memoized recursion is slower than iterative approach
due to function call and hashmap lookups.  The recursive version additionally 
takes up more memory for storing memoized values.
*/ 

use std::time::{Instant};
use std::collections::HashMap;

#[allow(dead_code)]
fn naive_recursion(in1: u32) -> u64 {
    if in1 < 2 {
        return 1;
    }
    naive_recursion(in1 - 1) + naive_recursion(in1 - 2)
}

#[allow(dead_code)]
fn naive_recursion_match(in1: u32) -> u64 {
    match in1 {
        0|1 => 1,
        _ => naive_recursion(in1 - 1) + naive_recursion(in1 - 2)
    }
}

fn iterative(in1: u32) -> u64 {
    let (mut n1, mut n2, mut sum, mut i) = (1, 1, 0, 1);
    while i < in1 {
        sum = n1 + n2;
        n1 = n2;
        n2 = sum;
        i+=1;
    }
    sum
}

fn memoized_recursion(in1: u32, h: &mut std::collections::HashMap<u32,u64>) -> u64 {
    let val = h.get(&in1);
    match val {
        Some(i) => {
            return *i;
        },
        None => {
            match in1 {
                0|1 => 1,
                _ => {
                    let ret = memoized_recursion(in1 - 1, h) + memoized_recursion(in1 - 2, h);
                    h.insert(in1, ret);
                    return ret;
                }
            }
        },
    }
}

fn time(f: fn(u32)->u64, arg: u32, s: String){
    let start_sys_time_recursive = Instant::now();
    println!("{}", f(arg));
    println!("{} : {:?} ns", s, 
    start_sys_time_recursive.elapsed().as_nanos());
}

fn main() {

    let times:u32 = 92;
    let mut mem = HashMap::new();
    let start_sys_time_recursive = Instant::now();
    println!("{}", memoized_recursion(times, &mut mem));
    println!("{} : {:?} ns", "memoized recursion", 
    start_sys_time_recursive.elapsed().as_nanos());
    time(iterative, times, String::from("iter"));

    // uncomment next lines to see _really slow_ performance of naive recursive algorithms
    //time(naive_recursion, times, String::from("naive recursion if/else"));
    //time(naive_recursion_match, times, String::from("naive recursion match"));
}