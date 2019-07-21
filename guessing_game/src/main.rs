//‘crate’ is a package of Rust code.
extern crate rand;

//IO Library(io library from the standard library)
use std::io;

//(Ordering type) type called std::cmp::Ordering into scope.
use std::cmp::Ordering;

//traits
use rand::Rng;

//fn syntax declares a new function, the () s indicate that there are no arguments,
//() , an empty tuple (primitive-types.html#tuples).
fn main() {

    //println!() is a macro
    println!("Guess the number!");

    //let statement, which is used to create ‘variable bindings’.
    //they’re immutable by default.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    //The {} s are a placeholder,
    //println!("The secret number is: {}", secret_number);

    loop{
        println!("Your Guess!");

        //uses mut : it makes a binding mutable.
        //String is a string type, provided by the standard library.
        //The ::new() syntax uses :: because this is an ‘associated function’ of a particular type.
        let mut guess = String::new();

        //could be std::io::stdin().
        //Methods (method-syntax.html) are like associated functions, 
        //but are only available on a particular instance of a type, rather than the type itself
        io::stdin().read_line(&mut guess).expect("Failed to guess");

        //io::Result (../std/io/type.Result.html). Rust has a number of types named Result in its standard
        //library: a generic Result (../std/result/enum.Result.html), and then specific versions for sub-libraries, like io::Result .
        //The purpose of these Result types is to encode error handling information.
        
        //"Shadowing" lets us re-use the guess name
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //A Result is returned by parse() , this is an enum like Ordering ,
        // but in this case, each variant has some data associated with it: Ok is a success, and Err is a failure.
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }

    }

}

//Rust has a feature called ‘references (references-and-borrowing.html)’,
//which allows you to have multiple references to one piece of data, which can reduce
//copying.


// enum Foo {
//       Bar,
//       Baz,
// }

// With this definition, anything of type Foo can be either a Foo::Bar or a Foo::Baz . We use the :: to
// indicate the namespace for a particular enum variant.

//let x: i32 = 5;
//x is a binding with the type i32 and the value 5 .
