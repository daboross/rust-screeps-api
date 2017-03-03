//! Room overview retrieval
//! {
//!     ok,
//!     owner: {
//!         username,
//!         badge: { type, color1, color2, color3, param, flip }
//!     },
//!     stats: {
//!         energyHarvested: [ { value, endTime } ],
//!         energyConstruction: [ { value, endTime } ],
//!         energyCreeps: [ { value, endTime } ],
//!         energyControl: [ { value, endTime } ],
//!         creepsProduced: [ { value, endTime } ],
//!         creepsLost: [ { value, endTime } ]
//!     },
//!     statsMax: {
//!         energy1440,
//!         energyCreeps1440,
//!         energy8,
//!         energyControl8,
//!         creepsLost180,
//!         energyHarvested8,
//!         energy180,
//!         energyConstruction180,
//!         creepsProduced8,
//!         energyControl1440,
//!         energyCreeps8,
//!         energyHarvested1440,
//!         creepsLost1440,
//!         energyConstruction1440,
//!         energyHarvested180,
//!         creepsProduced180,
//!         creepsProduced1440,
//!         energyCreeps180,
//!         energyControl180,
//!         energyConstruction8,
//!         creepsLost8
//!     }
//! }

use EndpointResult;
use data::{self, Badge};
use error::{ApiError, Result};
use std::marker::PhantomData;

/// Room overview raw result.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Response {
    ok: i32,
    owner: Option<OwnerResponse>,
    stats: Option<RoomStatsResponse>,
    statsMax: Option<RoomTotalStatsResponse>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OwnerResponse {
    username: String,
    badge: Badge,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RoomStatsResponse {
    energyHarvested: Vec<StatPointResponse>,
    energyConstruction: Vec<StatPointResponse>,
    energyCreeps: Vec<StatPointResponse>,
    energyControl: Vec<StatPointResponse>,
    creepsProduced: Vec<StatPointResponse>,
    creepsLost: Vec<StatPointResponse>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
struct StatPointResponse {
    value: u32,
    endTime: u32,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RoomTotalStatsResponse {
    energy8: u32,
    energy180: u32,
    energy1440: u32,
    energyConstruction8: u32,
    energyConstruction180: u32,
    energyConstruction1440: u32,
    energyControl8: u32,
    energyControl180: u32,
    energyControl1440: u32,
    energyCreeps8: u32,
    energyCreeps180: u32,
    energyCreeps1440: u32,
    creepsProduced8: u32,
    creepsProduced180: u32,
    creepsProduced1440: u32,
    creepsLost8: u32,
    creepsLost180: u32,
    creepsLost1440: u32,
}

/// A single statistics point, representing a quantity for data over an interval of time.
#[derive(Debug, Copy, Clone)]
pub struct StatPoint {
    /// The amount of whatever quantity this stat point is for
    pub amount: u32,
    /// The end time that this stat point is for.
    pub end_time: u32,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

impl From<StatPointResponse> for StatPoint {
    fn from(stat: StatPointResponse) -> StatPoint {
        StatPoint {
            amount: stat.value,
            end_time: stat.endTime,
            _phantom: PhantomData,
        }
    }
}

/// Total stats over a specific time period.
#[derive(Debug, Copy, Clone)]
pub struct TotalStats {
    /// Time period. Currently either "8" for hour long stats, "180" for day long stats, or "1440" for week-long stats.
    pub time_period: u32,
    /// Energy harvested during this time period
    pub energy_harvested: u32,
    /// Energy spent on creeps during this time period
    pub energy_spent_creeps: u32,
    /// Energy spent on control during this time period
    pub energy_spent_control: u32,
    /// Energy spent on construction during this time period
    pub energy_spent_construction: u32,
    /// Creep parts produced during this time period
    pub creep_parts_produced: u32,
    /// Creep parts lots during this time period
    creep_parts_lost: u32,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

/// The result of a room_overview call.
#[derive(Debug, Clone)]
pub struct RoomOverview {
    /// The username of the owner of the room.
    pub owner: Option<String>,
    /// The owner's badge
    pub owner_badge: Option<Badge>,
    /// Energy harvested during each interval of the requested time.
    pub energy_harvested: Vec<StatPoint>,
    /// Energy spent on creeps during each interval of the requested time.
    pub energy_spent_creeps: Vec<StatPoint>,
    /// Energy spent on control during each interval of the requested time.
    pub energy_spent_control: Vec<StatPoint>,
    /// Energy spent on construction during each interval of the requested time.
    pub energy_spent_construction: Vec<StatPoint>,
    /// Number of creep parts produced during each interval of the requested time.
    pub creep_parts_produced: Vec<StatPoint>,
    /// Number of creep parts lost during each interval of the requested time.
    pub creep_parts_lost: Vec<StatPoint>,
    /// A list of all total statistics provided (usually hour long, day long, and week long returned)
    pub total_stats: Vec<TotalStats>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

impl EndpointResult for RoomOverview {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RoomOverview> {
        let Response { ok, owner, stats, statsMax: stats_max, .. } = raw;
        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        let (username, badge) = match owner {
            Some(v) => (Some(v.username), Some(v.badge)),
            None => (None, None),
        };
        let stats = match stats {
            Some(v) => v,
            None => return Err(ApiError::MissingField("stats").into()),
        };
        let stats_max = match stats_max {
            Some(v) => v,
            None => return Err(ApiError::MissingField("statsMax").into()),
        };

        Ok(RoomOverview {
            owner: username,
            owner_badge: badge,
            energy_harvested: stats.energyHarvested.into_iter().map(Into::into).collect(),
            energy_spent_construction: stats.energyConstruction.into_iter().map(Into::into).collect(),
            energy_spent_creeps: stats.energyCreeps.into_iter().map(Into::into).collect(),
            energy_spent_control: stats.energyControl.into_iter().map(Into::into).collect(),
            creep_parts_produced: stats.creepsProduced.into_iter().map(Into::into).collect(),
            creep_parts_lost: stats.creepsLost.into_iter().map(Into::into).collect(),
            total_stats: vec![TotalStats {
                                  time_period: 8,
                                  energy_harvested: stats_max.energy8,
                                  energy_spent_creeps: stats_max.energyCreeps8,
                                  energy_spent_control: stats_max.energyControl8,
                                  energy_spent_construction: stats_max.energyConstruction8,
                                  creep_parts_produced: stats_max.creepsProduced8,
                                  creep_parts_lost: stats_max.creepsLost8,
                                  _phantom: PhantomData,
                              },
                              TotalStats {
                                  time_period: 180,
                                  energy_harvested: stats_max.energy180,
                                  energy_spent_creeps: stats_max.energyCreeps180,
                                  energy_spent_control: stats_max.energyControl180,
                                  energy_spent_construction: stats_max.energyConstruction180,
                                  creep_parts_produced: stats_max.creepsProduced180,
                                  creep_parts_lost: stats_max.creepsLost180,
                                  _phantom: PhantomData,
                              },
                              TotalStats {
                                  time_period: 1440,
                                  energy_harvested: stats_max.energy1440,
                                  energy_spent_creeps: stats_max.energyCreeps1440,
                                  energy_spent_control: stats_max.energyControl1440,
                                  energy_spent_construction: stats_max.energyConstruction1440,
                                  creep_parts_produced: stats_max.creepsProduced1440,
                                  creep_parts_lost: stats_max.creepsLost1440,
                                  _phantom: PhantomData,
                              }],
            _phantom: PhantomData,
        })
    }
}
