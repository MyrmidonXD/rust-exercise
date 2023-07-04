#![allow(dead_code)]

/* FizzBuzz implementation using Option type and functional approach
 * 
 * This is quite easy (relative to the HOF one) to implement,
 * and I think this is much better for anyone to understand the code.
 */

// A tuple type that holds a divisor and a corresponding string.
struct Condition<'a>(u32, &'a str);

impl<'a> Condition<'a> {
    fn test(&self, n: u32) -> Option<String> {
        if n % self.0 == 0 { Some(self.1.to_owned()) } else { None }
    }
}

// A trait for adding a functionality to the Option type, which I do not own.
// Rust does not allow directly adding a method to the type 
// from the outside of the crate in which the target type resides.
// i.e. I can't add `merge(...)` into the `impl<T> Option<T> {...}` block from here.
trait OptionExt<T> {
    fn merge<F>(self, other: Self, merger: F) -> Self where F: FnOnce(T, T) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn merge<F>(self, other: Option<T>, merger: F) -> Option<T> where F: FnOnce(T, T) -> T {
        match (self, other) {
            (Some(a), Some(b)) => Some(merger(a, b)),
            (x, y) => x.or(y)
        }
    }
}

// The FizzBuzz problem
fn fizzbuzz(n: u32) -> String {
    let conditions = IntoIterator::into_iter([
        Condition(3, "Fizz"),
        Condition(5, "Buzz"),
    ]);

    conditions
        .map(|cond| cond.test(n))
        .reduce(|a, b| a.merge(b, |s1, s2| s1 + &s2)).unwrap()
        .unwrap_or(n.to_string())
}

fn main() {
    for i in 1..=50 {
        println!("{}", fizzbuzz(i));
    }
}

// Extended version of the FizzBuzz problem
fn fizzbuzzhisshowl(n: u32) -> String {
    let conditions = IntoIterator::into_iter([
        Condition(3, "Fizz"),
        Condition(5, "Buzz"),
        Condition(7, "Hiss"),
        Condition(11, "Howl"),
    ]);

    conditions
        .map(|cond| cond.test(n))
        .reduce(|a, b| a.merge(b, |s1, s2| s1 + &s2)).unwrap()
        .unwrap_or(n.to_string())
}
