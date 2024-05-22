#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are similar to functions:
// they're declared with fn and their name,
// they can have parameters and a return value,
// and they contain code that is run when they're called from somewhere else.
// However, methods are different from functions in
// that they're defined within the context of a struct
// (or an enum or a trait object), and their first parameter
// is always self, which represents the instance of
// the struct the method is being called on.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rec1.area());
}
