extern crate http;
extern crate extra;
extern crate serialize;
extern crate collections;

use std::io::net::tcp;

use extra::url;
use serialize::json;
use collections::treemap;

use http::client::RequestWriter;
use http::method::{Get};

pub struct Couch {
  server: url::Url
}

impl Couch {
  pub fn new(server: url::Url) -> Couch {
    return Couch { server: server };
  }

  pub fn server_info(&self) -> ServerInfo {
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

    let json = match json::from_reader(&mut response) {
      Ok(json) => json,
      Err(_) => fail!("TODO: Not a CouchDB server?")
    };

    return match json {
      json::Object(tm) => ServerInfo { json: tm },
      _ => fail!("TODO: Wrong format")
    };
  }
}

pub struct ServerInfo {
  json: ~treemap::TreeMap<~str, json::Json>
}

impl ServerInfo {
  pub fn couchdb<'a>(&'a self) -> &'a str {
    return match self.json.find(&~"couchdb") {
      Some(&json::String(ref s)) => s.as_slice(),
      _ => fail!("Fail!")
    }
  }
  
  pub fn uuid<'a>(&'a self) -> Option<&'a str> {
    return match self.json.find(&~"uuid") {
      Some(&json::String(ref s)) => Some(s.as_slice()),
      _ => None
    }
  }

  pub fn version<'a>(&'a self) -> &'a str {
    return match self.json.find(&~"version") {
      Some(&json::String(ref s)) => s.as_slice(),
      _ => fail!("Fail!")
    }
  }
}

fn main() {
  let couch = Couch::new(from_str("http://localhost:5984/").unwrap());

  let server_info = couch.server_info();
  println!("{:?}", server_info.couchdb());
  println!("{:?}", server_info.uuid());
  println!("{:?}", server_info.version());
}
