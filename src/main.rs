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
    
    // Fix the error below with least amount of modification to the code
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
}
