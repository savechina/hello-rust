fn closure_sample() {
    // Example 1: Basic closure and Fn trait usage

    let add_one = |x: i32| x + 1; // A closure that takes an i32 and returns an i32

    let result = add_one(5);
    println!("Result: {}", result); // Output: Result: 6

    // Example 2: Closure as a function parameter using Fn

    fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32, // F must implement Fn(i32) -> i32
    {
        f(value)
    }

    let double = |x| x * 2;
    let doubled_result = apply(double, 10);
    println!("Doubled Result: {}", doubled_result); // Output: Doubled Result: 20

    //Example 3: Capturing environment variables and Fn.
    let captured_value = 10;
    let add_captured = |x: i32| x + captured_value;

    let captured_result = add_captured(5);
    println!("Captured Result: {}", captured_result); // Output: Captured Result: 15

    fn apply_captured<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(value)
    }

    let captured_result_applied = apply_captured(add_captured, 20);
    println!("Captured Result Applied: {}", captured_result_applied); //Output: Captured Result Applied: 30

    //Example 4: Using a closure that returns a different type.

    let stringify = |num: i32| format!("Number: {}", num);

    fn process_and_print<F>(f: F, value: i32)
    where
        F: Fn(i32) -> String,
    {
        let result_string = f(value);
        println!("{}", result_string);
    }

    process_and_print(stringify, 42); // Output: Number: 42

    //Example 5: Fn, FnMut, and FnOnce distinctions
    let mut mutable_value = 0;

    println!("Before increment: {}", mutable_value); //0

    let mut increment = |x: i32| {
        mutable_value += x;
        mutable_value
    };

    let incr_result = increment(5);
    println!("Incremented value: {}", incr_result); //5
    println!("After increment: {}", &incr_result); //5

    fn apply_mut<F>(mut f: F, value: i32) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        f(value)
    }

    let num = 0;
    apply_mut(&mut increment, 10);
    println!("After apply_mut: {}", num); //10

    //Example 6: Using FnOnce
    let owned_value = String::from("Owned");
    let consume_string = move || owned_value;

    fn apply_once<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }

    let consumed_string = apply_once(consume_string);
    println!("Consumed string: {}", consumed_string); //Owned

    //Note that consume_string can only be called once, after it has been called, it has moved out of scope.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_sample() {
        closure_sample();
    }
}
