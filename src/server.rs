use std::{env, io};
use actix_web::{web::Path};
use actix_web::{web,App, HttpServer};
use serde::de::DeserializeOwned;
use serde::{Serialize};
use std::collections::HashMap;

use crate::api::{self, Minable, Gettable};
use crate::APPLICATION_JSON;

pub trait HasEndpoint{
    fn get_endpoint(self) -> String;
}
pub struct WebServer<Minimal: Minable<Minimal> + Clone, DataContainer: Clone, Info: Clone>{
    endpoints: HashMap<String,api::APIEndpointContainer<Minimal,DataContainer,Minimal, Info>>,
    path: String,
    server_url: String,
    server_port: String, 
}

impl<Minimal: Minable<Minimal> + Clone,
     DataContainer: Clone, 
     Info: Clone> 
    WebServer<Minimal, DataContainer, Info>{
    pub fn new(endpoints: HashMap<String,api::APIEndpointContainer<Minimal,DataContainer,Minimal, Info>>,
        path: String,
        server_url: String,
        server_port: String )
            -> Self
        {
            WebServer{endpoints,path,server_url,server_port}
        }

}

impl<Minimal: 'static + Minable<Minimal> + Clone + Default + Serialize + DeserializeOwned + Send,
     DataContainer: 'static + Clone + Gettable<Info,Minimal> + Send,
     Info: 'static + Clone+HasEndpoint+DeserializeOwned + Send 
    >
     WebServer<Minimal, DataContainer, Info>{

    #[actix_rt::main]
    pub async fn start(self) -> io::Result<actix_web::dev::Server>{
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();

        
        let serv = HttpServer::new( move || {
    
            let m2 = self.endpoints.clone();
    
            let list = move |path: Path<Info>|{
                match m2.get(&path.clone().get_endpoint())
                {
                    Some(x) => x.clone().list(path),
                    None => actix_web::HttpResponse::Forbidden().content_type(APPLICATION_JSON).json({})
                }
            };
            
            App::new().service(
            web::resource(
                &self.path
                ).route(
                    web::get().to(list)
                ))
        }).bind(self.server_url+":"+&self.server_port);

        return match serv {
            Ok(server) => {
                let app = server.run();
                Ok(app)
            },
            Err(err) => Err(err)
        }
    }

}


