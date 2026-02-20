fn main() {
    let mut var1 = String::from("For ever");
    var1.push_str(" and ever");
    println!("Hello, world! {var1}");

    var1 = String::from("For ever and ever");
    println!("Hello, world! {var1}");

}
