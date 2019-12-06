/// Adapter
trait Read {
    fn output(&self) -> String;
}

#[derive(Debug, Default)]
struct Reader {
    body: String,
}

impl Reader {
    fn read(&mut self, from: impl Read) {
        self.body = from.output()
    }
}

struct Json {
    body: String,
}

impl Read for Json {
    fn output(&self) -> String {
        self.body.to_owned()
    }
}

struct Xml {
    body: Vec<u8>,
}

struct XmlAdapter {
    xml: Xml,
}

impl Read for XmlAdapter {
    fn output(&self) -> String {
        String::from_utf8(self.xml.body.to_owned()).unwrap()
    }
}

fn main() {
    println!("## Adapter ---------------------");

    let mut reader = Reader::default();

    let json = Json {
        body: "JSON".to_owned(),
    };

    reader.read(json);
    println!("read body from json: {}", reader.body);

    let xml = Xml {
        body: vec![b'X', b'M', b'L'],
    };

    let xml_adapter = XmlAdapter { xml };

    reader.read(xml_adapter);
    println!("read body from xml: {}", reader.body);
}
