extern crate warp;

use warp::Filter;

fn main() {
    let routes = warp::any().map(|| "Hello, World!");
    warp::serve(routes).run(([127, 0, 0, 1], 8888));
}