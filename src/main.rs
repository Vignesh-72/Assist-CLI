mod assistx;
use tokio;

#[tokio::main]
async fn main(){
  assistx::start().await;
}