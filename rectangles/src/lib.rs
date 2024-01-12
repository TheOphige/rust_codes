#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // creating method that calls another instance of the type Rectangle
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    pub fn width(&self) -> bool {
        self.width > 0
    }
}

// associated function square(its not necessarily a method)
impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}