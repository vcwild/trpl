extern crate communicator;

pub mod client;
pub mod network;

fn main() {
    communicator::network::server::connect();
    communicator::network::connect();
    communicator::client::connect();
    communicator::network::foo::connect();
    communicator::network::foo::server::connect();
}
