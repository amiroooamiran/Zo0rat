use std::{
    io::{self, Read, Write}, net::TcpListener, process::{self, Command}, sync::Arc
};

fn excute_command(command: &str, args: Vec<&str>){
    let output = Command::new(command)
            .args(args)
            .output()
            .expect("faild to excute the process");
    if output.status.success(){
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stderr).unwrap();
    }
}

fn run_server() -> io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    println!("Server is ready and listening on 0.0.0.0:8000");

    for stream in listener.incoming(){
        match stream {
            Ok(mut stream) =>{
                println!("Connection recived");
                
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer)?;
                let messages = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Message received: {}", messages);

                let response = "Ack from server";
                stream.write_all(response.as_bytes())?;
            }
            Err(e) => eprintln!("Error connection: {}", e), 
        }
    }

    Ok(())
}

fn main(){
    excute_command("figlet", vec!["Zo0rat"]);

    run_server();
    println!("Hello this server");
}