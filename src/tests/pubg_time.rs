use actix_web;
mod composites;
use std::collections::HashMap;
use mongodb::bson::oid::ObjectId;
#[allow(unused_imports)]
use composites::{LogGameStatePeriodic,LogHeal,LogItemAttach,
    LogItemDetatch,LogItemDrop,LogItemEquip,LogItemPickup,
    LogItemUnequip,LogItemUse,LogParachuteLanding,
    LogPlayerAttack,LogPlayerKill,LogPlayerMakeGroggy,
    LogPlayerRevive,LogPlayerTakeDamage,
    LogPlayerUseThrowable,LogArmorDestroy,Minimal};
use serde::Deserialize;
use crate::api::{self, Minable, Gettable};
use crate::server::HasEndpoint;
use crate::{mongo_connection, server};
use tokio::runtime::Runtime;
use std::sync::mpsc;

#[test]
fn run_db_connect(){

    let rt = Runtime::new().unwrap();

    rt.block_on( async move {
    let (tx, rx) = mpsc::channel();

    let _x = tokio::spawn(async{ test_timing_api(tx).await });
    match rx.recv(){
    Ok(x) => println!("recieved {}!",x),
    Err(e) => println!("{:?}",e)
    };
});
println!("unblocked?");

}

async fn test_timing_api(tx: mpsc::Sender<i32>) {

        type T = HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>>;

        println!("1");
        let db = mongo_connection("mongodb://192.168.0.101:27017".to_owned(), "pubg".to_owned()).await;
        println!("hello");

        let map: HashMap<String,api::APIEndpointContainer<Minimal, T,Minimal, Info>>= HashMap::from(
            [
                // ("LogPlayerAttack".to_owned(),      api::APIEndpointContainer::<LogPlayerAttack      , T, Minimal,Info>::new("logplayerattack",&db).await),
                ("LogArmorDestroy".to_owned(),      api::APIEndpointContainer::<LogArmorDestroy      , T, Minimal,Info>::new("logarmordestroy",&db).await),
                // ("LogItemAttach".to_owned(),        api::APIEndpointContainer::<LogItemAttach        , T, Minimal,Info>::new("logitemattach",&db).await),
                // ("LogItemDetatch".to_owned(),       api::APIEndpointContainer::<LogItemDetatch       , T, Minimal,Info>::new("logitemdetatch",&db).await),
                // ("LogItemDrop".to_owned(),          api::APIEndpointContainer::<LogItemDrop          , T, Minimal,Info>::new("logitemdrop",&db).await),
                // ("LogHeal".to_owned(),              api::APIEndpointContainer::<LogHeal              , T, Minimal,Info>::new("logheal",&db).await),
                // ("LogGameStatePeriodic".to_owned(), api::APIEndpointContainer::<LogGameStatePeriodic , T, Minimal,Info>::new("loggamestateperiodic",&db).await),
                // ("LogItemUnequip".to_owned(),       api::APIEndpointContainer::<LogItemUnequip       , T, Minimal,Info>::new("logitemunequip",&db).await),
                // ("LogItemUse".to_owned(),           api::APIEndpointContainer::<LogItemUse           , T, Minimal,Info>::new("logitemuse",&db).await),
                // ("LogParachuteLanding".to_owned(),  api::APIEndpointContainer::<LogParachuteLanding  , T, Minimal,Info>::new("logparachutelanding",&db).await),
                // ("LogPlayerKill".to_owned(),        api::APIEndpointContainer::<LogPlayerKill        , T, Minimal,Info>::new("logplayerkill",&db).await),
                // ("LogPlayerMakeGroggy".to_owned(),  api::APIEndpointContainer::<LogPlayerMakeGroggy  , T, Minimal,Info>::new("logplayermakegroggy",&db).await),
                // ("LogPlayerRevive".to_owned(),      api::APIEndpointContainer::<LogPlayerRevive      , T, Minimal,Info>::new("logplayerrevive",&db).await),
                // ("LogPlayerTakeDamage".to_owned(),  api::APIEndpointContainer::<LogPlayerTakeDamage  , T, Minimal,Info>::new("logplayertakedamage",&db).await),
                // ("LogPlayerUseThrowable".to_owned(),api::APIEndpointContainer::<LogPlayerUseThrowable, T, Minimal,Info>::new("logplayerusethrowable",&db).await),
                // ("LogItemEquip".to_owned(),         api::APIEndpointContainer::<LogItemEquip         , T, Minimal,Info>::new("LogItemEquip",&db).await),
                // ("LogItemPickup".to_owned(),        api::APIEndpointContainer::<LogItemPickup        , T, Minimal,Info>::new("LogItemPickup",&db).await),
            ]
        ); 

        let server_ = server::WebServer::new(map,
                                                                             "/{endpoint}/{match_id}/{account}/{timestamp}".to_owned(),
                                                                             "127.0.0.1".to_owned(),
                                                                             "9090".to_owned());
        let _x = tx.send(1);
        server_.start().unwrap();


}

#[derive(Deserialize, Clone)]
pub struct Info {
    pub endpoint: String,
    match_id: String,
    account:String,
    timestamp: String
}

impl api::Insertable<HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>>> for Minimal{
    fn obj_entry_or_insert(self,mut dict: HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>>) -> HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>> {
                                    dict.entry(self.mongo_match_id.to_owned())
                                    .or_insert(HashMap::new())
                                        .entry(self.account_id.clone())
                                        .or_insert(HashMap::new())
                                            .insert(self._d.clone(), self);
                                    return dict
    }
}

impl Gettable<Info,Minimal> for HashMap<ObjectId,HashMap<String,HashMap<String, Minimal>>>{
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

impl HasEndpoint for Info{
    fn get_endpoint(self) -> String {
        self.endpoint
    }
}

impl Default for Minimal{
    fn default() -> Self {
        let mut default_match_id: [u8; 12] = [0;12];
        let mut j = 0;
        for i in "612aacfc20c35c423897bed4".as_bytes(){
            default_match_id[j] = *i;
            j+=1;
            if j == 12 {break;}
        }
        let default_oid = ObjectId::from_bytes(default_match_id);


        Minimal {
            _d: "0000-00-00t00:00:00.000Z".to_string(),
            account_id: "default_account".to_owned(),
            mongo_match_id: default_oid,
            _id: ObjectId::new()
        }  
    }

}

impl Default for Info{
    fn default() -> Self {
        Self { endpoint: Default::default(), match_id: Default::default(), account: Default::default(), timestamp: Default::default() }
    }
}