use std::
{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    sync::mpsc,
};
use rust_webserver_practice::ThreadPool;



fn handle_connection(mut stream: TcpStream)
{
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, filename) = if request_line == "GET / HTTP/1.1"
    {
        ("HTTP/1.1 200 OK", "pages/index.html")
    }
    else
    {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };
    
   
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}
fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() 
    {
        let stream = stream.unwrap();
        thread_pool.execute(|| 
        {
            handle_connection(stream)
        });
       
    }
    
}