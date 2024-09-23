use  std::net::{TcpListener,TcpStream};
pub fn tcp_handle(stream:TcpStream){
    let mut info_buffer:Vec<u8> = Vec::new();
}
fn main() {
    let listener = TcpListener::bind("218.199.84.181:7878").unwrap();
    for stream in listener.incoming(){
        tcp_handle(stream.unwrap());
    }
}
