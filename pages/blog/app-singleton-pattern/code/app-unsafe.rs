use std::rc::Rc;

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
    socket: Rc<Socket>,
}

impl Connection {
    fn new(socket: Rc<Socket>) -> Result<Self, String> {
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
    socket: Rc<Socket>,
    connection: Connection,
}

impl Api {
    fn new() -> Result<Self, String> {
        let socket = Rc::new(Socket::new());
        let connection = Connection::new(Rc::clone(&socket))?;
        Ok(Api { socket, connection })
    }

    fn get_instance() -> &'static Api {
        static mut INSTANCE: Option<Api> = None;

        // Use unsafe block to initialize the instance only once
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Api::new().expect("Failed to create Api instance"));
            }
            INSTANCE.as_ref().unwrap()
        }
    }
}

fn main() {
    let api = Api::get_instance();
    println!("Hi there! API instance is ready.");
}
