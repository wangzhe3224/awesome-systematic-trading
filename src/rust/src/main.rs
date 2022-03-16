use polars::prelude::*;

fn main() {
    println!("Hello, world!");
    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let df: Result<DataFrame> = DataFrame::new(vec![s1, s2]);
    println!("{:?}", df.unwrap());
}
