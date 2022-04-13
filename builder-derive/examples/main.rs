use builder_derive::Builder;

#[derive(Builder, Debug)]
#[allow(dead_code)]
pub struct Film {
    title: String,
    director: String,
    on_netflix: bool,
    #[builder(each = "cast")]
    cast: Vec<String>,
    genre: Option<String>,
}

fn main() {
    let mamma_mia = Film::builder()
        .title("mamma mia".to_string())
        .director("dont know".to_string())
        .on_netflix(false)
        .cast("pierce bronson".to_string())
        .build();

    println!("{:#?}", mamma_mia)
}
