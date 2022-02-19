use actix_web::{web::Path};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;
use mongodb::{
    bson::doc,
    sync::{
        Client,
        Cursor,
        Database,
        Collection},
    error::Error};

use crate::APPLICATION_JSON;
use crate::composites::{Minimal, Minable};


#[derive(Clone)]
pub struct MyAwesomeInnerDataContainer<_T>{
    data: HashMap<ObjectId,HashMap<String,HashMap<String, _T>>>
}


impl<T: Minable+ serde::de::DeserializeOwned + Unpin + std::marker::Send + Sync + std::fmt::Debug> MyAwesomeInnerDataContainer<T>{

    pub fn list(self, path: Path<Info>) -> HttpResponse{
        let ac: String = path.account.clone();
        let ts: String = path.timestamp.clone();
        let mi = ObjectId::from_str(&path.match_id).unwrap();

        let mut answer = Minimal {
            _d: "0000-00-00t00:00:00.000Z".to_string(),
            account_id: ac.to_owned(),
            mongo_match_id: mi.to_owned(),
            _id: ObjectId::new()
        };

        match self.data.get(&mi){
            Some(hash) =>{
                match hash.get(&ac){
                    Some(data) =>{
                        for (date, strut) in data{
                            if answer._d<date.to_string() && date<&ts {
                                answer = strut.to_min()
                            }
                        }
                    }
                    None => return HttpResponse::NotFound().content_type(APPLICATION_JSON).json(ErrorMessage{error:"No Account".to_string()})
                }
            },
            None => return HttpResponse::NotFound().content_type(APPLICATION_JSON).json(ErrorMessage{error:"No Match".to_string()})
        }    
    
        HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(answer)

    }


    pub fn new(endpoint: &str)-> MyAwesomeInnerDataContainer<Minimal>{
        println!("{}",endpoint);
        let mut obj = MyAwesomeInnerDataContainer { data: HashMap::new() };

        let client = Client::with_uri_str("mongodb://192.168.0.100:27017");
        if client.is_err(){
            println!("{:?}",client);
            exit(1)
        };
        let con:  Client = client.unwrap();
        let db:  Database = con.database("pubg");
        let collection: Collection<T> = db.collection::<T>(endpoint);

        let prog_count = collection.count_documents(doc! {}, None).unwrap();

        let items: Result<Cursor<T>,Error> = collection.find(doc! {}, None);
    
        match items {
            Ok(j) => {
                let prog = indicatif::ProgressBar::new(prog_count);
                for record in j{
                    prog.inc(1);
                    match record{
                        Ok(rec) => {
                            let min: Minimal = rec.to_min();
                            obj.data.entry(min.mongo_match_id.to_owned())
                                    .or_insert(HashMap::new())
                                        .entry(min.account_id.clone())
                                        .or_insert(HashMap::new())
                                            .insert(min._d.clone(), min);
                        },
                        Err(k) => println!("{:?}",k)
                    };
            }
            },
            Err(k) => println!("{:?}",k)
        }

        obj
    }
}

#[derive(Deserialize)]
pub struct Info {
    pub endpoint: String,
    match_id: String,
    account:String,
    timestamp: String
}

#[derive(Serialize)]
struct ErrorMessage{
 error: String
}


