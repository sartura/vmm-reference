// Copyright 2020 Sartura All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;

/// A wrapper structure that provides basic read functionality backed by a unix
/// socket implementation.
pub struct SocketReaderWrapper(pub UnixStream);

impl SocketReaderWrapper {
    pub fn new(buf: String, socket: Option<UnixStream>) -> std::io::Result<SocketReaderWrapper> {
        match socket {
            Some(x) => Ok(SocketReaderWrapper(x)),
            None => match UnixStream::connect(buf) {
                        Ok(x) => return Ok(SocketReaderWrapper(x)),
                        Err(e) => return Err(e)
                    },
        }

    }
}

impl Read for SocketReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsRawFd for SocketReaderWrapper {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

/// A wrapper structure that provides basic write functionality backed by a unix
/// socket implementation.
pub struct SocketWriterWrapper(pub UnixStream);

impl SocketWriterWrapper {
    pub fn new(buf: String, socket: Option<UnixStream>) -> std::io::Result<SocketWriterWrapper> {
        match socket {
            Some(x) => Ok(SocketWriterWrapper(x)),
            None => match UnixStream::connect(buf) {
                        Ok(x) => return Ok(SocketWriterWrapper(x)),
                        Err(e) => return Err(e)
                    },
        }

    }
}

impl Write for SocketWriterWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
