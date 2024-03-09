use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    if let Err(err) = connect() {
        println!("err = {}", err);
    }
}

fn connect() -> Result<()> {
    let mut stream = TcpStream::connect("https://online.hcmue.edu.vn:443")?;   // www.rust-lang.org:80

    let mut request_data = String::new();
    request_data.push_str("GET / HTTP/1.0");
    request_data.push_str("\r\n");
    request_data.push_str("Host: www.rust-lang.org");
    request_data.push_str("\r\n");
    request_data.push_str("Connection: close"); // <== Here!
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");

    println!("request_data = {:?}", request_data);

    let request = stream.write_all(request_data.as_bytes())?;
    println!("request = {:?}", request);

    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;
    println!("result = {}", result);
    println!("buf = {}", buf);

    Ok(())
}
