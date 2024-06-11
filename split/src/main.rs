/// example of splitting on whitespace to populate a struct
/// must be a better way...
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::process::Command;

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
        //println!("{:?}", splits);

        let (inaddr, port) = splits[8].split_once(':').expect("couldn't split");

        Self {
            command: splits[0].to_string(),
            port: port.parse::<u32>().unwrap(),
            pid: splits[1].parse::<u32>().unwrap(),
            fd: splits[3].to_string(),
            user: splits[2].to_string(),
            node: splits[7].to_string(),
            inaddr: inaddr.to_string(),
            action: "LISTEN".to_string(), // todo: trim parens off this value
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
impl PartialEq for Listener {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
    }
}
impl Eq for Listener {}
impl Hash for Listener {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.command.hash(state);
        self.pid.hash(state);
        self.port.hash(state);
        self.inaddr.hash(state);
    }
}

fn main() {
    let mut listeners: HashSet<Listener> = HashSet::new();

    //'lsof -nP +c 0 -i4' command returns lines like the following:
    //node                        10166 pete   31u  IPv4 0xe4ad34249b227fc5      0t0  TCP 127.0.0.1:45623 (LISTEN)

    let lsof_lines = Command::new("lsof")
        .arg("-nP")
        .arg("+c")
        .arg("0")
        .arg("-i4")
        .output()
        .unwrap();
    let stdout = String::from_utf8(lsof_lines.stdout).expect("bad stdout from lsof command");
    for line in stdout.lines().skip(1).collect::<HashSet<_>>() {
        if line.ends_with("(LISTEN)") {
            listeners.insert(Listener::new(line));
        }
    }

    for l in &listeners {
        println!("{}", l);
    }
}
