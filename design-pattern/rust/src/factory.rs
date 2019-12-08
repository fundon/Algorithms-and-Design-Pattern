/// Factory
trait Shape {
    fn draw(&self) -> String;
}

struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) -> String {
        "Rectangle".to_owned()
    }
}

struct Square {}

impl Shape for Square {
    fn draw(&self) -> String {
        "Shape".to_owned()
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) -> String {
        "Circle".to_owned()
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

#[test]
fn main() {
    println!("## Factory ---------------------");
    let shape_1 = ShapeType::create(ShapeType::Rectangle);
    assert_eq!(shape_1.draw(), "Rectangle".to_owned());

    let shape_2 = ShapeType::create(ShapeType::Square);
    assert_eq!(shape_2.draw(), "Shape".to_owned());

    let shape_3 = ShapeType::create(ShapeType::Circle);
    assert_eq!(shape_3.draw(), "Circle".to_owned());
}
