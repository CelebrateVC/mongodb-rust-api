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
use crate::api::{self, Gettable};
use crate::server::HasEndpoint;
use crate::server;
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
        type API<Subtype> = api::APIEndpointContainer::<Subtype, T, Minimal,Info>;

        println!("1");
        let uri = "mongodb://192.168.0.101:27017";
        let database = "pubg";
        println!("hello");

        // TODO: Create internal library functionality; pass in type, string, and endpoint get back endpoint hashmap
        let data = futures::join!(
               API::<LogPlayerAttack>::new("logplayerattack",uri,database),
               API::<LogArmorDestroy>::new("logarmordestroy",uri,database),
               API::<LogItemAttach>::new("logitemattach",uri,database),
            //    API<LogItemDetatch>::new("logitemdetatch",uri,database),
            //    API<LogItemDrop>::new("logitemdrop",uri,database),
            //    API<LogHeal>::new("logheal",uri,database),
            //    API<LogGameStatePeriodic>::new("loggamestateperiodic",uri,database),
            //    API<LogItemUnequip>::new("logitemunequip",uri,database),
            //    API<LogItemUse>::new("logitemuse",uri,database),
            //    API<LogParachuteLanding>::new("logparachutelanding",uri,database),
            //    API<LogPlayerKill>::new("logplayerkill",uri,database),
            //    API<LogPlayerMakeGroggy>::new("logplayermakegroggy",uri,database),
            //    API<LogPlayerRevive>::new("logplayerrevive",uri,database),
            //    API<LogPlayerTakeDamage>::new("logplayertakedamage",uri,database),
            //    API<LogPlayerUseThrowable>::new("logplayerusethrowable",uri,database),
            //    API<LogItemEquip>::new("LogItemEquip",uri,database),
            //    API<LogItemPickup>::new("LogItemPickup",uri,database),
            );

            let data = [
                    ("LogPlayerAttack".to_owned(),      data.0),
                    ("LogArmorDestroy".to_owned(),      data.1),
                    ("LogItemAttach".to_owned(),        data.2),
                    // ("LogItemDetatch".to_owned(),       data.3),
                    // ("LogItemDrop".to_owned(),          data.4),
                    // ("LogHeal".to_owned(),              data.5),
                    // ("LogGameStatePeriodic".to_owned(), data.6),
                    // ("LogItemUnequip".to_owned(),       data.7),
                    // ("LogItemUse".to_owned(),           data.8),
                    // ("LogParachuteLanding".to_owned(),  data.9),
                    // ("LogPlayerKill".to_owned(),        data.10),
                    // ("LogPlayerMakeGroggy".to_owned(),  data.11),
                    // ("LogPlayerRevive".to_owned(),      data.12),
                    // ("LogPlayerTakeDamage".to_owned(),  data.13),
                    // ("LogPlayerUseThrowable".to_owned(),data.14),
                    // ("LogItemEquip".to_owned(),         data.15),
                    // ("LogItemPickup".to_owned(),        data.16),
                ];


        let map: HashMap<String,api::APIEndpointContainer<Minimal, T,Minimal, Info>>= HashMap::from(     data   ); 

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
                                answer = strut.clone();
                            }
                        }
                        Some(answer)
                    },
                    None => None}
                }
            None => None}
    }
}

impl HasEndpoint for Info{fn get_endpoint(self) -> String {self.endpoint}}

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

impl Default for Info{
    fn default() -> Self {
        Self { endpoint: Default::default(), match_id: Default::default(), account: Default::default(), timestamp: Default::default() }
    }
}