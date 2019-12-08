/// Decorator
trait Tag {
    fn draw(&self) -> String;
}

struct Text {
    content: String,
}

impl Tag for Text {
    fn draw(&self) -> String {
        self.content.to_owned()
    }
}

struct Span {
    tag: Box<dyn Tag>,
}

impl Tag for Span {
    fn draw(&self) -> String {
        format!("<span>{}</span>", self.tag.draw())
    }
}

struct Div {
    tag: Box<dyn Tag>,
}

impl Tag for Div {
    fn draw(&self) -> String {
        format!("<div>{}</div>", self.tag.draw())
    }
}

#[test]
fn main() {
    println!("## Decorator ---------------------");

    let text = Text {
        content: "hello world".to_owned(),
    };

    let span = Span {
        tag: Box::new(text),
    };

    let div = Div {
        tag: Box::new(span),
    };

    assert_eq!(div.draw(), "<div><span>hello world</span></div>");
}
