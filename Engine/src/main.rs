use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt,SinkExt};
use std::collections::{HashMap,HashSet};

#[tokio::main]
async fn main() {
   
}

pub struct Client{

}

pub struct network_handler{
    active_user : HashSet<String>,
    connections:u32,
    Users_rewardPoints : HashMap<String,String>,
}

impl network_handler{
    
    pub fn new()->network_handler{
        network_handler{
            active_user:HashSet::new(),
            connections:0,
            Users_rewardPoints:HashMap::new(),
        }
    }

    pub async fn run(self){
        let addr ="127.0.0.1:8001" ;

        let listener = TcpListener::bind(&addr).await.expect("Error while binding");
    
        while let Ok((stream,_)) = listener.accept().await{
            tokio::spawn(async move {
                if let Err(e) = network_handler::handle_incoming(stream).await {
                    eprintln!("Error while handling incoming connection: {}", e);
                }
            });
        }
    }

    pub async fn handle_incoming(stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await.expect("Error while handshanking");
        let (mut writer, mut reader) = ws_stream.split();
        let mut buf: Vec<u8> = Vec::new();
        todo!();
        Ok(())
    }
}