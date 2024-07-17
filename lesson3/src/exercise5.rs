// Problem 1: Convert the code based on the combinators

pub fn run() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = 0;

    /* The code in the for loop needs to be replaced */
    result = numbers.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum();

    println!("Result without combinators: {}", result);
}