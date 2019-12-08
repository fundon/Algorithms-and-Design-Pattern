/// Builder
trait Builder {
    fn status(&mut self, status: u16) -> &mut Self;
    fn body(&mut self, body: &str) -> &mut Self;
    fn header(&mut self, key: &str, value: &str) -> &mut Self;
}

#[derive(Debug, Default)]
struct Response {
    status: u16,
    body: String,
    header: (String, String),
}

impl Builder for Response {
    fn status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }

    fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_owned();
        self
    }

    fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.header = (key.to_owned(), value.to_owned());
        self
    }
}

#[derive(Debug)]
struct Director {}

impl Director {
    fn ok(r: &mut Response) {
        r.status(200).body("succeed").header("type", "ok");
    }

    fn err(r: &mut Response) {
        r.status(500).header("type", "err").body("wronged");
    }

    fn not_found(r: &mut Response) {
        r.header("type", "not found").body("").status(404);
    }
}

#[test]
fn main() {
    println!("## Builder ---------------------");
    let mut r_1 = Response::default();

    Director::ok(&mut r_1);
    assert_eq!(r_1.status, 200);
    assert_eq!(r_1.body, "succeed".to_owned());
    assert_eq!(r_1.header, ("type".to_owned(), "ok".to_owned()));

    Director::err(&mut r_1);
    assert_eq!(r_1.status, 500);
    assert_eq!(r_1.body, "wronged".to_owned());
    assert_eq!(r_1.header, ("type".to_owned(), "err".to_owned()));

    Director::not_found(&mut r_1);
    assert_eq!(r_1.status, 404);
    assert_eq!(r_1.body, "".to_owned());
    assert_eq!(r_1.header, ("type".to_owned(), "not found".to_owned()));
}
