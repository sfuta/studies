pub mod a {
    pub mod b {
        pub mod c {
            pub fn last() {
                println!("call a::b::c::last!!");
            }
        }
        pub mod c01 {
            pub fn last() {
                println!("call a::b::c01::last!!");
            }
        }
    }
}


// fn main() {
//     a::b::c::last();
// }


use a::b::c;
use a::b::c01::last;

#[derive(Debug)]
enum SampleVersion {
    V1, V2, V3
}
#[derive(Debug)]
enum SampleVersion2 {
    V21, V22, V23
}

use SampleVersion::{V1, V2};
use SampleVersion2::*;

fn main() {
    c::last();
    last();
    println!("{:?}, {:?}, {:?}", V1, V2, SampleVersion::V3);
    println!("{:?}, {:?}, {:?}", V21, V22, V23);
}