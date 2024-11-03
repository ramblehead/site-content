use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

struct Socket;

impl Socket {
    fn new() -> Self {
        println!("Creating Socket");
        Socket
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        println!("Closing Socket");
    }
}

struct Connection {
    socket: Arc<Socket>,
}

impl Connection {
    fn new(socket: Arc<Socket>) -> Result<Self, String> {
        println!("Creating Connection using Socket");
        // Uncomment the next line to simulate an error
        // return Err("Connection constructor failed".into());
        Ok(Connection { socket })
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("Closing Connection");
    }
}

struct Api {
    socket: Arc<Socket>,
    connection: Connection,
}

impl Api {
    fn new() -> Result<Self, String> {
        let socket = Arc::new(Socket::new());
        let connection = Connection::new(Arc::clone(&socket))?;
        Ok(Api { socket, connection })
    }

    fn get_instance() -> &'static Mutex<Api> {
        static INSTANCE: Lazy<Mutex<Api>> = Lazy::new(|| {
            Mutex::new(Api::new().expect("Failed to create Api instance"))
        });
        &INSTANCE
    }
}

fn main() {
    match Api::get_instance().lock() {
        Ok(api) => println!("Hi there! API instance is ready."),
        Err(e) => eprintln!("Failed to lock the Api instance: {:?}", e),
    }
}
