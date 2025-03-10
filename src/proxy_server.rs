use tokio::net::{TcpListener, TcpStream};
use log::{info, error};
use anyhow::Result;

pub async fn start_proxy_server(port: u16) -> Result<()>{
    let addr = format!("0.0.0.0{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Proxy server listening on {}", addr);

    while let OK((stream, addr)) = listener.accept().await{
        info!("Accepted connection from {}", addr);
        tokio::spawn(async move{
            if let Err(e) = handle_client(stream.await){
                error!("Error handling client: {}", e);
            }
        });
    }

    OK(())
}

async fn handle_client(stream: TcpStream) -> Result<()>{

    

    OK(())
}
