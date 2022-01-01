struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) { 
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn _drop() {
    #[allow(unused_variables)]
    let c = CustomSmartPointer { data: String::from("my stuff") };
    #[allow(unused_variables)]
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomerSmartPointer created.");
}

fn _mem_drop() {
    println!("Start _mem_drop");
    #[allow(unused_variables)]
    let c = CustomSmartPointer { data: String::from("some data") };
    drop(c);
    println!("CustomerSmartPointer created.");
    println!("End _mem_drop");
}
fn main() {
    _drop();
    _mem_drop();
}
