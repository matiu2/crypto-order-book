mod model;
pub mod api {
    tonic::include_proto!("orderbook");
}
fn main() {
    println!("Hello, world!");
}
