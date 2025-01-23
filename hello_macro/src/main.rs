use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use my_macro_default::MyDefault;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

#[derive(MyDefault, Debug)]
struct SomeData (u32,String);

#[derive(MyDefault, Debug)]
struct User {
    name: String,
    data: SomeData,
}


fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
    println!("{:?}", User::default());
}