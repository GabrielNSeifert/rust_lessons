// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
   
   {/*
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    }); 

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //

    fn inspect (arg: &String){
        if arg.ends_with("s") {
            println!("Oh yeah, plural!");
        } else {
            println!("Oh no, singular!")
        };
    }

    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    
    fn change (s: &mut String){
        if s.ends_with("s") {
            return;
        } else {
            s.push_str("s");
        }
    }

    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //

    fn eat(s: String) -> bool {
        if s.contains("a") && s.starts_with("b") {
            return true;
        } else {
            return false;
        }
    }

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //

    fn bedazzle (s: &mut String){
        *s = "sparkly".to_string();
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
*/}



    /*

    Write a short program that prints each number from 1 to 100 on a new line. 

    For each multiple of 3, print "Fizz" instead of the number. 

    For each multiple of 5, print "Buzz" instead of the number. 

    For numbers which are multiples of both 3 and 5, print "FizzBuzz" instead of the number.
    
     */

    fn fizz_buzz (){
        let mut n: i32 = 0;

        while n <= 99 {

            n = n + 1;

            if n % 3 == 0 && n % 5 == 0 {
                println!("FizzBuzz \n");
            } else if n % 5 == 0 {
                println!("Buzz \n");
            } else if n % 3 == 0 {
                println!("Fizz \n");
            } else {
                println!("{} \n", n);
            }
        };
    }

    fizz_buzz();

}
