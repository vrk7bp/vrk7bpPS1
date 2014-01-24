// Venkata-Gautam Kanumuru (vrk7bp)
//
// zhttpto.rs
//
// Final code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Originally coded by Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitCount: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {

        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
 
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);

            //Gets the contents of the 2nd /, so as to check if it is an empty path or not.
            let splitArray: ~[&str] = request_str.split(' ').collect();
            let path = splitArray[1].slice_from(1).to_owned();
            let testVal = ~"";
            println!("{}", path);

            //Obtain, potentially, the last four characters in the string to check if they are html
            let borrowedPath = path.clone();
            let size = path.len();
            let HTMLTestVal = ~"html";
            let mut isHTML = ~"N/A";
            if(size > 4) {
               isHTML = borrowedPath.slice_from(size-4).to_owned();
            };

            println(format!("Received request :\n{:s}", request_str));
            

            //First check if there is no directories being accessed. If so then show the original "home screen".
            if (std::str::eq(&path, &testVal)) {
                let response: ~str = 
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 </body></html>\r\n";
                stream.write(response.as_bytes());
            }

            //Second, check to see if the last four characters in the path are .html. If so, then there are two possible scenarios...
            else if(std::str::eq(&isHTML, &HTMLTestVal)) {
                let tempPath = path.clone();
                let filePath = Path::new(path);

                //The path/document is valid and the particular file will be shown through the browser.
                if(filePath.exists()) {
                    let mut msg_file = File::open(&filePath);
                    let msg_bytes: ~[u8] = msg_file.read_to_end();
                    stream.write(msg_bytes); 
                } 

                //Otherwise the path/document is not valid and the 403 page will be brought up with the relevant path.
                else {
                    let response: ~str = 
                    format!("HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                    <doctype !html><html><head><title>403 Forbidden</title>
                    <body> <h1> Forbidden </h1>
                    <p> You don't have permission to access /{:s} </p>
                    <hr>
                    </body></html>\r\n", tempPath);
                    stream.write(response.as_bytes());   
                };
            }

            //If none of the above two situations were "met", then show the 403 page with the relevant path.
            else {
                let response: ~str = 
                format!("HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>403 Forbidden</title>
                 <body> <h1> Forbidden </h1>
                 <p> You don't have permission to access /{:s} </p>
                 <hr>
                 </body></html>\r\n", path);
                stream.write(response.as_bytes());
            };

            //In the command prompt, print out "Connection terminates." as well as the new page visited number.
            println!("Connection terminates.");
            unsafe {
                visitCount = visitCount + 1;
                println!("This page has been called {} time(s).", visitCount);
            }
        }
    }
}
