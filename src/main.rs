fn main() {
    let server = Server::new("http://127.0.0.1:8080".to_string());
    server.run();
}


struct Server {
    addr: String,
}


impl  Server {

    fn new(addr: String) -> Server {
        return Server {
            addr
        };
    }


    fn run(self) {}
}

