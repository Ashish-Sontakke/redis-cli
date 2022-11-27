use async_std::io::{self, ReadExt, WriteExt};
use async_std::net::TcpStream;

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let command = b"*1\r\n$4\r\nPING\r\n";
    stream.write_all(command).await?;
    let mut buffer = vec![0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    let result = parse_response(&buffer[..bytes_read]);
    println!("{:?}", result);
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, String> {
    if buffer.is_empty() {
        return Err("Empty buffer".into());
    }
    if buffer[0] == ('-' as u8) {
        return Err(format!(
            "Error Response: {}",
            std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap()
        ));
    }
    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}
