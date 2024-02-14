use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


// In this function we'll read data from TCP stream and print it so we can see the data being sent from the browser.

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}