use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
   let _tcp_listener = TcpListener::bind("127.0.0.1:8976").expect("绑定端口失败"); //绑定监听端口
   for stream in _tcp_listener.incoming() {        //遍历连接的流
        match stream {     //模式匹配流
            Err(_error) =>  println!("Connection Error") ,   //建立连接失败
            Ok(stream) => handle_client(stream) ,   //连接成功，处理客户端消息
            
        }
   }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];    //栈上开辟字节数组
    loop {
        stream.read(&mut buffer).expect("读取失败");     //流中读取信息到buffer中，如果失败打印错误信息
        let str_receive = match String::from_utf8(buffer.to_vec()) {   //模式匹配转换后的字符数组
            Ok(str) => str,            //成功返回str
            Err(_) => return (),       //失败返回空元组，return函数
        };
        
        println!("{}",str_receive);    //打印读取到的转换后的字符数组
    }

}