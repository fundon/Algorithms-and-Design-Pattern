/// Factory
trait Shape {
    fn draw(&self);
}

struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("{} is drawing", "Rectangle")
    }
}

struct Square {}

impl Shape for Square {
    fn draw(&self) {
        println!("{} is drawing", "Shape")
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("{} is drawing", "Circle")
    }
}

enum ShapeType {
    Rectangle,
    Square,
    Circle,
}

impl ShapeType {
    fn create(t: Self) -> Box<dyn Shape> {
        match t {
            ShapeType::Rectangle => Box::new(Rectangle {}),
            ShapeType::Square => Box::new(Square {}),
            ShapeType::Circle => Box::new(Circle {}),
        }
    }
}

fn main() {
    println!("## Factory ---------------------");
    let shape_1 = ShapeType::create(ShapeType::Rectangle);
    shape_1.draw();

    let shape_2 = ShapeType::create(ShapeType::Square);
    shape_2.draw();

    let shape_3 = ShapeType::create(ShapeType::Circle);
    shape_3.draw();
}
