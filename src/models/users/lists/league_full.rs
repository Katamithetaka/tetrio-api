use std::sync::Arc;

use crate::models::packet::Packet;
use crate::models::users::user_rank::UserRank;
use crate::models::users::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueFullData {
    pub gamesplayed: i64,
    pub gameswon: i64,
    pub rating: f64,
    pub rank: UserRank,
    pub bestrank: Option<UserRank>,
    pub glicko: Option<f64>,
    pub rd: Option<f64>,
    pub apm: Option<f64>,
    pub pps: Option<f64>,
    pub vs: Option<f64>,
    pub decaying: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueFullUser {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub username: Arc<str>,
    pub role: UserRole,
    pub xp: f64,
    pub country: Option<Arc<str>>,
    pub supporter: Option<bool>,
    pub verified: bool,
    pub league: LeagueFullData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueFullPacketData {
    pub users: Box<[LeagueFullUser]>,
}

pub type LeagueFullPacket = Packet<LeagueFullPacketData>;
