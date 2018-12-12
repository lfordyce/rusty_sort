extern crate euler_sum;

use euler_sum::{SelectSortable, Fibonacci};
use std::env;
use std::process;

macro_rules! yo {
    ($name:expr) => {
        println!("Yo {}!", $name);
    };
}

#[derive(Debug)]
struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

fn main() {

    let mut new_v = Vec::new();

    for i in 0..10 {
        new_v.push(i + 1);
        println!("{:?}", new_v);
    }

    while new_v.len() > 1 {
        new_v.pop();
        println!("{:?}", new_v);
    }

    let mut pairs = Vec::new();
    for key in ["A", "B","C", "D", "E"].iter() {
        pairs.push(Pair{key, value: "even"});
        pairs.push(Pair{key, value: "odd"});
    }
    println!("{:?}", pairs);
    for (i, pair) in (&pairs).iter().enumerate() {
        println!("{}: key={} value={}",i, pair.key, pair.value);
    }




    let solution = euler_sum::euler_solution(10);
    println!("Euler Solution: {}", solution);
    yo!("Lee");

    let mut v = vec![3, 2, 1, 8, 5, 7, 6, 4];
    v.selection_sort();
    println!("Selection sort result: {:?}", v);



    let values = vec![1, 2, 3, 4, 5, 6];
    {
        let mut values = (&values).into_iter();
        while let Some(&n) = values.next() {
            println!("{}", n);
        }
    }
    // Borrow scope end

    {
        let mut values = (&values).into_iter();
        while let Some(&n) = values.next() {
            println!("{}", n);
        }
    }
    // Borrow scope end

    let mut values2 = vec![1, 2, 3, 4, 5, 6];
    {
        let mut values2 = (&mut values2).into_iter();
        while let Some(n) = values2.next() {
            *n += 3;
        }
    }
    // Borrow scope end

    {
        let mut values2 = (&mut values2).into_iter();
        while let Some(n) = values2.next() {
            println!("{}", n);
        }
    }
    // Borrow scope end

    let fibo: Vec<_> = Fibonacci::new().take(8).collect();

    for (i, item) in fibo.iter().enumerate() {
        println!("Fibonacci sequence: {} at index: {}", item, i)
    }

    let idx = match env::args().nth(1).and_then(|n| n.parse().ok()) {
        None => {
            println!("Provide a number");
            process::exit(1);
        }

        Some(0) => {
            println!("Value out of range");
            process::exit(2);
        }

        Some(n) => n
    };

    let value = match Fibonacci::new().nth(idx - 1) {
        None => {
            println!("Value out of range");
            process::exit(2);
        }

        Some(n) => n,
    };

    println!("{}", value);
}