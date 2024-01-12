// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//          area(width1, height1)
//     );
// }

// Refactoring with Tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//           area(rect1)
//     );
// }

// Refactoring with Structs
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:#?}", rect1);

//     println!(
//         "The area of the rectangle is {} square pixels.\n",
//           area(&rect1)
//     );

    // using dbg! macro instead
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);
    // dbg! macro can be really helpful when you’re 
    // trying to figure out what your code is doing!
// }



use rectangles::Rectangle;

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // calling an associated funtion with ::
    let sq = Rectangle::square(3);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the square is {} square pixels.",
        sq.area()
    );

    // using name width for a method this is known as a getter
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }   

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
 
}
