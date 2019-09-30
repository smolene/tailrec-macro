#![allow(dead_code)]

use tailrec::{tailrec};

fn main() {
    println!("sum: {}", sum(100));
    println!("fib: {}", fib(50));
    let mut vec = vec![];
    repeat(1000_000, || { vec.push(()) });
    println!("len: {}", vec.len());
    let test = vec!["((()))", "()(", ")))(((", "(Fg)", "(()())", "(()(()))())(()"];
    for test in test {
        println!("balance: {} | {}", balance(&mut test.chars()), test);
    }

    let n = 1000_000;
    let mut iter = std::iter::repeat("()").flat_map(|s| s.chars()).take(n);
    println!("balance: {} | {}", balance(&mut iter), n);
}

fn balance(list: &mut impl Iterator<Item=char>) -> bool {
    #[tailrec]
    fn f(list: &mut impl Iterator<Item=char>, counter: i32) -> bool {
        if counter < 0 {
            false
        } else if let Some(c) = list.next() {
            match c {
                '(' => return f(list, counter + 1),
                ')' => return f(list, counter - 1),
                _ => return f(list, counter),
            }
        } else {
            counter == 0
        }
    }

    f(list, 0)
}


#[tailrec]
fn repeat<F: FnMut()>(n: usize, f: F) {
    if n != 0 {
        f();
        return repeat(n - 1, f);
    }
}

fn repeat_<F: FnMut()>(n: usize, f: F) {
    if n != 0 {
        f();
        return repeat_(n - 1, f);
    }
}

fn sum(n: u64) -> u64 {
    #[tailrec]
    fn f(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            return f(n - 1, acc + n)
        }
    }

    fn h(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            return h(n - 1, acc + n)
        }
    }

    fn g(n: u64, acc: u64) -> u64 {
        let mut n = n; //
        let mut acc = acc; //
        loop { //
            let res = if n == 0 {
                acc
            } else {
                /*let new_n; //
                let new_acc; //

                new_n = n - 1;
                new_acc = acc + n;

                n = new_n; //
                acc = new_acc; //
                continue;// */

                let args = (n - 1, acc + n);
                n = args.0;
                acc = args.1;
                continue
            };
            return res
        }
    }

    let (rf, rg) = (f(n, 0), g(n, 0));
    assert_eq!(rf, rg);
    rf
}

fn fib(n: u64) -> u64 {
    #[tailrec]
    fn f(n: u64, a: u64, b: u64) -> u64 {
        if n == 0 {
            b
        } else {
            return f(n - 1, b, a + b)
        }
    }

    fn g(n: u64, a: u64, b: u64) -> u64 {
        let mut n = n;
        let mut a = a;
        let mut b = b;
        loop {
            let res = if n == 0 {
                b
            } else {
                {
                    let new_n;
                    let new_a;
                    let new_b;

                    new_n = n - 1;
                    new_a = b;
                    new_b = a + b;

                    n = new_n;
                    a = new_a;
                    b = new_b;
                    continue;
                }
            };
            return res;
        }
    }

    let (rf, rg) = (f(n, 1, 1), g(n, 1, 1));
    assert_eq!(rf, rg);
    rf
}
