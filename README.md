# Building a Single-Threaded Web Server

In this project, I will following the [Rust docs](https://doc.rust-lang.org/book/ch20-01-single-threaded.html).


Here I'll write somethings I thing is useful/good/important, whatever

_______________________________________________________________________________________________________________

We've choose the port `7878` for two reasons: HTTP isn't normally accepted on this port so our server in unlikely to conflict any other web server you might have running on your machine, and 7878 is `rust` typed on a telephone.


Sometimes, you'll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the *favicon.ico* icon that appears in the browser tab.

When `stream` goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation.

## Reading the request (not is the final)

Receiving the result, looks like this:

(Depending on your browser, you might get slightly different output)

```
Request: [
    "GET / HTTP/1.1",
    "Host: localhost:7878",
    "Connection: keep-alive",
    "Cache-Control: max-age=0",
    "sec-ch-ua: \"Not A(Brand\";v=\"99\", \"Microsoft Edge\";v=\"121\", \"Chromium\";v=\"121\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br",
    "Accept-Language: pt-BR,pt;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
]
```

## A Closer Look at an HTTP Request

HTTP is a text-based protocol, and a request takes this format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the request line that holds information about what the client is requesting. The first part of the request line indicates the method being used, such as GET or POST, which describes how the client is making this request. Our client used a GET request, which means it is asking for information.

The next part of the request line is /, which indicates the Uniform Resource Identifier (URI) the client is requesting: a URI is almost, but not quite, the same as a Uniform Resource Locator (URL). The difference between URIs and URLs isn’t important for our purposes in this chapter, but the HTTP spec uses the term URI, so we can just mentally substitute URL for URI here.

The last part is the HTTP version the client uses, and then the request line ends in a CRLF sequence. (CRLF stands for carriage return and line feed, which are terms from the typewriter days!) The CRLF sequence can also be written as \r\n, where \r is a carriage return and \n is a line feed. The CRLF sequence separates the request line from the rest of the request data. Note that when the CRLF is printed, we see a new line start rather than \r\n.

Looking at the request line data we received from running our program so far, we see that GET is the method, / is the request URI, and HTTP/1.1 is the version.

Try making a request from a different browser or asking for a different address, such as 127.0.0.1:7878/test, to see how the request data changes.

## Writing a response.

Responses have the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

reason phrase provides a text description of the status code. After the CRLF sequence are any headers, another CRLF sequence, and the body of the response.

Here is an example response that uses HTTP version 1.1, has a status code of 200, an OK reason phrase, no headers, and no body:

```
HTTP/1.1 200 OK\r\n\r\n
```


# I will start the second part (I should have done that a long time ago)

The single thread server just process one request in turn, meaning it won't process a second connection until the first is finished processing.

##### Simulating a Slow request in the Current server implementation.

We will switch from `if` to `match` now that we have three cases. We need to explicitly match on a slice of `request_line` to pattern match against the string literal values; `match` doesn't do automatic referencing and dereferencing like the equality method does.

There are multiple techniques we could use to avoid requests backing up behind a slow request; the one we'll implement is a thread pool.

### Improving a Throughput with a Thread Pool

We'll limit the number of threads in the pool to a small number to protect us from Denial of Service (DoS) attacks.

Request that come in are sent to the pool for processing. The pool will maintain a queue of incoming requests.

Other options to improve the throughout of a web server are the *fork/join model*, the *single-threaded async I/O model*, or the *multi-threaded async I/O model*.

### Spawning a Thread for Each Request

Making `Main` spawn a new thread to handle each stream within the `for` loop.

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
} 
```

## Creating a Finite Number of Threads

Use a hypothetical interface for a `ThreadPool` struct we want to use instead of `thread::spawn`.

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
} 
```

We need to implement `pool.execute`. This code won't yet compile

## Building ThreadPool Using Compiler Driven Development

With you run `cargo check`, will get an error and this error tells us we need a `ThreadPool` type or module

Our `ThreadPool` will be independent of the kind of work our web server is doing.

Lets switch the project from binary crate to a library crate. Creating the file `src/lib.rs`

We use `usize` as the type of the `size` parameter, because we know that a negative number of thread doesn't make any sense.

We'll define the `execute` method on `ThreadPool` to take a closure as a parameter, then the function it takes the closure it's given and gives it to an idle thread in the pool to run.

In the `execute` method, we use `FnOnce()` because the thread for running a request will only execute that requet's closure one time.

The `F` type parameter also has the trait bound `Send` and the lifetime bound `'static`.

We need `Send` to transfer the closure from one thread to another and `static` because we don't know how long the thread will take to execute.

And the `FnOnce` use  the `()` because represents a closure that takes no paramters and returns the unit type `()`.

Just like functions definitions, the return type can be omitted from the signature, but even if we have no parameters, we still need the parentheses.

## Validatin the Number of Threads in new