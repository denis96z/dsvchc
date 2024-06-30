#[macro_use]
extern crate dlopen_derive;

mod config;
mod plugins;
mod server;

use clap::Parser;
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: String,
}

fn main() {
    let args = Args::parse();
    let conf = config::Config::load(&args.config).unwrap();

    let mut p = plugins::Plugins::new();
    for (k, v) in &conf.plugins {
        p.load(k, &v.path, &v.conf_path).unwrap();
        for (kk, vv) in &v.checks {
            p.add_check(k, kk, &vv.conf_path).unwrap();
        }
    }

    let srv_conf = server::ServerConfig {
        port: conf.server.port,
    };

    let mut srv = server::Server::new(&srv_conf, &mut p);
    srv.run(move || {
        Signals::new([SIGINT, SIGTERM]).unwrap().forever().next();
    })
    .unwrap();

    p.release().unwrap();
}
