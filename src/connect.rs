use std::process::exit;
use mongodb::sync::{Client,Database};



pub fn mongo_connection(uri:String, database:String) -> Database{
    let client = Client::with_uri_str(uri);
    if client.is_err(){
        println!("{:?}",client);
        exit(1)
    };
    let con:  Client = client.unwrap();
    con.database(&database)

}
