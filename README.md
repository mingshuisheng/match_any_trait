# match_any_trait

Provides a procedural macro, that is match expressions for any trait


# Install
```shell
cargo add match_any_trait
```

Example

```rust
use std::any::Any;
use match_any_trait::match_any_trait;

#[derive(Debug)]
struct MyStruct {
    x: u32,
}

#[derive(Debug)]
struct MyNum {
    y: u32,
}

#[derive(Debug)]
struct MyTuple;

#[derive(Debug)]
struct MyTemplate;

fn main() {
    // let num = MyStruct{
    //     x: 1,
    // };

    // let num = MyNum{
    //     y: 1,
    // };

    // let num = MyTuple;

    let num = MyTemplate;

    let num = &num as &dyn Any;

    match_any_trait! {
        match num {
            MyStruct(s) | MyNum(s) => println!("num is a {:?}", s),
            MyTuple => println!("num is a MyTuple") 
            MyTemplate => println!("num is a MyTemplate"),
            _ => println!("num is unknown"),
        }
    }

}
```