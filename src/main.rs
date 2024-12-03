//use core::panicking::panic;

#[allow(unused_variables)]

fn main() {  
    //DAY ONE - variables, mutabilty, shadowing, output, destructuring, datatypes, memory
    println!("Hello World!");
    let x = 5; //immutable
    let mut y = 10; //mutable
    y = 15; // reassigns value to y
    
    const _MAX_POINTS: u32 = 100_000;
    
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr: [i32; 3] = [1, 2, 3];
    
    let mut s = String::from("hello");
    s.push_str(", world");
    
    let slice:&str = &s[0..5];
    
    //1. Fix the error below with least amount of modification to the code
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning ! escaped with (_)

    assert_eq!(x, 5);
    println!("Success!");
    
    //2. Fill the blanks in the code to make it compile
    let mut x = 1; //added mut
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
    
    //3. Scope- // Fix the error below with least amount of modification
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    
    //4.  Fix the error with the use of define_x
    fn _major() {
        define_x();  //fixed
    }
    
    fn define_x() {
        let x: &str = "hello";
        println!("{}, world", x);
    }
    
    // 5. shadowing -Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); //replaced 5 with 12
    }
    assert_eq!(x, 5); //replaced 12 with 5

    let x = 42;
    println!("{}", x); // Prints "42".
    
    // 6.Remove a line in the code to make it compile
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x; 
    x += 3;
    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 
    println!("Success!");
    
    //7. Unused variables -Fix the warning below with (escape error)
    let _x = 1;
    
    
    // 8. Destructuring -Fix the error below with least amount of modification
    let (mut x, y) = (1, 2); //added mut
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
    
    // 9. Destructuring assignments
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]); //added[3,2]

    println!("Success!");

     //DAY TWO - NUMBERS, SIGNED AND UNSIGNED INTS, FLOATS
     // 1. Remove something to make it work
     let x: i32 = 5;
     let mut y: i32 = 5; //replaced u32 with i32
 
     y = x;
     
     let z = 10; // Type of z ? 
 
     println!("Success 1 day2!");

     //2. Fill the blank
    let v: u16 = 38_u8 as u16;

    println!("Success 2 day 2!");

    
    // 3. Modify `assert_eq!` to make it work

    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success 3 day 2!");
    // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
     
     
    //4. Fill the blanks to make it work
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
    println!("Success 4 day 2!");

    //5. Fix errors and panics to make it work
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);

    
    //6. Modify `assert!` to make it work
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success 6 day 2!");


    //7.  Fill the blank to make it work

    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(typeOf(&x), "f64".to_string());
    println!("Success 7 day 2!");
    fn typeOf<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
 
    // 8. Make it work in two distinct ways
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("Success 8 day 2!");


    // 9. Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }

    
    //10. Fill the blanks
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success 10 day 2!");

    
    //11. Fill the blanks and fix the errors

    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    
    
    // DAY THREE- CHAR, BOOL AND UNIT
    
    //1. Make it work char
    use std::mem::size_of_val;
    
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success 1 day 3!");

    
    //2. Make it work - char
    let c1: char = '中'; //change double quotes to single. double is for &str type, single is for char type 
    print_char(c1);
 
    fn print_char(c : char) {
        println!("{}", c);
    }

    
    //3. Make println! work - bool
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success 3 day 3!");
    } 
   
   
    //4.  Make it work - bool
    let f: bool = true;
    let t: bool = true || false;
    assert_eq!(t, f);

    println!("Success 4 day 3!");


    
    //5. Make it work, don't modify `implicitly_ret_unit` ! - unit type
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success 5 day 3!");

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }


    // 6. What's the size of the unit type?
    // Modify `4` in assert to make it work
    // use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success 6 day 3!");


    // DAY FOUR - STATEMENTS, EXPRESSIONS AND FUNCTIONS
    // Statements - Instructions that perform some action but do not produce a value
    // Function definitions are statements, as well as all code that ends with ';'
    // Expression - evaluate to a resultant value
  
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: () = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    //1.  Make it work with two ways
    let v:i32  = {
        let mut x: i32 = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
    println!("Success 1 day 4!");

    
    //2.
    let v: i32 = {
        let x: i32 = 3;
        x
    };
    assert!(v == 3);
    println!("Success 2 day 4!");


    //3.
    let s:i32 = add(1 , 2);
    assert_eq!(s, 3);
    println!("Success 3 day 4!");
    fn add(x: i32, y: i32) -> i32  {
        x + y
    }
 

    // FUNCTIONS
    

    //4. Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: i32 = sumM(x, y);

    assert_eq!(s, 3);

    println!("Success 4 day 4!");

    fn sumM(x: i32, y: i32) -> i32 {
        x + y
    }

    //5.
    print(); 
    // Replace i32 with another type
    fn print() -> () {
       println!("Success 5 day 4!");
    }

    //6. Solve it in two ways
    // DON'T let `println!` work
    never_return();
    println!("Failed!");

    fn never_return() -> () { //replace () wit !
        // Implement this function, don't modify the fn signatures
        // panic!() //uncomment
        
    }

    
    //7.
    println!("Success 7 day 4!");
    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        };
        
        // Rather than returning a None, we use a diverging function instead
        never_return_fn()
    }

    // IMPLEMENT this function in THREE ways
    fn never_return_fn() -> ! {
        panic!() //or
        // unimplemented!() //or
        //todo!()
    }

    //8. FILL in the blank
    // let b: bool = false;

    // let _v: i32 = match b {
    //     true => 1,
    //     // Diverging functions can also be used in match expression to replace a value of any value
    //     false => {
    //         println!("Success 8 day 4!");
    //         panic!("we have no value for `false`, but we can panic");
    //     }
    // };

    // println!("Exercise Failed if printing out this line!");


    //DAY FIVE - SCOPE, OWNERSHIP, MEMORY, CONTROL FLOWS
    // 1.  Use as many approaches as you can to make it work
     let x: String = String::from("Hello world");
     let y: String = x.clone();
     println!("{}, {}",x, y);
     println!("Success 1 day 5!");

     //2.  Don't modify code in main!
    let s1: String = String::from("Hello world");
    let s2: String  = take_ownership(s1);

    println!("{}", s2);
    println!("Success 2 day 5!");

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        return s;
    };

    
    let s = give_ownership();
    println!("{}", s);
    println!("Success 3 day 5!");

    // Only modify the code below!
    fn give_ownership() -> String {
        let s: String = String::from("Hello world");
        // Convert String to Vec
        let _s= s.as_bytes();
        return s;
    }

    //DAY SIX - COMPOUND TYPES, PATTERN MATCH

    //DAY SEVEN - METHODS AND ASSOCIATED FUNCTIONS

    //DAY EIGHT - GENERICS AND TRAITS

    //DAY NINE - COLLECTION TYPES

    //DAY TEN - TYPE CONVERSION

    //DAY ELEVEN - RESULT AND PANIC
    
    //DAY TWELVE -CRATE AND MODULE

    //DAY THIRTEEN - COMMENTS AND DOCS

    //DAY FOURTEEN - FORMATTED OUTPUT

    //DAY FIFTEEN - LIFETIME

    //DAY SIXTEEN - FUNCTIONAL PROGRAMMING

    //DAY SEVENTEEN - RECAP AND ASSESSMENTS

    //DAY EIGHTEEN  - MINI PROJECTS

    //DAY NINETEEN - MORE PROJECTS

    //DAY TWENTY - RE-LEARNING ENUM AND STRUCTS

    //DAY TWENTY ONE  - REVISITING PATTERN MATCHING

    //DAY TWENTY TWO - STACK AND HEAP AGAIN

    //DAY TWENTY THHREE - MALARIA TEST PROMPT

    //DAY TWENTY FOUR - MALARIA TEST PROMPT CONT'D

    //DAY TWENTY FIVE - MORE PROJECTS...
    
    //DAY TWENTY SIX - MORE ...

    //DAY TWENTY SEVEN - GETTING RUSTY OR GETTING BETTER WITH RUST?
    
    //DAY TWENTY EIGHT - CIRCLED BACK

    //DAY TWENTY NINE - DEBUGGING

    //DAY THIRTY - STILL DEBUGGING

    //DAY THIRTY ONE - I GOT CONFUSED

    //DAY THIRTY TWO - A CALCULATOR 

    //DAY THIRTY-THREE - VECTOR, ARRAYS

    //DAY THIRTY FOUR - MORE ON TUPLES

    //DAY THIRTY FIVE -RUST BEING RUST

    //DAY THIRTY SIX - I AM TIRED

    //DAY THIRTY SEVEN - RUST

     //DAY THIRTY EIGHT - RUST

     //DAY THIRTY NINE - RUST

     //day forty - rust
}
