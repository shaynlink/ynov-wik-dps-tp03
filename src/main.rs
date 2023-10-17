use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    thread,
};
use gethostname::gethostname;

fn main() {
    // Result<T, E>
    let port = match env::var("PING_LISTEN_PORT") {
        Ok(port) => port,
        Err(_) => "8080".to_string(),
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request[0].clone();
    let header_lines: Vec<_> = http_request[1..].to_vec();

    let mut headers = HashMap::new();

    for header_line in header_lines {
        let header_parts: Vec<_> = header_line.split(": ").collect();
        headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
    }

    // Get http version
    let http_version = request_line.split(" ").collect::<Vec<_>>()[2];

    if http_version != "HTTP/1.1" {
        let contents = "";
        let length = contents.len();

        let response = format!("HTTP/1.1 505 HTTP Version Not Supported\r\nContent-Length: {length}\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
        return;
    } 

    if request_line == "GET /ping HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = hashmap_to_json(&headers);
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // Get computer hostname
       println!("{:?}", gethostname());

        stream.write_all(response.as_bytes()).unwrap()
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = "";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap()
    }
}

fn hashmap_to_json(map: &HashMap<String, String>) -> String {
    let mut json_string = String::from("{");

    let mut first = true;
    for (key, value) in map {
        if !first {
            json_string.push_str(",");
        } else {
            first = false;
        }

        json_string.push_str(format!("\"{}\":\"{}\"", key, value).as_str());
    }

    json_string.push_str("}");

    json_string
}