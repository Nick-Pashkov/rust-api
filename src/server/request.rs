use std::str::FromStr;

pub struct Request {
    method: RequestMethods,
    path: String,
    body: String,
}

enum Errors {
    MethodUnsupported { details: String },
}

#[derive(Debug)]
enum RequestMethods {
    GET, POST, PUT, DELETE, PATCH
}

impl FromStr for RequestMethods {
    type Err = Errors;

    fn from_str(s: &str) -> Result<RequestMethods, Errors> {
        match s {
            "GET" => Ok(RequestMethods::GET),
            "POST" => Ok(RequestMethods::POST),
            "PUT" => Ok(RequestMethods::PUT),
            "DELETE" => Ok(RequestMethods::DELETE),
            "PATCH" => Ok(RequestMethods::PATCH),
            _ => Err(Errors::MethodUnsupported { details: String::from("Method unsupported") })
        }
    }
}

impl Request {
    pub fn new(input: String) -> Request {

        let input: Vec<&str> = input.lines().collect();
        
        // Request line
        let line: Vec<&str> = input[0].split_whitespace().collect();
        
        let method = line[0].parse::<RequestMethods>();
        match method {
            Ok(m) => m,
            Err(e) => println!("Invalid")
        };

        let path = String::from(line[1]);
        
        let body = String::default();

        println!("{:?}", input);

        Request { method, path, body }
    }
}