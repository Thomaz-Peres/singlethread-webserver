use std::net::TcpListener;

fn main() {

    // This code will listen at the local address 127.0.0.1:7878 for incoming TCP streams.
    // When it gets an incoming stream, it will print "Connections established!".
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // bind works like the new function, returning a new TcpListener instance.

    // The incoming method on TcpLinterner return an iterator that gives us a sequence of streams (more specifically, streams of type TcpStream).
    // A single stream represents an open connection between the client and the server.
    // The server generates a response, and the server closes the connection.
    // As such, we will read from the TcpStream to see what the client sent and then write our response to the stream to send data back to the client.
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connections established!");
    }
}
