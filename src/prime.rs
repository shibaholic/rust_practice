
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn is_prime() {
    loop {
        print!("Enter a number to test if prime or not: ");
        io::stdout().flush().expect("flush should work");
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).expect("read should work");
        input.truncate(input.len()-1);
        let number:u64 = input.parse::<u64>().unwrap_or(2);
        
        let mut multiples:Vec<u64> = Vec::new();
        
        integer_factorization(number, &mut multiples);
        
        if multiples.len() > 2 {
            println!("{} is not a prime.", number);
            
            println!("{} contains the multiples:", number);
            for multiple in multiples {
                print!("{} ", multiple)
            }
            println!("");
        } else {
            println!("{} is a prime.", number);
        }
    }
}

fn brute_force(number:i32, multiples: &mut Vec<i32>) {
    // brute-force, exhaustive check, and finds all multiples if not a prime
    // this is not the fastest way to find all multiples, idk how you would optimize a and b so it runs the fastest
    // instead of o(n^2) checks
    let mut a = number - 1;
    multiples.push(1);
    multiples.push(number);
    loop {
        if a == 1 {
            break;
        }
        let mut b = number - 1;
        loop {
            if b == 1 {
                break;
            }
            if a * b == number {
                multiples.push(a);
                multiples.push(b);
            }
            b -= 1;
        }
        a -= 1;
    }
}

fn trail_division(number:u64, multiples: &mut Vec<u64>) {
    let mut f = 2;
    let mut n = number;
    while f*f <= n {
        if n % f == 0 {
            multiples.push(f);
            n /= f;
        } else {
            f += 1;
        }
    }
    if n != 1 {
        multiples.push(n);
    }
}

fn integer_factorization(n:u64, multiples: &mut Vec<u64>) {
    let n_sqrt = f64::sqrt(n as f64) as u64 + 1;

    let mut factors:HashSet<u64> = HashSet::new();

    for a in 1..n_sqrt {
        if n % a == 0 {
            // a is a factor, and n/a is its factor pair.
            factors.insert(a);
            factors.insert(n/a);
        }
    }
    *multiples = factors.into_iter().collect();
    multiples.sort();
}