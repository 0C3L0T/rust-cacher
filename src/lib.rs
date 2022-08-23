pub mod cacher {
    use std::collections::HashMap;

    // memoization pattern
    pub struct Cacher<T, U: Copy>
        where T: Fn(usize) -> U
    {
        calculation: T,
        hash_map: HashMap<usize, U>,
    }

    impl<T, U: Copy> Cacher<T, U>
        where T: Fn(usize) -> U
    {
        pub fn new(calculation: T) -> Cacher<T, U> {
            Cacher {
                calculation,
                hash_map: HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: usize) -> U {
            match self.hash_map.get(&arg) {
                Some(value) => {
                    // index has value
                    *value
                },
                None => {
                    // index has no value
                    // return calculation and add to hash table
                    let value = (self.calculation)(arg);
                    self.hash_map.insert(arg, value);

                    value
                }
            }
        }
    }
}
