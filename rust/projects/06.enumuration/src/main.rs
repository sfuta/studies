/**
 * Define and basic use
 */
#[derive(Debug)]
enum IpPrtcl {
    V4, V6
}
fn _define() {
    let v4 = IpPrtcl::V4;
    let v6 = IpPrtcl::V6;
    println!("{:?}, {:?}", v4, v6);

    // Set Enum parameter
    fn _use_param(ip_type: IpPrtcl) {
        println!("{:?}", ip_type);
    }
    _use_param(IpPrtcl::V4);
    _use_param(IpPrtcl::V6);
}

/**
 * Combine structure
 */
#[derive(Debug)]
struct IpAddr {
    prctl: IpPrtcl,
    address: String
}
fn _struct() {
    let loopback_v4 = IpAddr { prctl: IpPrtcl::V4, address: String::from("127.1.1.1") };
    let loopback_v6 = IpAddr { prctl: IpPrtcl::V6, address: String::from("::1") };
    println!("{:?}, {:?}", loopback_v4, loopback_v6);
}

/**
 * Define2
 */
#[derive(Debug)]
enum IpAddr2 { V4(String), V6(String), }
#[derive(Debug)]
enum IpAddr3 { V4(u8, u8, u8, u8), }
#[derive(Debug)]
enum IpAddr4 { V4(IpAddr), }
#[derive(Debug)]
enum IpAddr5 { V4(IpPrtcl), }
fn _define2() {
    let _v4 = IpAddr2::V4(String::from("127.2.3.4"));
    let _v6 = IpAddr2::V6(String::from("::1"));
    println!("{:?}, {:?}", _v4, _v6);

    let __v4 = IpAddr3::V4(127, 0, 0, 3);
    println!("{:?}", __v4);

    let ___v4 = IpAddr4::V4(IpAddr { prctl: IpPrtcl::V4, address: String::from("127.0.2.3")});
    println!("{:?}", ___v4);

    let ____v4 = IpAddr5::V4(IpPrtcl::V4);
    println!("{:?}", ____v4);
}

/**
 * Method define
 */
#[derive(Debug)]
enum Message {
    Quit, Position {x: i32, y: i32}, Value(String), Color(i32, i32, i32)
}
impl Message {
    fn call(&self) {
        // self is enum part
        //  ex) Value(String), Quit
        println!("{:?}", self);

    }
    fn call2(&self) {
        println!("{:?}", self);
    }
}
fn _define_method() {
    println!("Message::Value:{:?}", Message::Value(String::from("msg")));
    println!("Message::Position:{:?}", Message::Position{x:20, y:10});
    println!("Message::Color.call():{:?}", Message::Color(2, 4, 5).call());
    println!("Message::Quit.call2():{:?}", Message::Quit.call2());
}

/**
 * Option and null
 */
fn _option() {
    let some_num: Option<i8> = Some(5);
    let some_str = Some("a string");
    let none_num: Option<i32> = None;
    let num: i8 = 4;

    println!("{:?}, {:?}, {:?}, {}", some_num, some_str, none_num, num);
    // NG: some_num and num of type is not same.
    // let sum = some_num + num;

}
fn main() {
    _define();
    _struct();
    _define2();
    _define_method();
    _option();
}
