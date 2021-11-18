// Copyright 2020 Sartura All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use crate::legacy::pipe::{PipeReaderWrapper, PipeWriterWrapper};
use crate::legacy::socket::{SocketReaderWrapper, SocketWriterWrapper};
use crate::legacy::tcp::{TCPReaderWrapper, TCPWriterWrapper};
use std::io::{stdin, stdout, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};


/// Enum that determines the console type
#[derive(Clone, Debug, PartialEq)]
pub enum ConsoleType {
    /// Default console type (STDIN/STDOUT)
    Standard,
    /// Named pipe
    Pipe,
    /// Unix socket
    Socket,
    /// TCP socket
    TCP
}

pub struct ConsoleWrapper {
    pub reader: ConsoleReaderWrapper,
    pub writer: ConsoleWriterWrapper,
}

impl ConsoleWrapper {
    pub fn new(input: Option<String>, output: Option<String>, console_type: ConsoleType) -> ConsoleWrapper {
        match console_type {
            ConsoleType::Standard => { 
                let reader = ConsoleReaderWrapper::new(None, None, None, ConsoleType::Standard);
                let writer = ConsoleWriterWrapper::new(None, None, None, ConsoleType::Standard);
                ConsoleWrapper { reader, writer }
            },
            ConsoleType::Pipe => {
                let reader = match input {
                    None => ConsoleReaderWrapper::new(None, None, None, ConsoleType::Standard),
                    Some(x) => ConsoleReaderWrapper::new(Some(PipeReaderWrapper::new(x).unwrap()), None, None, console_type.clone()),
                };
                let writer = match output {
                    None => ConsoleWriterWrapper::new(None, None, None, ConsoleType::Standard),
                    Some(x) => ConsoleWriterWrapper::new(Some(PipeWriterWrapper::new(x).unwrap()), None, None, console_type.clone()),
                };
                ConsoleWrapper { reader, writer }
            }
            ConsoleType::Socket => {
                match (input, output) {
                    (Some(x), Some(y)) => {
                        let unix = SocketWriterWrapper::new(y, None).unwrap();
                        let socket = unix.0.try_clone().unwrap();
                        let reader = ConsoleReaderWrapper::new(None, Some(SocketReaderWrapper::new(x, Some(socket)).unwrap()), None, console_type.clone());
                        let writer = ConsoleWriterWrapper::new(None, Some(unix), None, console_type.clone());
                        ConsoleWrapper { reader, writer }
                    },
                    (_, _) => {
                        let reader = ConsoleReaderWrapper::new(None, None, None, ConsoleType::Standard);
                        let writer = ConsoleWriterWrapper::new(None, None, None, ConsoleType::Standard);
                        ConsoleWrapper { reader, writer }
                    }
                }
            }
            ConsoleType::TCP => {
                match (input, output) {
                    (Some(x), Some(y)) => {
                        let tcp = TCPReaderWrapper::new(x, None).unwrap();
                        let socket = tcp.0.try_clone().unwrap();
                        let reader = ConsoleReaderWrapper::new(None, None, Some(tcp), console_type.clone());
                        let writer = ConsoleWriterWrapper::new(None, None, Some(TCPWriterWrapper::new(y, Some(socket)).unwrap()), console_type.clone());
                        ConsoleWrapper { reader, writer }
                    },
                    (_, _) => {
                        let reader = ConsoleReaderWrapper::new(None, None, None, ConsoleType::Standard);
                        let writer = ConsoleWriterWrapper::new(None, None, None, ConsoleType::Standard);
                        ConsoleWrapper { reader, writer }
                    }
                }
                
            }

        }
    }
}


/// A generic type used for wrapping the guest input implementation
/// (stdin, named pipe, unix socket etc.)
pub struct ConsoleReaderWrapper {
    pipe_reader: Option<PipeReaderWrapper>,
    socket_reader: Option<SocketReaderWrapper>,
    tcp_reader: Option<TCPReaderWrapper>,
    console_type: ConsoleType
}

impl ConsoleReaderWrapper {
    pub fn new(pipe_reader: Option<PipeReaderWrapper>, socket_reader: Option<SocketReaderWrapper>, tcp_reader: Option<TCPReaderWrapper>, console_type: ConsoleType) -> ConsoleReaderWrapper {
        match console_type {
            ConsoleType::Standard => ConsoleReaderWrapper { pipe_reader: None, socket_reader: None, tcp_reader: None, console_type: ConsoleType::Standard },
            ConsoleType::Pipe => match pipe_reader {
                None => ConsoleReaderWrapper { pipe_reader: None, socket_reader: None, tcp_reader: None, console_type: ConsoleType::Standard },
                Some(x) => ConsoleReaderWrapper {
                    pipe_reader: Some(x),
                    socket_reader: None,
                    tcp_reader: None,
                    console_type: ConsoleType::Pipe,
                }
            },
            ConsoleType::Socket => match socket_reader {
                None => ConsoleReaderWrapper { pipe_reader: None, socket_reader: None, tcp_reader: None, console_type: ConsoleType::Standard },
                Some(x) => ConsoleReaderWrapper {
                    pipe_reader: None,
                    socket_reader: Some(x),
                    tcp_reader: None,
                    console_type: ConsoleType::Socket,
                }
            },
            ConsoleType::TCP => match tcp_reader {
                None => ConsoleReaderWrapper { pipe_reader: None, socket_reader: None, tcp_reader: None, console_type: ConsoleType::Standard },
                Some(x) => ConsoleReaderWrapper {
                    pipe_reader: None,
                    socket_reader: None,
                    tcp_reader: Some(x),
                    console_type: ConsoleType::TCP,
                }
            },
        }
    }
}

impl Read for ConsoleReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.console_type {
            ConsoleType::Standard => stdin().read(buf),
            ConsoleType::Pipe => match self.pipe_reader.as_mut() {
                None => stdin().read(buf),
                Some(x) => x.read(buf),
            },
            ConsoleType::Socket => match self.socket_reader.as_mut() {
                None => stdin().read(buf),
                Some(x) => x.read(buf),
            },
            ConsoleType::TCP => match self.tcp_reader.as_mut() {
                None => stdin().read(buf),
                Some(x) => x.read(buf),
            },
        }
    }
}

impl AsRawFd for ConsoleReaderWrapper {
    fn as_raw_fd(&self) -> RawFd {
        match self.console_type {
            ConsoleType::Standard => stdin().as_raw_fd(),
            ConsoleType::Pipe => match &self.pipe_reader {
                None => stdin().as_raw_fd(),
                Some(x) => x.as_raw_fd(),
            },
            ConsoleType::Socket => match &self.socket_reader {
                None => stdin().as_raw_fd(),
                Some(x) => x.as_raw_fd(),
            },
            ConsoleType::TCP => match &self.tcp_reader {
                None => stdin().as_raw_fd(),
                Some(x) => x.as_raw_fd(),
            },
        }
    }
}

/// A generic type used for wrapping the guest output implementation
/// (stdout, named pipe, unix socket etc.)
pub struct ConsoleWriterWrapper {
    writer: Box<dyn Write + Send>,
    console_type: ConsoleType
}

impl ConsoleWriterWrapper {
    pub fn new(pipe_writer: Option<PipeWriterWrapper>, socket_writer: Option<SocketWriterWrapper>, tcp_writer: Option<TCPWriterWrapper>, console_type: ConsoleType) -> ConsoleWriterWrapper {
        let mut writer_type = ConsoleType::Standard;
        let writer_obj: Box<dyn Write + Send> = match console_type {
            ConsoleType::Standard => Box::new(stdout()),
            ConsoleType::Pipe => match pipe_writer {
                None => Box::new(stdout()),
                Some(x) => {
                    writer_type = ConsoleType::Pipe;
                    Box::new(x)
                },
            },
            ConsoleType::Socket => match socket_writer {
                None => Box::new(stdout()),
                Some(x) => {
                    writer_type = ConsoleType::Socket;
                    Box::new(x)
                },
            },
            ConsoleType::TCP => match tcp_writer {
                None => Box::new(stdout()),
                Some(x) => {
                    writer_type = ConsoleType::TCP;
                    Box::new(x)
                },
            },
        };
        ConsoleWriterWrapper { writer: writer_obj, console_type: writer_type}
    }
}

impl Write for ConsoleWriterWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}