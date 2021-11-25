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
}
fn _use_param(ip_type: IpPrtcl) {
    println!("{:?}", ip_type);
}
fn main() {
    _define();
    _use_param(IpPrtcl::V4);
}
