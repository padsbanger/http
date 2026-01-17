use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

use chrono::prelude::*;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
}

impl Response {
    pub fn new(
        status_code: StatusCode,
        body: Option<Vec<u8>>,
        content_type: Option<String>,
    ) -> Self {
        Response {
            status_code,
            body,
            content_type,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        // Status line
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.status_code as u16,
            self.status_code.reason_phase(),
        )?;

        write!(
            stream,
            "Date: {}\r\n",
            Utc::now().format("%a, %d %b %Y %H:%M:%S GMT")
        )?;

        if let Some(ct) = &self.content_type {
            write!(stream, "Content-Type: {}\r\n", ct)?;
        } else if self.body.is_some() {
            write!(stream, "Content-Type: application/octet-stream\r\n")?;
        }

        if let Some(body) = &self.body {
            write!(stream, "Content-Length: {}\r\n", body.len())?;
        } else {
            write!(stream, "Content-Length: 0\r\n")?;
        }

        write!(stream, "Connection: keep-alive\r\n\r\n")?;
        write!(stream, "Server: MyRustServer\r\n\r\n")?;

        write!(stream, "\r\n")?;

        if let Some(body) = &self.body {
            stream.write_all(body)?;
        }

        Ok(())
    }
}
