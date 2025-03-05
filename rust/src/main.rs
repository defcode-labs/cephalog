mod server;
mod routes;
mod handlers;
mod models;
mod middleware;

use server::server::start;


fn main() {
    start();
}
