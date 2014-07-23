extern crate url;
extern crate http;
extern crate serialize;

use std::io;

use serialize::json;

#[deriving(Decodable)]
pub struct Vendor {
  pub version: String, pub name: String
}

#[deriving(Decodable)]
pub struct ServerInfo {
  couchdb: String,
  uuid: String,
  version: String,
  vendor: Vendor
}

pub struct Couch {
  server: url::Url
}

fn parse_json<T: serialize::Decodable<json::Decoder, json::DecoderError>>(body: io::IoResult<Vec<u8>>) -> Option<T> {
  let body = match body {
    Ok(b) => b, Err(_) => return None
  };

  let mut reader = io::BufReader::new(body.as_slice());
  
  let json = match json::from_reader(&mut reader) {
    Ok(j) => j, Err(_) => return None
  };

  let mut decoder = json::Decoder::new(json);

  return match serialize::Decodable::decode(&mut decoder) {
    Ok(j) => Some(j), Err(_) => None
  };
}

impl Couch {
  pub fn new(server: url::Url) -> Couch {
    return Couch { server: server };
  }

  pub fn do_request(&self, method: http::method::Method, path: url::Path, body: Option<&[u8]>) -> io::IoResult<Vec<u8>> {
    let mut url = self.server.clone();

    url.path = path;

    let mut request: http::client::RequestWriter = match http::client::RequestWriter::new(method, url) {
      Ok(r) => r,
      Err(e) => return Err(e)
    };

    match body {
      Some(body) => {
        request.headers.content_length = Some(body.len());
        match request.write(body) {
          Err(e) => return Err(e), _ => ()
        };
      },
      None => {}
    }

    return match request.read_response() {
      Ok(mut res) => res.read_to_end(),
      Err((_, e)) => return Err(e)
    }
  }

  pub fn get(&self, path: url::Path) -> io::IoResult<Vec<u8>> {
    return self.do_request(http::method::Get, path, None);
  }

  pub fn delete(&self, path: url::Path) -> io::IoResult<Vec<u8>> {
    return self.do_request(http::method::Put, path, None);
  }

  pub fn put(&self, path: url::Path, body: &[u8]) -> io::IoResult<Vec<u8>> {
    return self.do_request(http::method::Put, path, Some(body));
  }

  pub fn post(&self, path: url::Path, body: &[u8]) -> io::IoResult<Vec<u8>> {
    return self.do_request(http::method::Put, path, Some(body));
  }

  pub fn server_info(&self) -> ServerInfo {
    return match parse_json(self.get(url::Path::new("/".to_string(), Vec::new(), None))) {
      Some(s) => s, None => fail!("couchdb couldn't be parsed")
    };
  }
}

static server_url: &'static str = "http://localhost:5984/";

#[test]
fn test_server_info() {
  let info = Couch::new(from_str(server_url).unwrap()).server_info();

  assert_eq!(info.couchdb, "Welcome".to_string());
  assert_eq!(info.uuid.len(), 32);
  assert_eq!(info.version, info.vendor.version);
  assert_eq!(info.vendor.name, "The Apache Software Foundation".to_string());
}
