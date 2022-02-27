extern crate jsonrpc;
extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Deserialize)]
struct MyStruct {
    elem1: bool,
    elem2: String,
    elem3: Vec<usize>
}

fn main() {
    // The two Nones are for user/pass for authentication
    let rtt = jsonrpc::simple_rtt::Tripper::new();
    let client = jsonrpc::client::Client::with_rtt(rtt, "example.org".to_owned(), None, None);
    match client.do_rpc::<MyStruct>(&request) {
        Ok(mystruct) => println!("Ok"),// Ok!
        Err(e) => println!("Err"), // Not so much.
    }
}
