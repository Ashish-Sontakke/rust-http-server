use std::{str::FromStr};
use super::request::ParseError;

#[derive(Debug)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    CONNECT,
    TRACE,
    PUT,
    POST,
    DELETE,
    PATCH,
}


impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPTIONS" => Ok(Self::OPTIONS),
            "GET" => Ok(Self::GET),
            "HEAD"=> Ok(Self::HEAD),
            "CONNECT"=> Ok(Self::CONNECT),
            "TRACE"=> Ok(Self::TRACE),
            "PUT"=> Ok(Self::PUT),
            "POST"=> Ok(Self::POST),
            "DELETE"=> Ok(Self::DELETE),
            "PATCH"=> Ok(Self::PATCH),
            _ =>  Err(ParseError::InvalidMethod)  
        }
    }
}

