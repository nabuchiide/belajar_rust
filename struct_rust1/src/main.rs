#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // area with variable primitive
    let width = 30;
    let height = 50;
    println!("Area of rectangle with variable primitive is {} square pixels", area(width, height));

    //area with tuple
    let rect = (30, 50);
    println!("Area of rectangle with tuple is {} squaer pixels", area_tuples(rect));

    //area with Rectangle
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    dbg!("rectangle is {:#?}",&rectangle);
    println!("Area of rectangle with struct is {} squaer pixels", area_struct(&rectangle));
}

fn area(width: i32, height: i32) -> u32 {
    (width * height) as u32
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(dimension: &Rectangle) -> u32 {
    dimension.width * dimension.height
}