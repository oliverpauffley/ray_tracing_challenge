use builder_derive::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Command {
    name: String,
    shapes: Vec<String>,
    properties: Option<i64>,
}

#[derive(Builder, Debug, PartialEq)]
struct File {
    name: String,
    #[builder(eac = "location")]
    locations: Vec<String>,
}

fn main() {
    let builder = Command::builder();
    assert!(builder.name.is_none());
    assert!(builder.shapes.is_none());
    assert!(builder.properties.is_none());
}
