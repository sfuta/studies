fn _find_max_v() {

    // pattern 1
    let nums = vec![40, 50, 33, 6];
    let mut largest = nums[0];
    for num in nums {
        if num > largest {
            largest = num; 
        }
    }
    println!("Max value is {} in 'nums'", largest);

    // pattern 2
    let nums = vec![40, 60, 50, 33, 6];
    fn find_max_from(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    println!("Max value is {} in 'nums'", find_max_from(&nums));

    // pattern 3 
    let i32_list = vec![24, 68, 89, 44];
    let char_list = vec!['h', 'k', 'a', 'm'];
    fn find_max_from_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    fn find_max_from_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    println!("Max value is {} in 'i32_list'", find_max_from_i32(&i32_list));
    println!("Max value is '{}' in 'char_list'", find_max_from_char(&char_list));

    // pattern 4 use generics
    // NG, binary operation '>' can not be applied to type 'T'
    // fn p4_find_max_from<T>(list: &[T]) -> T {
    //     let mut largest = list[0];
    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }
    // println!("Max value is {} in 'i32_list'. (use generics function)", p4_find_max_from(&i32_list));
    // println!("Max value is '{}' in 'char_list'. (use generics function)", p4_find_max_from(&char_list));

}

/**
 * Generics with structure and enum
 */
fn _generics_struct_enum () {

    struct Point<T> {x: T, y: T}
    let _int = Point {x: 5, y: 5};
    let _flout = Point {x: 5.1, y: 3.4};
    // NG: x, y type is not same
    // let int_flot = Point {x: 5.1, y: 3};

    #[allow(dead_code)]
    struct Point2<T, U> {x: T, y: U}
    let _char = Point2 {x: "arc" , y: 'b'};
    let _int_flot = Point2 {x: 5.1, y: 3};

    // // enum generics sample from library
    // enum Option<T> {Some(T), None}
    // enum Result<T, E> {Ok(T), Err(E)}

    impl<T> Point<T> { fn x(&self) -> &T { &self.x }}
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    println!("Point x is {} in '_int'", _int.x());
    println!("x, y distance is {} in '_flout'", _flout.distance_from_origin());

    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {x: self.x, y: other.y}
        }
    }

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // ジェネリクスを使用してもパフォーマンスには影響しない
    // ->コンパイラーによって、再定義されるため(単相化)
    // ex)
    let _int = Some(5);
    let _float = Some(5.0);
    // ->
    #[allow(dead_code,non_camel_case_types)]
    enum Option_i32 {Some(i32), None}
    #[allow(dead_code,non_camel_case_types)]
    enum Option_f64 {Some(f64), None}
    let _int = Option_i32::Some(5);
    let _float = Option_f64::Some(5.0);
}

/**
 * trait
 */
pub trait Summary {
    // fn summarize(&self) -> String;
    // -> add defult 
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    headline: String, location: String, author: String, pub content: String
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    username: String, pub content: String, pub reply: bool, pub retweet: bool
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Task {
    pub no: u32, pub summary: String, pub detail: String
}
impl Summary for Task {}

fn _use_trait() {

    let tweet = Tweet {
        username: String::from("horce_ebooks")
        , content: String::from("of cource, as you probably already know, people")
        , reply: false, retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguines win the Stanley Cup Championship!")
        , location: String::from("Pittsburgh, PA, USA")
        , author: String::from("Iceburgh")
        , content: String::from("The Pittesburgh Penguins once again are the best hockey team in the NHL.")
    };
    println!("New article available! {}", article.summarize());

    let task = Task {
        no: 3, summary: String::from("my sample"), detail: String::from("Trait with default")
    };
    println!("Trait sample. {}", task.summarize());
}

fn _use_trait_domain() {

    /**
     * basic
     */
    #[allow(unused_variables, dead_code)]
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize())
    }

    use std::fmt::Display;

    /**
     * muluti
     */
    #[allow(unused_variables, dead_code)]
    fn notify2<T: Summary + Display>(item: T) {
        // println!("Breaking news! {}", item.summarize()); // NG, summarize()がどちらか不明のため
        println!("Setting trait domains")
    }
    #[allow(unused_variables, dead_code)]
    fn notify3<T, U>(item: T, user: U) 
        where T: Summary + Display, U: Display {
        // println!("Breaking news! {}", item.summarize()); // NG, summarize()がどちらか不明のため
        println!("Setting trait domains. use where")
    }

    let i32_list = vec![24, 68, 89, 44];
    let char_list = vec!['h', 'k', 'a', 'm'];
    /**
     * fixe pattern 4 use genericsd
     */
    fn p4_find_max_from<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    println!("Max value is {} in 'i32_list'. (use generics function)", p4_find_max_from(&i32_list));
    println!("Max value is '{}' in 'char_list'. (use generics function)", p4_find_max_from(&char_list));

    // trait境界でメソッド実装を条件分けする
    #[allow(dead_code)]
    struct Pair<T> {x: T, y: T}
    impl<T> Pair<T> {
        #[allow(dead_code)]
        fn new(x: T, y: T) -> Self {Self {x, y}}
    }
    impl<T: Display + PartialOrd> Pair<T> {
        #[allow(dead_code)]
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // blanket implementation
    // DisplayにはToStringが常に実装してある
    // impl<T: Display> ToString for T {....}

}

/**
 * Life time(scope)
 */
fn _lifetime_explain() {

    // Use borrow checker on Rust
    // Follow is compile error, a' > b'
    /*
    let r;                  //--------------+--- a'
    {                       //              |
        let x = 5;          //---+--- b'    | 
        r = &x;             //   |          |
                            //   |          |
    }                       //---+          |
    println!("{}", r);      //--------------+
    */
    // Follow is not compile error, b' > a'
    {
        let x = 5;          //--------------+--- b'
                            //              |
        let r = &x;         //---+--- a'    | 
                            //   |          |
        println!("{}", r);  //   |          |
                            //---+          |
    }                       //--------------+
}

fn _lifetime() {

    // 'a is lifetime declare
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let str1 = String::from("abcd");
    let str2 = "nnn";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);

    // Follow is compile error
    /*
    let sttr1 = String::from("hhhhhh");
    {
        let sttr2 = String::from("nnn");
        let reult = longest(sttr1.as_str(), sttr2);
        println!("The longest string is {}", result);
    }
    let stttr1 = String::from("hhhhhh");
    let rslt;
    {
        let stttr2 = String::from("nnn");
        rslt = longest(stttr1.as_str(), stttr2);
        println!("The longest string is {}", rslt);
    }
    */
}

fn main() {
    _find_max_v();
    _generics_struct_enum();
    _use_trait();
    _use_trait_domain();
    _lifetime_explain();
    _lifetime();
}
