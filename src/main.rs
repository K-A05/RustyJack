fn main() {
    
    mod proxy_server;
    mod request_handler;
    mod tls_handle;

    use clap::Parser;
    use log::info;

    #[derive(Parser)] // will automatically generate the code needed to parse the coommand line
    struct CLI{
        #[args(short , long, default_value:"8080")]
        port: u16,
    }

    #[tokio:: main]
    async fn main() -> anyhow::Result<()>{
        env_logger::init();
        let cli = CLI::parse();

        info!("Starting annonymous proxy server on port {}", cli.port);
        proxy_server::start_proxy_server(cli.port).await?;

        OK(())
    }

}
