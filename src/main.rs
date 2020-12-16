use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn main() {
    // TcpListenerにより、アドレス127.0.0.1:7878でTCP接続をリッスンできます
    // unwrap メソッドは Ok なら中の値を返し、Err なら panic を起こす
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incomingメソッドは、一連のストリームを与えるイテレータを返す
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 接続が確立しました
        handle_connection(stream);
    }
}

//  次回データを要求した時のためにそのデータを保存する可能性がある
//  故に、内部の状態が変化する可能性がある
fn handle_connection(mut stream: TcpStream) {
    // 読み取ったデータを保存する
    // サイズは1024
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // ↓↓↓
    // Request: GET / HTTP/1.1
    // Host: 127.0.0.1:7878
    // Connection: keep-alive
    // Cache-Control: max-age=0
    // Upgrade-Insecure-Requests: 1
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
    // Sec-Fetch-Site: none
    // Sec-Fetch-Mode: navigate
    // Sec-Fetch-User: ?1
    // Sec-Fetch-Dest: document
    // Accept-Encoding: gzip, deflate, br
    // Accept-Language: ja,en-US;q=0.9,en;q=0.8

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // 文字列をバイトに変換
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぐ
    stream.flush().unwrap();
}
