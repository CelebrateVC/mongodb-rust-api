use actix_web::{web::Path};
use actix_web::HttpResponse;
use serde::Serialize;
use mongodb::{
    bson::doc,
    Cursor,
    Database,
    Collection,
    error::Error};
use futures::stream::TryStreamExt;

use crate::{mongo_connection,APPLICATION_JSON};


pub trait Insertable<T>{
    fn obj_entry_or_insert(self,dict: T) -> T;
}

pub trait Gettable<Info,Minimal>{
    fn get(&self, k: Path<Info>) -> Option<Minimal>;
}


#[derive(Clone)]
pub struct APIEndpointContainer<Base,DataContainer: Clone, Minimal: Clone, Info: Clone>{
    data: DataContainer,
    base: Base,
    min: Minimal,
    info: Info
}


impl<Minimal: std::default::Default +  Serialize + Clone,
     DataContainer: Clone + Gettable<Info,Minimal>,
     Info: Clone
     > APIEndpointContainer<Minimal,DataContainer,Minimal, Info>{

    pub fn list(self, path: Path<Info>) -> HttpResponse{


        match self.data.get(path){
            Some(data) =>{
                HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(data)
            },
            None => HttpResponse::NotFound().content_type(APPLICATION_JSON).json(ErrorMessage{error:"No Match Found".to_string()})
        }    
    }
}

impl<
    Minimal: Clone + Default + Insertable<DataContainer> ,
    T: Into<Minimal> + serde::de::DeserializeOwned + Unpin + std::marker::Send + Sync + std::fmt::Debug,
    DataContainer: Clone+Default,
    Info: Clone + Default  >
    APIEndpointContainer<T,DataContainer,Minimal, Info>{
    pub async fn new(endpoint: &str,uri: &str, database: &str)-> APIEndpointContainer<Minimal,DataContainer,Minimal, Info>{
        println!("{} getting Count",endpoint);
        let mini: Minimal = std::default::Default::default();
        let blank_data: DataContainer = std::default::Default::default();
        let info = std::default::Default::default();
        let mut obj: APIEndpointContainer<Minimal, DataContainer, Minimal, Info> = APIEndpointContainer { data: blank_data, min: mini.clone(), base:mini, info:info };

        let db: Database = mongo_connection(uri.to_owned(),database.to_owned()).await;
        let collection: Collection<T> = db.collection::<T>(endpoint);

        let prog_count = collection.count_documents(doc! {}, None).await.unwrap();
        println!("{} has {} records, reading",endpoint, prog_count);

        let items: Result<Cursor<T>, Error> = collection.find(doc! {}, None).await;
    
        match items {
            Ok(mut j) => {
                println!("{} - data loading",endpoint);
                while let Ok(record) = j.try_next().await{
                    if let Some(data) = record{
                        let min: Minimal = data.into();
                        obj.data = min.obj_entry_or_insert(obj.data);
                    }
                    else{break}
                }
                },
            Err(k) => println!("{} - Collection Error\n\t{:?}",endpoint, k)
        }
        println!("{} - data recieved",endpoint);
        obj
}}



#[derive(Serialize)]
struct ErrorMessage{
 error: String
}


