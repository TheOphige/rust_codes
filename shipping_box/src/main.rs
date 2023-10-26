#![deny(clippy::all)]

// box color enum
enum Color {
    Brown,
    Red,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
            Color::Green => println!("green"),
        }
    }
}


// Dimensions struct
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}


// box characteristics dimensions, weight, and color
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    // print characteristiscs
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    // small box
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();

    // medium box
    let medium_dimensions = Dimensions {
        width: 1.5,
        height: 3.0,
        depth: 4.0,
    };
    let medium_box = ShippingBox::new(7.0, Color::Green, medium_dimensions);
    medium_box.print();

    // big box
    let big_dimensions = Dimensions {
        width: 2.0,
        height: 4.0,
        depth: 4.5,
    };
    let big_box = ShippingBox::new(5.0, Color::Brown, big_dimensions);
    big_box.print();
}
