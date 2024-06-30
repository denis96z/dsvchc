use std::sync::Arc;
use std::thread;

use tiny_http::{Header, Method, Response, StatusCode};

use crate::plugins;

pub struct ServerConfig {
    pub port: u16,
}

pub struct Server<'a> {
    conf: &'a ServerConfig,
    plugins: &'a mut plugins::Plugins,
}

impl<'a> Server<'a> {
    pub fn new(conf: &'a ServerConfig, plugins: &'a mut plugins::Plugins) -> Self {
        Self { conf, plugins }
    }

    pub fn run(&mut self, wait: fn()) -> Result<(), String> {
        let srv = Arc::new(tiny_http::Server::http(format!("0.0.0.0:{}", self.conf.port)).unwrap());

        let srvx = srv.clone();
        let awth = thread::spawn(move || {
            wait();
            srvx.unblock();
        });
        for req in srv.incoming_requests() {
            if req.method() != &Method::Get || req.url().split('?').next().unwrap() != "/health" {
                let body = "NOT FOUND".as_bytes();
                req.respond(Response::new(
                    StatusCode(404),
                    Vec::new(),
                    body,
                    Some(body.len()),
                    None,
                ))
                .unwrap();
                continue;
            }

            let s = self.plugins.perform_checks().unwrap();
            let rsp = Response::new(
                StatusCode(200),
                vec![Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()],
                s.as_bytes(),
                Some(s.len()),
                None,
            );

            req.respond(rsp).unwrap();
        }

        awth.join().unwrap();
        Ok(())
    }
}
