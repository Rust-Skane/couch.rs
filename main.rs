extern crate http;
extern crate extra;
extern crate serialize;

use std::io::net::tcp;

use extra::url;
use serialize::json;

use http::client::RequestWriter;
use http::method::{Get};

pub struct Couch {
  server: url::Url
}

impl Couch {
  fn new(server: url::Url) -> Couch {
    return Couch { server: server };
  }

  fn server_info(&self) -> json::Json {
    let mut url = self.server.clone();

    url.path = ~"/";

    let request = match RequestWriter::<tcp::TcpStream>::new(Get, url) {
      Ok(req) => req,
      Err(_) => fail!("TODO: Implement")
    };

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_) => fail!("TODO: This example can progress no further with no response :-("),
    };

    return match json::from_reader(&mut response) {
      Ok(json) => json,
      Err(_) => fail!("TODO: Not a CouchDB server?")
    }
  }
}

fn main() {
  let couch = Couch::new(from_str("http://localhost:5984/").unwrap());

  println!("{:?}", couch.server_info());
}
