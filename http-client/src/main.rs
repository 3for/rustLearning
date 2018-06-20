extern crate hyper;

use hyper::Client;
use hyper::rt::{self, lazy, Future, Stream};

const DEFAULT_URL: &'static str = "http://127.0.0.1:3000";

fn main() {
    // A runtime is needed to execute our asynchronous code.
    rt::run(lazy(|| {
        let client = Client::new();

        client
            // Make a GET /ip to 'http://httpbin.org'
            //.get("http://httpbin.org/ip".parse().unwrap())
            .get(DEFAULT_URL.parse().unwrap())
            // And then, if the request gets a response...
            .and_then(|res| {
                println!("status: {}", res.status());

                // Concatenate the body stream into a single buffer...
                // This returns a new future, since we must stream body.
                res.into_body().concat2()
            })

            // And then, if reading the full body succeeds...
            .and_then(|body| {
                // The body is just bytes, but let's print a string...
                let s = ::std::str::from_utf8(&body)
                    .expect("httpbin sends utf-8 JSON");

                println!("body: {}", s);

                // and_then requires we return a new Future, and it turns
                // out that Result is a Future that is ready immediately.
                Ok(())
            })

            // Map any errors that might have happened...
            .map_err(|err| {
                println!("error: {}", err);
            })
    }));

}
