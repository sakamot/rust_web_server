use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    // TcpListenerにより、アドレス127.0.0.1:7878でTCP接続をリッスンできます
    // unwrap メソッドは Ok なら中の値を返し、Err なら panic を起こす
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 新しいスレッドを生成
    let pool = ThreadPool::new(4);

    // incomingメソッドは、一連のストリームを与えるイテレータを返す
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // プールが各ストリームに対して実行すべきクロージャを受け取る
        pool.execute(|| {
            // 接続が確立しました
            handle_connection(stream);
        })
    }
}

//  次回データを要求した時のためにそのデータを保存する可能性がある
//  故に、内部の状態が変化する可能性がある
fn handle_connection(mut stream: TcpStream) {
    // 読み取ったデータを保存する
    // サイズは1024
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // バイト文字列に変換
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    // 文字列をバイトに変換
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぐ
    stream.flush().unwrap();
}


