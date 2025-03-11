mod server;
mod routes;
mod handlers;
mod models;
mod middleware;

extern crate db;

use server::server::start;


fn main() {
    start();
}
