use rs_cluster::App;

#[tokio::main]
async fn main() {
    let _ = App::default().run().await;
}
