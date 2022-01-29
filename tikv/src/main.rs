use tikv_client::{TransactionClient, Error};

async fn run() -> Result<(), Error> {
    let txn_client = TransactionClient::new(vec!["127.0.0.1:2379"]).await?;
    let mut txn = txn_client.begin_optimistic().await?;
    txn.put("key".to_owned(), "value".to_owned()).await?;
    let value = txn.get("key".to_owned()).await?;
    println!("value: {:?}", value);
    txn.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
