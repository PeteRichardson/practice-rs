use std::fmt;

#[derive(Debug)]
struct Listener {
    command: String,
    port: u32,
    pid: u32,
    fd: String,
    user: String,
    node: String,
    inaddr: String,
    action: String,
    _full_command: String,
}

impl Listener {
    fn new(lsof_line: &str) -> Self {
        let splits: Vec<&str> = lsof_line.split_ascii_whitespace().collect();
        // for s in splits {
        //     println!("{}", s)
        // }
        Self {
            command: splits[0].to_string(),
            port: splits[1].parse::<u32>().unwrap(),
            pid: 716,
            fd: String::from("10u"),
            user: String::from("pete"),
            node: String::from("TCP"),
            inaddr: String::from("127.0.0.1"),
            action: String::from("LISTEN"),
            _full_command: lsof_line.to_string(),
        }
    }
}

impl fmt::Display for Listener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}:{}",
            self.command,
            self.port,
            self.pid,
            self.fd,
            self.user,
            self.node,
            self.inaddr,
            self.action
        )
    }
}

fn main() {
    let lsof_line = "node                        10166 pete   31u  IPv4 0xe4ad34249b227fc5      0t0  TCP 127.0.0.1:45623 (LISTEN)";
    let l = Listener::new(lsof_line);
    println!("{}", l);
}
