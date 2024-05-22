use crate::any::macros;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;

const TWITCH_IRC: &'static str = "irc.chat.twitch.tv:6667";
const REQUEST_DELAY: u64 = 1000;

pub(crate) struct TwitchHandler {
  socket: TcpStream,
  channel: String,
  username: String,
  handle: Option<thread::JoinHandle<()>>,
  sender: Option<Sender<String>>,
}

impl TwitchHandler {
  pub(crate) fn new(channel: String, username: String) -> Self {
    let socket = TcpStream::connect(TWITCH_IRC);
    if let Err(e) = socket {
      macros::exit_err!("There was a failure trying to connect to Twitch: {}", e);
    }

    Self {
      socket: socket.unwrap(),
      channel: channel,
      username: username,
      handle: None,
      sender: None,
    }
  }

  pub(crate) fn login(&mut self, token: String) -> &Self {
    self.send_raw(format!("PASS: {}", token));
    self.send_raw(format!("JOIN: #{}", self.channel));
    self.send_raw(format!("NICK: {}", self.username));

    macros::log_ok!("Successfully authenticated and joined {}", self.channel);

    self
  }

  pub(crate) fn send_raw(&mut self, data: String) -> &Self {
    socket_send_raw(&mut self.socket, data);
    self
  }

  pub(crate) fn send(&mut self, data: String) -> &Self {
    let data = format!("PRIVMSG #{} :{}", self.channel, data);
    socket_send_raw(&mut self.socket, data);
    self
  }

  pub(crate) fn get_reader(&self) -> BufReader<&TcpStream> {
    BufReader::new(&self.socket)
  }
}

fn socket_send_raw(socket: &mut TcpStream, data: String) -> () {
  let data = format!("{data}\r\n");
  let write = socket.write(data.as_bytes());
  if let Err(e) = write {
    macros::log_err!("Raw send failed: {}", e);
  }
}
