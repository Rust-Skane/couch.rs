#[warn(experimental)];

extern crate http;
extern crate extra;
extern crate serialize;
extern crate collections;

use std::fmt;
use std::io::net::tcp;

use extra::url;
use serialize::json;
use collections::treemap;

use http::method;
use http::client::RequestWriter;

pub struct Couch {
  server: url::Url
}

impl Couch {
  pub fn new(server: url::Url) -> Couch {
    return Couch { server: server };
  }

  fn do_request(&self, method: method::Method, path: ~str) -> json::Json {
    let mut url = self.server.clone();

    url.path = path;

    let request = match RequestWriter::<tcp::TcpStream>::new(method, url) {
      Ok(req) => req,
      Err(_) => fail!("TODO: Implement")
    };

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(e) => {
          println!("{:?}", e);
          fail!("TODO: This example can progress no further with no response :-(");
        }
    };

    return match json::from_reader(&mut response) {
      Ok(json) => json,
      Err(e) => {
        println!("{:?}", e);
        fail!("TODO: Not a CouchDB server?");
      }
    };
  }

  pub fn server_info(&self) -> ServerInfo {
    return match self.do_request(method::Get, ~"/") {
      json::Object(tm) => ServerInfo { json: tm },
      _ => fail!("TODO: Wrong format")
    };
  }

  pub fn create_database(&self, name: &str) -> Option<Database> {
    let path = format_args!(fmt::format, "/{:s}", name);

    return match self.do_request(method::Put, path) {
      json::Object(tm) => {
        match tm.find(&~"ok") {
          Some(&json::Boolean(true)) => Some(self.get_database(name)), _ => None
        }
      },
      _ => fail!("TODO: Wrong format")
    };
  }
  
  pub fn get_database(&self, name: &str) -> Database {
    return Database { server: self.server.clone(), database: name.to_owned() };
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

pub struct Database {
  server: url::Url,
  database: ~str
}

impl Database {
  fn do_request(&self, method: method::Method, path: ~str, data: json::Json) -> json::Json {
    let mut url = self.server.clone();

    url.path = path;

    let mut request = match RequestWriter::<tcp::TcpStream>::new(method, url) {
      Ok(req) => req,
      Err(_) => fail!("TODO: Implement")
    };

    //request.headers.content_length = Some(data.len());

    data.to_writer(&mut request);
    
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(e) => {
          println!("{:?}", e);
          fail!("TODO: This example can progress no further with no response :-(");
        }
    };

    return match json::from_reader(&mut response) {
      Ok(json) => json,
      Err(e) => {
        println!("{:?}", e);
        fail!("TODO: Not a CouchDB server?");
      }
    };
  }  
  
  pub fn create_document(&self, id: &str, content: json::Json) -> Option<(~str, ~str)> {
    let path = format_args!(fmt::format, "/{:s}/{:s}", self.database, id);

    return match self.do_request(method::Put, path, content) {
      json::Object(tm) => {
        match tm.find(&~"ok") {
          Some(&json::Boolean(true)) => {
            let id = match tm.find(&~"id") {
              Some(&json::String(ref s)) => s.to_owned(),
              _ => fail!("finding id")
            };

            let rev = match tm.find(&~"rev") {
              Some(&json::String(ref s)) => s.to_owned(),
              _ => fail!("finding rev")
            };
            Some((id, rev))
          },
          _ => fail!("finding ok")
        }
      },
      _ => None
    };
  }
}

static server_url:&'static str = "http://localhost:5984/";

#[test]
fn test_server_info() {
  let couch = Couch::new(from_str(server_url).unwrap());
  couch.server_info();
  assert!(true);
}

#[test]
fn test_create_database() {
  let couch = Couch::new(from_str(server_url).unwrap());
  couch.create_database("rust");
  assert!(true);
}

#[test]
fn test_create_document() {
  let couch = Couch::new(from_str(server_url).unwrap());
  let database = couch.get_database("rust");
  let content = json::from_str("{\"magic\":true}").unwrap();
  assert!(match database.create_document("test-doc-id", content) {
    Some(_) => true,
    _ => false
  });
  
}