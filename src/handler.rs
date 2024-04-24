use std::{
    fs::read,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{reponse::Response, router::Router, status_code::StatusCode};

// use crate::request::Request;
//

pub fn serve(router: Router, listener: TcpListener) {
    for stream in listener.incoming() {
        println!("Connection established!");
        println!("{:?}", stream);

        let mut stream = stream.unwrap();
        let bytes = stream.try_clone().unwrap().bytes();

        let mut headers = vec![];
        for byte in bytes {
            headers.push(byte.unwrap());
            let len = headers.len();
            if len >= 4 && headers[len - 4..] == [b'\r', b'\n', b'\r', b'\n'] {
                break;
            }
        }

        let headers = String::from_utf8_lossy(&headers)
            .trim()
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let content_length: usize = headers[5][16..].parse().unwrap();
        let mut buffer = vec![0; content_length];
        let _ = stream.read_exact(&mut buffer);
        let data = String::from_utf8_lossy(&buffer);
        dbg!(data);

        let response = Response {
            status: StatusCode::Ok,
            version: "HTTP/1.1".into(),
            content: "Hello".into(),
        };

        stream.write_all(String::from(response).as_bytes()).unwrap();
        println!("Connection finished!");
    }
}


#[cfg(test)]
mod test {
    // use super::find_header_end;

    #[test]
    fn test_find_header_end() {
        // let request = "GET /index.html HTTP/1.1\r\n\
        //            Host: www.example.com\r\n\
        //            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:98.0) Gecko/20100101 Firefox/98.0\r\n\
        //            Accept: text/html\r\n\
        //            Connection: close\r\n\
        //            \r\n".as_bytes();
        // let expected = Some(16);
        // let actual = find_header_end(request);
        // assert_eq!(expected, actual);
    }
    #[test]
    fn test_get_content_length() {
        let headers = [
            "POST / HTTP/1.1",
            "Host: localhost:3000",
            "User-Agent: curl/8.6.0",
            "Accept: */*",
            "Content-Type: application/json",
            "Content-Length: 16",
        ];
        let content_length = &headers[5][16..].parse::<i32>();
        dbg!(content_length);
    }
}
