fn integers( ) -> i32{
    let x: i32 = 27;
    return x 
}

fn unsigned_int() -> u32{
    let x: u32 = 82;
    return x 
}

fn boolean() -> bool{
    let y: bool = true;
    return y
}

fn strings() {
    let some_slice: &str = "donut"; //slice
    let some_string: String = String::from("donut"); //string
    println!("This is a slice: '{}' and this is a string: '{}' ",
     some_slice, some_string);
}

fn constants() -> i32 {
    const SOME_CONSTANT: i32 =29;
    return SOME_CONSTANT
}

fn constants2() -> u32 {
    const SOME_CONSTANT: u32 =29;
    return SOME_CONSTANT
}

fn ifer() {
    if boolean() {
        println!("Hello, world!");
    }
}

fn whiler() {
    let mut x = 0;
    while boolean() {
        x += 1;
        if x < 6 {
            println!("whiler")
        }
        else {
            break
        }
    }
}

// Arrays
fn arrays() {
    // Mutable
    let mut arr: [i32, 3] = [4, 5, 3];
    arr[0] =1;
    arr[1] =2;
    for x in &arr {
        println!("{}", x);
    }

    // Initialize with expression
    let exp_array: [i32; 5] = [0; 5];
    for x in &exp_array {
        println!("{}", x);
    }

    // Initialize variables with an array
    let [greg, mark] = ["Greg".to_string(), "Mark".to_string()];
    println!("{}", greg);
    println!("{}", mark);
}

// Vectors
fn vectors() {
    let vector: Vec<i32> = vec![1, 3, 7, -1];
    let mut vector: Vec<i32> = (0..10).collect();
    for x in &vector {
        println!("{}", x);
    }
    // Append
    vector.push(-2);
    println!("The last value now is {}", vector[vector.len() - 1]);
    println!("{:?}", vector.pop());
}

// Iterating
fn iterating() {

    // Arrays
    let arr: [i32; 5] = [0; 5];
    // By reference (.iter)
    for x in arr.iter() {
        println!("{}", x);
    }
    // By reference (.iter) - enumerated
    for x in arr.iter().enumerate() {
        // gives you a tuple: (index, value)
        println!("{}", x);
    }
    // By value (.into_iter) will not reference variable but pull into
    for item in arr.iter().enumerate() {
        // You can choose to do both index and value
        let (i, x): (usize, i32) = item;
        println!("Index: {}, Value: ()", i, x);
    }

    println!("------------------------");

    // Vectors
    let vector: Vec<i32> = (0..5).collect();
    for x in vector.iter() {
        println!("{}", x);
    }
    let mut mut_vector: Vec<i32> = (0..5).collect();
    for x in mut_vector.iter_mut() {
        *x += 3;
    }
    println!("{:?}", mut_vector);
}



fn console() {
    println!("\nThe addition of constant and integer is {}",
     constants() + integers());
    println!("The addition of constant and unsigned integer is {}",
     constants2() + unsigned_int());
    strings();
    ifer();
    whiler();
    arrays();
    vectors();
    iterating();
}

fn main() {
    console();  
}
