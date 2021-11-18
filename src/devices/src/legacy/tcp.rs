// Copyright 2020 Sartura All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::net::TcpStream;

/// A wrapper structure that provides basic read functionality backed by a TCP
/// socket implementation.
pub struct TCPReaderWrapper(pub TcpStream);

impl TCPReaderWrapper {
    pub fn new(buf: String, socket: Option<TcpStream>) -> std::io::Result<TCPReaderWrapper> {
        match socket {
            Some(x) => Ok(TCPReaderWrapper(x)),
            None => match TcpStream::connect(buf) {
                        Ok(x) => return Ok(TCPReaderWrapper(x)),
                        Err(e) => return Err(e)
                    },
        }

    }
}

impl Read for TCPReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsRawFd for TCPReaderWrapper {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

/// A wrapper structure that provides basic write functionality backed by a TCP
/// socket implementation.
pub struct TCPWriterWrapper(pub TcpStream);

impl TCPWriterWrapper {
    pub fn new(buf: String, socket: Option<TcpStream>) -> std::io::Result<TCPWriterWrapper> {
        match socket {
            Some(x) => Ok(TCPWriterWrapper(x)),
            None => match TcpStream::connect(buf) {
                        Ok(x) => return Ok(TCPWriterWrapper(x)),
                        Err(e) => return Err(e)
                    },
        }

    }
}

impl Write for TCPWriterWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
