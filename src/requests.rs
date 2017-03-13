use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{self, Future};
use tokio_curl::Session;
use serde;
use serde_json;
use curl::easy::{Easy, List};

use objects;
use errors::*;

fn make_request<S: serde::Serialize>(api: &str,
                                     endpoint: &str,
                                     json: &S,
                                     timeout_sec: u64,
                                     connect_timeout_sec: u64)
                                     -> Result<(Easy, Arc<Mutex<Vec<u8>>>)> {
    let mut body = Vec::new();
    {
        let mut ser = serde_json::Serializer::new(&mut body);
        json.serialize(&mut ser)?;
    }
    let mut header = List::new();
    header.append("Content-Type: application/json")?;

    let mut req = Easy::new();
    let buf = Arc::new(Mutex::new(Vec::new()));
    {
        let buf = buf.clone();
        req.post(true)?;
        req.post_fields_copy(&body)?;
        req.http_headers(header)?;
        req.url(&format!("{}/{}", api, endpoint))?;
        req.follow_location(true)?;
        req.accept_encoding("")?; // accept all encoding
        req.useragent(concat!(env!("CARGO_PKG_NAME"),
                               "/",
                               env!("CARGO_PKG_VERSION"),
                               " (",
                               env!("CARGO_PKG_HOMEPAGE"),
                               ")"))?;
        req.timeout(Duration::from_secs(timeout_sec))?;
        req.connect_timeout(Duration::from_secs(connect_timeout_sec))?;
        req.write_function(move |data| {
                buf.lock().unwrap().extend_from_slice(data);
                Ok(data.len())
            })?;
    }
    Ok((req, buf))
}

fn parse_error(code: u32, v: &[u8]) -> Error {
    serde_json::from_slice(v)
        .map(|e| ErrorKind::Mojang(code, e).into())
        .unwrap_or_else(|e| e.into())
}

#[derive(Serialize, Debug)]
pub struct Agent<'a> {
    pub name: &'a str,
    pub version: u32,
}

#[derive(OptionConstructor, Serialize, Debug)]
pub struct Authenticate<'a> {
    pub agent: Agent<'a>,
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientToken")]
    pub client_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestUser")]
    pub request_user: Option<bool>,
}

impl<'a> Authenticate<'a> {
    pub fn send_with_api(self,
                         session: Session,
                         api: &str)
                         -> impl Future<Item = objects::Authenticate, Error = Error> + 'a {
        futures::future::result(make_request(api, "authenticate", &self, 10, 10))
            .and_then(move |(req, resp_body)| {
                session.perform(req).map_err(|e| e.into()).map(move |resp| (resp, resp_body))
            })
            .and_then(|(mut resp, resp_body)| {
                futures::future::result(resp.response_code())
                    .map_err(|e| e.into())
                    .map(move |resp_code| (resp_code, resp_body))
            })
            .and_then(|(resp_code, resp_body)| if resp_code == 200 {
                Ok(resp_body)
            } else {
                Err(parse_error(resp_code, &resp_body.lock().unwrap()))
            })
            .and_then(|resp_body| {
                serde_json::from_slice(&resp_body.lock().unwrap()).map_err(|e| e.into())
            })
    }

    pub fn send(self,
                session: Session)
                -> impl Future<Item = objects::Authenticate, Error = Error> + 'a {
        self.send_with_api(session, ::API)
    }
}

#[derive(OptionConstructor, Serialize, Debug)]
pub struct Refresh {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectedProfile")]
    pub selected_profile: Option<objects::Profile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestUser")]
    pub request_user: Option<bool>,
}

impl Refresh {
    pub fn send_with_api<'a>(self,
                             session: Session,
                             api: &str)
                             -> impl Future<Item = objects::Refresh, Error = Error> + 'a {
        futures::future::result(make_request(api, "refresh", &self, 10, 10))
            .and_then(move |(req, resp_body)| {
                session.perform(req).map_err(|e| e.into()).map(move |resp| (resp, resp_body))
            })
            .and_then(|(mut resp, resp_body)| {
                futures::future::result(resp.response_code())
                    .map_err(|e| e.into())
                    .map(move |resp_code| (resp_code, resp_body))
            })
            .and_then(|(resp_code, resp_body)| if resp_code == 200 {
                Ok(resp_body)
            } else {
                Err(parse_error(resp_code, &resp_body.lock().unwrap()))
            })
            .and_then(|resp_body| {
                serde_json::from_slice(&resp_body.lock().unwrap()).map_err(|e| e.into())
            })
    }

    pub fn send<'a>(self,
                    session: Session)
                    -> impl Future<Item = objects::Refresh, Error = Error> + 'a {
        self.send_with_api(session, ::API)
    }
}

#[derive(OptionConstructor, Serialize, Debug)]
pub struct Validate {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
}

impl Validate {
    pub fn send_with_api<'a>(self,
                             session: Session,
                             api: &str)
                             -> impl Future<Item = (), Error = Error> + 'a {
        futures::future::result(make_request(api, "validate", &self, 10, 10))
            .and_then(move |(req, resp_body)| {
                session.perform(req).map_err(|e| e.into()).map(move |resp| (resp, resp_body))
            })
            .and_then(|(mut resp, resp_body)| {
                futures::future::result(resp.response_code())
                    .map_err(|e| e.into())
                    .map(move |resp_code| (resp_code, resp_body))
            })
            .and_then(|(resp_code, resp_body)| if resp_code == 204 {
                Ok(())
            } else {
                Err(parse_error(resp_code, &resp_body.lock().unwrap()))
            })
    }

    pub fn send<'a>(self, session: Session) -> impl Future<Item = (), Error = Error> + 'a {
        self.send_with_api(session, ::API)
    }
}

#[derive(OptionConstructor, Serialize, Debug)]
pub struct Signout {
    pub username: String,
    pub password: String,
}

impl Signout {
    pub fn send_with_api<'a>(self,
                             session: Session,
                             api: &str)
                             -> impl Future<Item = (), Error = Error> + 'a {
        futures::future::result(make_request(api, "signout", &self, 10, 10))
            .and_then(move |(req, resp_body)| {
                session.perform(req).map_err(|e| e.into()).map(move |resp| (resp, resp_body))
            })
            .and_then(|(mut resp, resp_body)| {
                futures::future::result(resp.response_code())
                    .map_err(|e| e.into())
                    .map(move |resp_code| (resp_code, resp_body))
            })
            .and_then(|(resp_code, resp_body)| if resp_code == 204 {
                Ok(())
            } else {
                Err(parse_error(resp_code, &resp_body.lock().unwrap()))
            })
    }

    pub fn send<'a>(self, session: Session) -> impl Future<Item = (), Error = Error> + 'a {
        self.send_with_api(session, ::API)
    }
}


#[derive(OptionConstructor, Serialize, Debug)]
pub struct Invalidate {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
}

impl Invalidate {
    pub fn send_with_api<'a>(self,
                             session: Session,
                             api: &str)
                             -> impl Future<Item = (), Error = Error> + 'a {
        futures::future::result(make_request(api, "invalidate", &self, 10, 10))
            .and_then(move |(req, resp_body)| {
                session.perform(req).map_err(|e| e.into()).map(move |resp| (resp, resp_body))
            })
            .and_then(|(mut resp, resp_body)| {
                futures::future::result(resp.response_code())
                    .map_err(|e| e.into())
                    .map(move |resp_code| (resp_code, resp_body))
            })
            .and_then(|(resp_code, resp_body)| if resp_code == 204 {
                Ok(())
            } else {
                Err(parse_error(resp_code, &resp_body.lock().unwrap()))
            })
    }

    pub fn send<'a>(self, session: Session) -> impl Future<Item = (), Error = Error> + 'a {
        self.send_with_api(session, ::API)
    }
}
