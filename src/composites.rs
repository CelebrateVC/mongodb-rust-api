use std::str::FromStr;

use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId};

mod primatives;



#[derive(Debug, Serialize, Deserialize)]
pub struct LogArmorDestroy{
    pub _id: ObjectId,
    #[serde(rename="attackId")]
    attack_id : Option<i64>,
    attacker: Option<primatives::Account>,
    victim: Option<primatives::Account>,
    #[serde(rename="damageTypeCategory")]
    damage_type_category: Option<String>,
    #[serde(rename="damageReason")]
    damage_reason: Option<String>,
    #[serde(rename="damageCauserName")]
    damage_causer_name: Option<String>,
    item: Option<primatives::Item>,
    distance: Option<f64>,
    common: Option<primatives::Common>,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

// struct LogCarePackageLand{}
// struct LogCarePackageSpawn{}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogGameStatePeriodic{
    _id: ObjectId,
    #[serde(rename="gameState")]
    game_state: primatives::GameState,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LogHeal{
    _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    #[serde(rename="healAmount")]
    heal_amount: Option<f64>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogItemAttach{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    #[serde(rename="parentItem")]
    parent_item: Option<primatives::Item>,
    #[serde(rename="childItem")]
    child_item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LogItemDetatch {
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    #[serde(rename="parentItem")]
    parent_item: Option<primatives::Item>,
    #[serde(rename="childItem")]
    child_item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogItemDrop{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogItemEquip{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogItemPickup{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

// struct LogItemPickupFromCarePackage{}
// struct LogItemPickupFromLootbox{}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogItemUnequip{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogItemUse{
    pub _id: ObjectId,
    character: Option<primatives::Account>,
    item: Option<primatives::Item>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

/* Many Skipped */

#[derive(Debug,Serialize,Deserialize)]
pub struct LogParachuteLanding{
    _id: ObjectId,
    character: Option<primatives::Account>,
    distance: Option<f64>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerAttack{
    _id: ObjectId,
    #[serde(rename="attackId")]
    attack_id: u64,
    attacker: Option<primatives::Account>,
    #[serde(rename="attackType")]
    attack_type: Option<String>,
    weapon: Option<primatives::Item>,
    // Vehicle
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerKill{
    _id: ObjectId,
    #[serde(rename="attackId")]
    attack_id: u64,
    killer: Option<primatives::Account>,
    victim: Option<primatives::Account>,
    #[serde(rename="damageTypeCategory")]
    damage_type_category: Option<String>,
    #[serde(rename="damageCauserName")]
    damage_causer_name: Option<String>,
    distance: f64,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerMakeGroggy{
    _id: ObjectId,
    #[serde(rename="attackId")]
    attack_id: u64,
    attacker: Option<primatives::Account>,
    victim: Option<primatives::Account>,
    #[serde(rename="damageTypeCategory")]
    damage_type_category: Option<String>,
    #[serde(rename="damageCauserName")]
    damage_causer_name: Option<String>,
    distance: f64,
    #[serde(rename="isAttackerInVehicle")]
    is_attacker_in_vehicle: bool,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerPosition{
    _id: ObjectId,
    character: Option<primatives::Account>,
    #[serde(rename="elapsedTime")]
    elapsed_time: i32,
    #[serde(rename="numAlivePlayers")]
    num_alive_players: i16,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerRevive{
    _id: ObjectId,
    victim: Option<primatives::Account>,
    reviver: Option<primatives::Account>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LogPlayerTakeDamage{
    _id: ObjectId,
    #[serde(rename="attackId")]
    attack_id: u64,
    attacker: Option<primatives::Account>,
    victim: Option<primatives::Account>,
    #[serde(rename="damageTypeCategory")]
    damage_type_category: Option<String>,
    #[serde(rename="damageReason")]
    damage_reason: Option<String>,
    damage: f64,
    #[serde(rename="damageCauserName")]
    damage_causer_name: Option<String>,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogPlayerUseThrowable{
    _id: ObjectId,
    attack_id: u64,
    fire_weapon_stack_count: u64,
    attacker: Option<primatives::Account>,
    attack_type: String,
    wepon: primatives::Item,
    common: primatives::Common,
    #[serde(rename="_V")]
    _v: Option<i8>,
    #[serde(rename="_D")]
    pub _d: String,
    #[serde(rename="_T")]
    _t: Option<String>,
    mongo_match_id: Option<ObjectId>,
}




/*
################
# Minimization #
################
*/
fn default_account() -> primatives::Account{
    primatives::Account{
        account_id:Some("".to_string()),
        name: Some("".to_string()),
        team_id: Some(-5.0),
        health: Some(0.0),
        location: Some(primatives::Location{ x: Some(0.0), y: Some(0.0), z: Some(0.0) }),
        ranking: Some(0.0) }
}

fn unpack(_id: ObjectId, account: Option<primatives::Account>,match_id:Option<ObjectId>, _d: String) -> Minimal{
    let account_id = account.unwrap_or_else(default_account).account_id.unwrap_or(DEFAULT_STRING.to_string());
    let mongo_match_id = match_id.unwrap_or(ObjectId::from_str(&"612aa499b9574543cbceb4ac".to_string()).unwrap());
    return Minimal{_id,account_id,mongo_match_id,_d};
}


pub const DEFAULT_STRING: &str = "DEFAULT STRING";


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Minimal{
    pub _id: ObjectId, 
    pub account_id: String,
    pub mongo_match_id:ObjectId,
    pub _d: String,
}

pub trait Minable {
    fn to_min(&self)->Minimal;
}

impl Minable for Minimal{fn to_min(&self) ->Minimal {self.clone()}}

impl Minable for LogArmorDestroy{
    fn to_min(&self) ->Minimal {
         unpack(self._id, self.attacker.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemAttach{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(), self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemDetatch{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(), self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemDrop{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(), self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogHeal{
    fn to_min(&self)->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogGameStatePeriodic{
    fn to_min(&self)->Minimal {
        Minimal{
            _id: self._id,
            account_id: "any".to_string(),
            mongo_match_id: self.mongo_match_id.unwrap_or(ObjectId::from_str(&"612aa499b9574543cbceb4ac".to_string()).unwrap()),
            _d: self._d.clone()
        }
    }
}

impl Minable for LogItemUnequip{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemUse{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogParachuteLanding{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerAttack{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.attacker.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerKill{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.killer.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerMakeGroggy{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.attacker.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerRevive{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.reviver.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerTakeDamage{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.attacker.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogPlayerUseThrowable{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.attacker.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemEquip{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}

impl Minable for LogItemPickup{
    fn to_min(&self) ->Minimal {
        unpack(self._id, self.character.clone(),self.mongo_match_id, self._d.clone())
    }
}