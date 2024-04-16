use std::io::{stdin,stdout,Write};
use std::fs;
use std::process::Command;


pub struct Befehl<'a>{
    command: &'a str,
}

impl<'a> Befehl<'a> {
    pub fn new(command: &'a str) -> Befehl<'a>{
        return Befehl {
                command: command
            };
    }
    pub fn as_str(&self) -> &'a str{
        return self.command;
    }
    pub fn to_string(&self) -> String{
        let s = self.command.to_string();
        return s;
    }
}

pub fn system(command: Befehl) -> Befehl{
    let _output = Command::new("sh")
        .arg("-c")
        .arg(command.as_str())
        .output()
        .expect("Failed to execute command");
    return command;
}

pub fn input(printer: &str) -> String {
    print!("{}", printer);
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    return s.trim().to_string()
}

pub fn ask(question: &str) -> bool {
    let i = input(question);
    return i == "y" || i == "Y"
}

pub fn download(mirror: &str, file: &str) {
    let cmd = format!("wget -q {}/{} > /dev/null", mirror, file);
    system(Befehl::new(&cmd));
}

pub fn extract(file: &str, outdir: &str) {
    system(Befehl::new(format!("mkdir {}", outdir).as_str()));
    let cmd = format!("tar -xzvf {} --strip-components=1 -C {} > /dev/null", file, outdir);
    system(Befehl::new(&cmd));
}

pub fn readfile(file: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file)
}
