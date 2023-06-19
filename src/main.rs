fn main() {
    println!("Hello, world!");
}

#[test]
fn foo(){
    println!("Hello, world!");
}

mod bar{
    #[test]
    fn foo_2(){
        println!("Hello, world!");
    }
}