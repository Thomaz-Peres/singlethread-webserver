# Building a Single-Threaded Web Server

In this project, I will following the [Rust docs](https://doc.rust-lang.org/book/ch20-01-single-threaded.html).


Here I'll write somethings I thing is useful/good/important, whatever

_______________________________________________________________________________________________________________

We've choose the port `7878` for two reasons: HTTP isn't normally accepted on this port so our server in unlikely to conflict any other web server you might have running on your machine, and 7878 is `rust` typed on a telephone.


Sometimes, you'll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the *favicon.ico* icon that appears in the browser tab.

When `stream` goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation.

