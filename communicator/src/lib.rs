pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    use super::network;


    #[test]
    fn it_works() {
        client::connect();
        network::connect();
        network::server::connect();

    }
}
