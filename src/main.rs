extern crate alsa;

use std::error::Error;

fn run() -> Result<(), Box<Error>> {
  let sequencer = alsa::Seq::open(None, None, true)?;
  let clients = alsa::seq::ClientIter::new(&sequencer);
  for client in clients {
    let ports = alsa::seq::PortIter::new(&sequencer, client.get_client());
    for port in ports {
      let can_read  = port.get_capability().contains(alsa::seq::READ);
      let can_write = port.get_capability().contains(alsa::seq::WRITE);
      println!("{}:{}\t{}\t{:?}:{:?}",
        client.get_client(),
        port.get_port(),
        match (can_read, can_write) {
          (true, true) => "DUPLEX",
          (true, false) => "SOURCE",
          (false, true) => "SINK",
          (false, false) => "INERT"
        },
        client.get_name()?,
        port.get_name()?
        
      );
    }
  }
  Ok(())
}

fn main() {
  // run and, if necessary, print error message to stderr
  if let Err(error) = run() {
    eprintln!("Error: {}", error);
    std::process::exit(1);
  }
}
