#![allow(non_snake_case)]

// fibonacci sequence with recursion (using match)
fn fib(number: i32) -> i32 {
    match number {
        0 => 1,
        1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}

// dynamic programming recursive
pub fn dynamic_programming_recursive_fibonacci(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

// bottom up approach 
fn bottom_up_approach(n:usize) -> usize {
    match n {
        1 | 2 => 1,
        _ => {
            let mut bottom_up: Vec<usize> = Vec::new();
            bottom_up.push(1);
            bottom_up.push(1);

            for i in 2..= n {
                bottom_up.push(bottom_up.get(i - 1).unwrap().clone() + bottom_up.get(i - 2).unwrap().clone())
            }

            bottom_up.get(n).unwrap().clone()
        }
    }
}

fn main() {
    println!("Hello, world!");

    // user input 
    let mut fibNumberString: String = String::new();
    std::io::stdin().read_line(&mut fibNumberString).expect("Failed to read line");

    // parsing from string to i32 and handling error
    let fibNumber: i32 = match fibNumberString.trim().parse() {
        Ok(fibNumber) => fibNumber,
        Err(_) => 0,
    };

    // recursive approach
    println!("The fibonacci sequence with recursion of {} is: {} ",fibNumber,  fib(fibNumber));

    // dynamic programming approach
    println!("The fibonacci sequence with dynamic programming of {} is: {} ",fibNumber,  dynamic_programming_recursive_fibonacci(std::convert::TryInto::try_into(fibNumber).unwrap()));

    // bottom up approach
    println!("The fibonacci sequence with bottom up of {} is: {} ",fibNumber,  bottom_up_approach(std::convert::TryInto::try_into(fibNumber).unwrap()));
}
