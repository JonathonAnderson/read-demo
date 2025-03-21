use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let server = thread::spawn(server);
    let client = thread::spawn(client);
  
    server.join().unwrap();
    client.join().unwrap();
}

// hostname and port for the server to bind to
const HOST: &str = "localhost:3333";

/// number of bytes for server to produce and client to read
const TOTAL_BYTES: usize = 1_000_000;

fn client() {
    // connect to server
    let mut stream = loop {
        match TcpStream::connect(HOST) {
            Ok(stream) => break stream,
            Err(_)     => thread::sleep(Duration::from_millis(10)),
        }
    };
    
    // run tests
    client_inner("TcpStream (Read)", &mut stream);
    client_inner("BufReader<TcpStream> (BufRead)", &mut BufReader::new(stream));
}

fn client_inner<TRead: Read>(description: &str, stream: &mut TRead) {
    let mut index = 0;

    // create a large buffer to hold all incoming data
    let mut buffer: Vec<u8> = (0..TOTAL_BYTES).map(|_| 0).collect();

    let start = Instant::now();
    // loop while there are still bytes to be read
    loop {
        // get the next one-byte slice of the buffer to use for reading (the
        // tiny slice is chosen to make this as inefficient as possible, for
        // illustration purposes)
        let buffer_slice = &mut buffer[index..usize::min(index + 1, TOTAL_BYTES)];

        // read new data into the buffer slice
        let received_bytes = stream.read(buffer_slice).unwrap();

        if received_bytes > 0 {
            // advance `index` by the number of bytes read
            index += received_bytes;
        } else {
            // if there are no more bytes to be read, we're done
            break;
        }
    }
    let end = Instant::now();

    println!(
        "{} took {}ms",
        description,
        end.duration_since(start).as_millis()
    );
}

fn server() {
    // create a large set of data [0, 1, .., 255, 0, 1, ..]
    let data: Vec<u8> = (0..TOTAL_BYTES).map(|n| (n % 255) as u8).collect();

    // listen for incoming connections
    let listener = TcpListener::bind(HOST).unwrap();
    let mut stream = match listener.accept() {
        Ok((stream, _)) => stream,
        Err(_) => todo!(),
    };

    let _ = stream.write(&data).unwrap();
    let _ = stream.write(&data).unwrap();
}
