extern crate blog2;
use blog2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");
    let post = post.request_review();
    let post = post.approve();

    println!("{}", post.content());
}