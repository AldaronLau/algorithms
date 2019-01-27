#![allow(unused)]

const SECOND: u128 = 1000000;
const MINUTE: u128 = SECOND * 60;
const HOUR: u128 = MINUTE * 60;
const DAY: u128 = HOUR * 24;
const MONTH: u128 = DAY * 30;
const YEAR: u128 = MONTH * 365;
const CENTURY: u128 = YEAR * 100;

fn factorial() -> u128 {
    let limit = CENTURY; // how much time a factorial algorithm has to run.
    let mut i = 1;
    let mut x = 1;
    loop {
        i += 1;
        let prev = x;
        x *= i;
        if x > limit {
            return i - 1;
        }
    }
}

// * limit is how much time a factorial algorithm has to run.
fn pow(limit: u128) -> u128 {
    let mut i = 0;
    let mut x = 1;
    loop {
        i += 1;
        let prev = x;
        x *= 2;
        if x > limit {
            return i - 1;
        }
    }
}

fn root(limit: u128) -> u128 {
    let mut x = 0u128;
    // x cubed
    loop {
        x += 1;
        let c = x.pow(2);
        if c > limit {
            return x - 1;
        }
    }
}

fn lg(mut x: u128) -> u128 {
    let mut r = 0u128;
    loop {
        x >>= 1;
        r += 1;
        if x == 0 {
            return r;
        }
    }
}

fn nlgn(limit: u128) -> u128 {
    let mut x = 21024009999999u128;
    loop {
        x += 100000000;
        let c = x * lg(x);
        if c > limit {
            return x - 1;
        }
    }
}

fn main() {
//    println!("factorial: {}", factorial());
    println!("SECOND: {}", nlgn(SECOND));
    println!("MINUTE: {}", nlgn(MINUTE));
    println!("HOUR: {}", nlgn(HOUR));
    println!("DAY: {}", nlgn(DAY));
    println!("MONTH: {}", nlgn(MONTH));
    println!("YEAR: {}", nlgn(YEAR));
    println!("CENTURY: {}", nlgn(CENTURY));
}
