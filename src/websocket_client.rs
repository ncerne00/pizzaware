use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tungstenite::connect;
pub struct WebSocketClient {
    server_url: String,
    message_handler: Arc<Mutex<Box<dyn Fn(String) + Send>>>,
    running: Arc<Mutex<bool>>
}

impl WebSocketClient {
    pub fn new<F>(server_url: &str,  message_handler: F) -> Self 
    where
        F: Fn(String) + Send + 'static,
    {
        WebSocketClient {
            server_url: server_url.to_string(),
            message_handler: Arc::new(Mutex::new(Box::new(message_handler))),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let server_url = self.server_url.clone();
        let message_handler = Arc::clone(&self.message_handler);
        let running = Arc::clone(&self.running);

        *running.lock().unwrap() = true;

        thread::spawn(move || {
            let (mut socket, _response) = match connect(server_url) {
                Ok((socket, response)) => (socket, response),
                Err(err) => {
                    panic!("Error: unable to establish socket connection: {err}")
                }
            };

                
            while *running.lock().unwrap() {
                match socket.read() {
                    Ok(message) => {
                        if let Ok(message_text) = message.to_text() {
                            // Create a new owned String from the message text
                            let text = String::from(message_text);
                            
                            // Now acquire the lock and call the handler
                            let handler = message_handler.lock().unwrap();
                            handler(text);
                        }
                    },
                    Err(err) => {
                        eprintln!("Error reading message: {}", err);
                        // Handle the error as needed
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        })
    }
}