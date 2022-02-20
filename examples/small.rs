extern crate mongodb_api;

use mongodb_api::api::Minable;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;

type DataContainer = HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>>;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    type API<Subtype> = mongodb_api::api::APIEndpointContainer::<Subtype, DataContainer, Minimal,Info>;
    let uri = "mongodb://192.168.0.101:27017";
    let database = "pubg";
    let map = HashMap::from([("LogAttack".to_string(),API::<LogPlayerKill>::new("logplayerkill",uri,database).await)]);
    mongodb_api::server::WebServer::new(map,
        "/{endpoint}/{match_id}/{account}/{timestamp}".to_owned(),
        "127.0.0.1".to_owned(),
        "9090".to_owned()).start()
}

#[derive(Deserialize, Clone, Default)]
pub struct Info {
    pub endpoint: String,
    match_id: String,
    account:String,
    timestamp: String
}

impl mongodb_api::server::HasEndpoint for Info{fn get_endpoint(self) -> String {self.endpoint}}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Minimal{
    pub _id: ObjectId, 
    pub account_id: String,
    pub mongo_match_id:ObjectId,
    pub _d: String,
}

impl Minable<Minimal> for Minimal{fn to_min(&self) ->Minimal {self.clone()}}

impl Minable<Minimal> for LogPlayerKill{
    fn to_min(&self) ->Minimal {
        Minimal{_id:self._id,
                account_id: self.killer.clone().unwrap_or_default().account_id.unwrap_or("default account".to_string()),
                mongo_match_id: self.mongo_match_id.unwrap_or_default(), 
                _d: self._d.clone()}
    }
}

impl Default for Minimal{
    fn default() -> Self {
        Minimal {
            _d: "0000-00-00t00:00:00.000Z".to_string(),
            account_id: "default_account".to_owned(),
            mongo_match_id: Default::default(),
            _id: Default::default()
        }  
    }
}

impl mongodb_api::api::Insertable<DataContainer> for Minimal{
    fn obj_entry_or_insert(self,mut dict: DataContainer) -> DataContainer {
                                    dict.entry(self.mongo_match_id.to_owned())
                                    .or_insert(HashMap::new())
                                        .entry(self.account_id.clone())
                                        .or_insert(HashMap::new())
                                            .insert(self._d.clone(), self);
                                    return dict
    }
}

impl mongodb_api::api::Gettable<Info,Minimal> for DataContainer{
    fn get(&self, path: actix_web::web::Path<Info>) -> Option<Minimal> {

        let ac: String = path.account.clone();
        let ts: String = path.timestamp.clone();

        let mut default_match_id: [u8; 12] = [0;12];
        let mut j = 0;
        for i in path.match_id.as_bytes(){
            default_match_id[j] = *i;
            j+=1;
            if j == 12 {break;}
        }

        let mi = ObjectId::from_bytes(default_match_id);
        let mut answer: Minimal = std::default::Default::default();


        match self.get(&mi){
            Some(hash) =>{
                match hash.get(&ac){
                    Some(data) =>{
                        for (date, strut) in data{
                            if answer._d<date.to_string() && date<&ts {
                                answer = strut.to_min();
                            }
                        }
                        Some(answer)
                    },
                    None => None}
                }
            None => None}
    }
}



//###################
// Data from MongoDB
//###################
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Account {
    pub name: Option<String>,
    #[serde(rename="teamId")] pub team_id: Option<f32>,
    pub health: Option<f32>,
    pub ranking: Option<f32>,
    #[serde(rename="accountId")] pub account_id: Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerKill{
    _id: ObjectId,
    #[serde(rename="attackId")] attack_id: i64,
    killer: Option<Account>,
    victim: Option<Account>,
    #[serde(rename="damageTypeCategory")] damage_type_category: Option<String>,
    #[serde(rename="damageCauserName")] damage_causer_name: Option<String>,
    distance: f64,
    #[serde(rename="_D")]
    pub _d: String,
    mongo_match_id: Option<ObjectId>,
}

