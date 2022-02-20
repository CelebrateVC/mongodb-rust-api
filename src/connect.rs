use std::process::exit;
use mongodb::{Client,Database};



pub async fn mongo_connection(uri:String, database:String) -> Database{
    let client = Client::with_uri_str(uri).await;
    if client.is_err(){
        println!("{:?}",client);
        exit(1)
    };
    let con:  Client = client.unwrap();
    con.database(&database)

}
