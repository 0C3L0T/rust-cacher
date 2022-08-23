use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// memoization pattern
// a struct that holds a closure and its result
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    hash_map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            hash_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.hash_map.get(&arg) {
            Some(value) => {
                // index has value
                *value
            },
            None => {
                // return calculation and add to hash table
                let value = (self.calculation)(arg);
                self.hash_map.insert(arg, value);

                value
            }
        }
    }
}

fn expensive_function(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

fn main() {
    let mut cached_function = Cacher::new(expensive_function);

    println!("Doing slow calculation..");
    let result1 = cached_function.value(1);
    println!("Result is {}", result1);

    println!("Doing same calculation..");
    let result2 = cached_function.value(1);
    println!("Result is {}", result2);

    println!("Doing different calculation..");
    let result3 = cached_function.value(2);
    println!("Result is {}", result3);
}
