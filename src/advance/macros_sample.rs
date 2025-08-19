use macros::log;
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn declare_macros_hello_sample() {
    say_hello!(); // 调用宏
}

macro_rules! make_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

fn declare_macros_make_sample() {
    let v = make_vec!(1, 2, 3, 4);
    println!("{:?}", v); // [1, 2, 3, 4]
}

macro_rules! print_value {
    ( $val:expr ) => {
        println!("The value is: {}", $val);
    };
}

fn declare_macros_print_sample() {
    print_value!(42);
    print_value!("hello");
}

#[log]
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn proc_macros_log_sample() {
    say_hello("World");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_declare_macros_hello_sample() {
        declare_macros_hello_sample();
    }

    #[test]
    fn test_declare_macros_make_sample() {
        declare_macros_make_sample();
    }

    #[test]
    fn test_declare_macros_print_sample() {
        declare_macros_print_sample();
    }

    #[test]
    fn test_macros_proc_log() {
        proc_macros_log_sample();
    }
}
