fn main() {
    println!("Hello, world!");
    
    // create variable
    let name = "Rust";
    println!("Hello, {}", name);
    
    let test = add(1, 2);
    println!("Add: {}", test);
    
    // create mutable variable
    let mut age = 20;
    println!("Age: {}", age);
    age = 21;
    println!("Age: {}", age);
    let add = |a: i32, b: i32| a + b;
    println!("Add: {}", add(1, 2));
    
    // create if else statement
    let is_true = true;
    if is_true {
        println!("True");
    } else {
        println!("False");
    }
    
    // create loop
    let mut i = 0;
    loop {
        println!("Loop: {}", i);
        i += 1;
        if i == 5 {
            break;
        }
    }
    
    // create while loop
    let mut j = 0;
    while j < 5 {
        println!("While: {}", j);
        j += 1;
    }
    
    
}

// create simple function for testing rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// create simple test for add function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

