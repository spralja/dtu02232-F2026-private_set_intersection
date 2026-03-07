use crate::client::ClientStateInit;
use crate::server::ServerStateInit;
use crate::types::Element;

use std::sync::mpsc;
use std::thread;

pub fn run_threaded(
  server_set: Vec<Element>,
  client_set: Vec<Element>,
) -> Vec<Element> {
  let (server_to_client1_tx, server_to_client1_rx) = mpsc::channel();
  let (client_to_server2_tx, client_to_server2_rx) = mpsc::channel();
  let (server_to_client3_tx, server_to_client3_rx) = mpsc::channel();

  let server_thread = thread::spawn(move || {
    let server_init = ServerStateInit { X: server_set };

    let (server_state1, message1) = server_init.start();
    server_to_client1_tx.send(message1).unwrap();

    let message2 = client_to_server2_rx.recv().unwrap();

    let (_, message3) = server_state1.respond(message2);
    server_to_client3_tx.send(message3).unwrap();
  });

  let client_thread = thread::spawn(move || {
    let client_init = ClientStateInit { Y: client_set };

    let message1 = server_to_client1_rx.recv().unwrap();

    let (client_state2, message2) = client_init.respond(message1);
    client_to_server2_tx.send(message2).unwrap();

    let message3 = server_to_client3_rx.recv().unwrap();
    client_state2.complete(message3)
  });

  let result = client_thread.join().unwrap();

  server_thread.join().unwrap();

  result
}
