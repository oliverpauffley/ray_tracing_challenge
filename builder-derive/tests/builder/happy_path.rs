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
    #[builder(each = "location")]
    locations: Vec<String>,
}

fn main() {
    {
        let builder = Command::builder();
        assert!(builder.name.is_none());
        assert!(builder.shapes.is_none());
        assert!(builder.properties.is_none());
    }
    {
        let mut builder = Command::builder();
        builder.name("test".to_owned());
        builder.shapes(vec![
            "some".to_string(),
            "more".to_string(),
            "testing".to_string(),
        ]);
        builder.properties(1);
        let got = builder.build().unwrap();

        let want = Command {
            name: "test".to_string(),
            shapes: vec![
                "some".to_string(),
                "more".to_string(),
                "testing".to_string(),
            ],
            properties: Some(1),
        };

        assert_eq!(want, got);
    }
    {
        let file = File::builder()
            .name("test-file.rs".to_string())
            .location("here".to_string())
            .location("there".to_string())
            .build();
        let want = File {
            name: "test-file.rs".to_string(),
            locations: vec!["here".to_string(), "there".to_string()],
        };
        assert_eq!(want, file.unwrap())
    }
}
