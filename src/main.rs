pub fn hello_world() -> String {
    String::from("Hello, world!")
}
pub fn goodbye_world() -> String {
    String::from("Goodbye, world!")
}
fn main() {
    let hello = hello_world();
    let goodbye = goodbye_world();
    println!("hello:{} goodbye:{}", hello, goodbye);
}
