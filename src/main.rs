use std::{time::Duration, thread};
use cacher::cacher::Cacher;


fn expensive_function(argument: usize) -> u32 {
    thread::sleep(Duration::from_millis(2000));
    argument as u32
} 

fn main() {
    
    let mut cached_function = Cacher::new(expensive_function);

    println!("Running calculation..");
    let result = cached_function.value(2);
    println!("Result is {}", result);

    println!("Running calculation again..");
    let result = cached_function.value(2);
    println!("Result is {}", result);

    println!("Running different calculation..");
    let result = cached_function.value(3);
    println!("Result is {}", result);

}