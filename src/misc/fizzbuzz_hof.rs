#![allow(dead_code)]

/* FizzBuzz implementation using Higher Order Functions 
 *     This originates from https://themonadreader.files.wordpress.com/2014/04/fizzbuzz.pdf
 *     Another material: https://youtu.be/FyCYva9DhsI?t=928
 * 
 * This approach eliminates unnecessary checks (e.g. n % 15 == 0, str == "", ...)
 * and does not introduce a combinatorial explosion when it is extended over other divisors.
 * 
 * However, IMHO, this approach is not so comprehensible for everyone at a first glance,
 * and it would not be expected to implement FizzBuzz in this way, 
 * both in a real world project and in a job interview.
 * 
 * So this code piece serves as a learning exercise
 * about Higher Order Functions and Functional Programming.
 */

// A Program generates a string using another string
type Program = Box<dyn FnOnce(String) -> String>;

// A Context is a Program that consists of another Program
// This is the 'program with a hole' in the above pdf.
type Context = Box<dyn FnOnce(Program) -> Program>;

// Generate a Context that includes a Program that prints `s` only if `n` is divisible by `d`
fn test(n: u32, d: u32, s: &str) -> Context {
    let id_context: Context = Box::new(|f: Program| f);
    let s = s.to_owned(); // this prevents closures from having 

    if n % d == 0 {
        Box::new(move |f: Program| Box::new(move |_: String| format!("{}{}", s, f("".to_owned()))))

        // "".to_owned() prevents `n` from being printed when it is divisible by `d`
        // It serves a similar purpose to the HALT command in the above pdf.
    } else {
        id_context
    }
}

// This chains two Contexts into one Context.
// As its name states, this is a function composition.
fn compose(a: Context, b: Context) -> Context {
    Box::new(move |f: Program| a(b(f)))
}

// The FizzBuzz problem
fn fizzbuzz(n: u32) -> String {
    let id_program: Program = Box::new(|s: String| s);

    let fizz: Context = test(n, 3, "Fizz");
    let buzz: Context = test(n, 5, "Buzz");

    let fb: Program = compose(fizz, buzz)(id_program);
    fb(n.to_string())
}

// Prints FizzBuzz from 1 to 50
fn main() {
    for i in 1..=50 {
        println!("{}", fizzbuzz(i));
    }
}

// Extended version of the FizzBuzz problem
fn fizzbuzzhisshowl(n: u32) -> String {
    let id_program: Program = Box::new(|s: String| s);

    // Those conditions can be stored in a 'declarative' fashion, as a list,
    // and they can be chained by `reduce()` and `compose()`.
    let tests = IntoIterator::into_iter([
        test(n, 3, "Fizz"),
        test(n, 5, "Buzz"),
        test(n, 7, "Hiss"),
        test(n, 11, "Howl"),
    ]);

    let fbhh = (tests.reduce(compose).unwrap())(id_program);
    fbhh(n.to_string())
}