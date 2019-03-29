extern crate session_types;
use session_types::*;
use std::thread;

type Server = Recv<i64, Send<bool, Eps>>;
type Client = <Server as HasDual>::Dual;

fn srv(c: Chan<(), Server>) {
    let (c, n) = c.recv();
    if n % 2 == 0 {
        c.send(true).close()
    } else {
        c.send(false).close()
    }
}

fn cli(c: Chan<(), Client>) {
    let n = 42;
    let c = c.send(n);
    let (c, b) = c.recv();

    if b {
        println!("{} is even", n);
    } else {
        println!("{} is odd", n);
    }

    c.close();
}

fn main() {
    let (server_chan, client_chan) = session_channel();

    let srv_t = thread::spawn(move || srv(server_chan));
    let cli_t = thread::spawn(move || cli(client_chan));

    let _ = (srv_t.join(), cli_t.join());
}

