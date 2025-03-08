use clap::{Parser, Subcommand};
use crate::core::chain::Blockchain;
mod core;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use actix_web::{web, App, HttpServer};
mod config;
use crate::config::AppConfig;
mod network;

#[derive(Parser, Debug)]
#[clap(version, about)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 创建新节点
    New,
    /// 挖矿并添加新区块 [data]
    Mine { data: String },
    /// 查看区块链
    View,
    /// 验证区块链有效性
    Validate,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::default().clone();
    let blockchain = Arc::new(Mutex::new(Blockchain::new(config.clone())));

    // 自动挖矿任务
    let chain_ref = blockchain.clone();
    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_secs(config.mine_interval_secs));
        loop {
            interval.tick().await;
            if config.auto_mine {
                let mut chain = chain_ref.lock().unwrap();
                chain.add_block("Auto mined block");
                chain.save_to_file().unwrap();
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(web::resource("/blocks").route(web::get().to(crate::network::server::get_blocks)))
            .service(web::resource("/blocks/{index}").route(web::get().to(crate::network::server::get_block)))
            .service(web::resource("/sync").route(web::post().to(crate::network::server::sync_chain)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
