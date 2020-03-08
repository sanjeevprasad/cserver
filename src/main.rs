use futures::executor::block_on;
use nix::sys::stat;
use nix::unistd;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Shutdown, TcpListener};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

fn main() {
  block_on(async {
    println!("=================================================\n");
    let basepath = env::current_dir().unwrap().join(r"tcppipes");
    fs::remove_dir_all(&basepath).unwrap();
    fs::create_dir(&basepath).unwrap();
    let listener = TcpListener::bind("0.0.0.0:4444").unwrap();
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          let peer_addr = stream.peer_addr().unwrap();
          let ip_port = peer_addr.ip().to_string() + ":" + &peer_addr.port().to_string();
          let fifo_tx = basepath.join(ip_port.clone() + r".tx");
          let fifo_rx = basepath.join(ip_port.clone() + r".rx");
          println!("Received a connection! - {:?}", ip_port);
          let mut buf = BufReader::new(stream.try_clone().unwrap());
          unistd::mkfifo(&fifo_tx, stat::Mode::S_IRWXU).unwrap();
          unistd::mkfifo(&fifo_rx, stat::Mode::S_IRWXU).unwrap();
          let (tx, rx) = mpsc::channel::<usize>();
          thread::spawn(move || {
            block_on(async {
              loop {
                //let mut s = [0u8; 4096];
                let mut s = String::new();
                // let keeprunning = tx::
                let keeprunning = true;
                println!("keeprunning {}", keeprunning);
                match rx.try_recv() {
                  Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                  }
                  Err(TryRecvError::Empty) => {}
                }
                match buf.read_line(&mut s) {
                  Ok(b) => {
                    if b == 0 {
                      break;
                    }
                    print!("Received data ({} bytes): {}", b, s);
                    fs::write(&fifo_rx, s).unwrap();
                  }
                  Err(e) => println!("Error receiving data! - {}", e),
                }
              }
            });
            println!("Client {} dropped", ip_port);
            fs::remove_file(&fifo_tx).unwrap();
            fs::remove_file(&fifo_rx).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
          });
          thread::sleep(std::time::Duration::from_secs(5));
          tx.send(0).unwrap();
        }
        Err(e) => println!("Error! - {}", e),
      }
    }
    drop(listener);
  });
}
