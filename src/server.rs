use std::{env, io};
use actix_web::{web::Path};
use actix_web::{web,App, HttpServer};
use std::collections::HashMap;
use crate::composites::{LogGameStatePeriodic,LogHeal,LogItemAttach,
                         LogItemDetatch,LogItemDrop,LogItemEquip,LogItemPickup,
                         LogItemUnequip,LogItemUse,LogParachuteLanding,
                         LogPlayerAttack,LogPlayerKill,LogPlayerMakeGroggy,
                         LogPlayerRevive,LogPlayerTakeDamage,
                         LogPlayerUseThrowable,LogArmorDestroy};
use crate::composites;
use crate::api;
use crate::APPLICATION_JSON;


#[actix_rt::main]
async fn main() -> io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let map: HashMap<String,api::MyAwesomeInnerDataContainer<composites::Minimal>> = HashMap::from(
        [
            ("LogPlayerAttack".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerAttack>::new("logplayerattack")),
            ("LogArmorDestroy".to_owned(), api::MyAwesomeInnerDataContainer::<LogArmorDestroy>::new("logarmordestroy")),
            ("LogItemAttach".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemAttach>::new("logitemattach")),
            ("LogItemDetatch".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemDetatch>::new("logitemdetatch")),
            ("LogItemDrop".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemDrop>::new("logitemdrop")),
            ("LogHeal".to_owned(), api::MyAwesomeInnerDataContainer::<LogHeal>::new("logheal")),
            ("LogGameStatePeriodic".to_owned(), api::MyAwesomeInnerDataContainer::<LogGameStatePeriodic>::new("loggamestateperiodic")),
            ("LogItemUnequip".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemUnequip>::new("logitemunequip")),
            ("LogItemUse".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemUse>::new("logitemuse")),
            ("LogParachuteLanding".to_owned(), api::MyAwesomeInnerDataContainer::<LogParachuteLanding>::new("logparachutelanding")),
            ("LogPlayerKill".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerKill>::new("logplayerkill")),
            ("LogPlayerMakeGroggy".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerMakeGroggy>::new("logplayermakegroggy")),
            ("LogPlayerRevive".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerRevive>::new("logplayerrevive")),
            ("LogPlayerTakeDamage".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerTakeDamage>::new("logplayertakedamage")),
            ("LogPlayerUseThrowable".to_owned(), api::MyAwesomeInnerDataContainer::<LogPlayerUseThrowable>::new("logplayerusethrowable")),
            ("LogItemEquip".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemEquip>::new("LogItemEquip")),
            ("LogItemPickup".to_owned(), api::MyAwesomeInnerDataContainer::<LogItemPickup>::new("LogItemPickup")),
        ]
    ); 
    
    HttpServer::new( move || {

        let m2 = map.clone();

        let list = move |path: Path<api::Info>|{
            match m2.get(&path.endpoint)
            {
                Some(x) => x.clone().list(path),
                None => actix_web::HttpResponse::Forbidden().content_type(APPLICATION_JSON).json({})
            }
        };
        
        App::new().service(
        web::resource(
            "/{endpoint}/{match_id}/{account}/{timestamp}"
            ).route(
                web::get().to(list)
            ))
    }).bind("127.0.0.1:9090")?.run().await
}