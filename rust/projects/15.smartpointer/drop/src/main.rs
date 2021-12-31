struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) { 
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    #[allow(unused_variables)]
    let c = CustomSmartPointer { data: String::from("my stuff") };
    #[allow(unused_variables)]
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomerSmartPointer created.");
}
