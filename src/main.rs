use tiny_ping::Pinger;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            run().await.unwrap();
        })
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinger = Pinger::new("10.4.0.98".parse()?)?;
    pinger.timeout(std::time::Duration::from_secs(1));
    let rez = pinger.ping(0).await;

    println!("{:?}", rez);
    Ok(())
}
