// compilerはsrc/lib.rsのみ確認を行う
mod network;
// network module separated file to network.rs
// mod network {
//     // if call, network::connect()
//     fn connect() {
//     }

//     mod server {
//         // if call, network::server::connect()
//         fn connect() {
//         }
//     }
// }

mod client;
// client module separated file
// mod client {
//     // if call, client::connect()
//     fn connect() {
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
