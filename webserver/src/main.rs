use std::net::TcpListener;
use webserver::utils;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            utils::handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
