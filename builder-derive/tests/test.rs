#[derive(builder_derive::Builder)]
struct Command {
    name: String,
    shapes: Vec<String>,
    properties: Option<i64>,
}

#[allow(dead_code)]
#[cfg(test)]
mod test_builder_derive {
    use super::*;

    #[test]
    fn test_empty_builder() {
        let builder = Command::builder();

        assert!(builder.name.is_none());
        assert!(builder.shapes.is_none());
        assert!(builder.properties.is_none());
    }

    #[test]
    fn test_can_add_fields() {
        let builder = Command::builder();

        builder.name("test".to_owned());
        assert_eq!(builder.name, Some("test".to_owned()));

        builder.shapes(vec!["some", "more", "testing"]);
        assert_eq!(
            builder.shapes,
            Some(vec![
                "some".to_owned(),
                "more".to_owned(),
                "testing".to_owned()
            ])
        );
    }
}
