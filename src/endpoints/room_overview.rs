//! Interpreting room overview results.

use EndpointResult;
use data::{self, Badge};
use error::{ApiError, Result};
use std::marker::PhantomData;

/// Room overview raw result.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
#[doc(hidden)]
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
    _phantom: PhantomData<()>,
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
    _phantom: PhantomData<()>,
}

/// Various statistics about a single room, returned as a result from `room_overview` calls.
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
    _phantom: PhantomData<()>,
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

#[cfg(test)]
mod tests {
    use super::RoomOverview;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = RoomOverview::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_overview_e15n52() {
        test_parse(json! ({
            "ok": 1,
            "owner": {
                "badge": {
                    "color1": "#260d0d",
                    "color2": "#6b2e41",
                    "color3": "#ffe56d",
                    "flip": false,
                    "param": -100,
                    "type": 21
                },
                "username": "daboross"
            },
            "stats": {
                "creepsLost": [
                    {
                        "endTime": 3101205,
                        "value": 0
                    },
                    {
                        "endTime": 3101206,
                        "value": 0
                    },
                    {
                        "endTime": 3101207,
                        "value": 0
                    },
                    {
                        "endTime": 3101208,
                        "value": 0
                    },
                    {
                        "endTime": 3101209,
                        "value": 0
                    },
                    {
                        "endTime": 3101210,
                        "value": 0
                    },
                    {
                        "endTime": 3101211,
                        "value": 0
                    },
                    {
                        "endTime": 3101212,
                        "value": 0
                    }
                ],
                "creepsProduced": [
                    {
                        "endTime": 3101205,
                        "value": 117
                    },
                    {
                        "endTime": 3101206,
                        "value": 8
                    },
                    {
                        "endTime": 3101207,
                        "value": 8
                    },
                    {
                        "endTime": 3101208,
                        "value": 83
                    },
                    {
                        "endTime": 3101209,
                        "value": 86
                    },
                    {
                        "endTime": 3101210,
                        "value": 128
                    },
                    {
                        "endTime": 3101211,
                        "value": 47
                    },
                    {
                        "endTime": 3101212,
                        "value": 26
                    }
                ],
                "energyConstruction": [
                    {
                        "endTime": 3101205,
                        "value": 91
                    },
                    {
                        "endTime": 3101206,
                        "value": 146
                    },
                    {
                        "endTime": 3101207,
                        "value": 89
                    },
                    {
                        "endTime": 3101208,
                        "value": 129
                    },
                    {
                        "endTime": 3101209,
                        "value": 120
                    },
                    {
                        "endTime": 3101210,
                        "value": 122
                    },
                    {
                        "endTime": 3101211,
                        "value": 107
                    },
                    {
                        "endTime": 3101212,
                        "value": 87
                    }
                ],
                "energyControl": [
                    {
                        "endTime": 3101205,
                        "value": 428
                    },
                    {
                        "endTime": 3101206,
                        "value": 825
                    },
                    {
                        "endTime": 3101207,
                        "value": 1740
                    },
                    {
                        "endTime": 3101208,
                        "value": 1755
                    },
                    {
                        "endTime": 3101209,
                        "value": 1830
                    },
                    {
                        "endTime": 3101210,
                        "value": 1875
                    },
                    {
                        "endTime": 3101211,
                        "value": 1920
                    },
                    {
                        "endTime": 3101212,
                        "value": 1425
                    }
                ],
                "energyCreeps": [
                    {
                        "endTime": 3101205,
                        "value": 6950
                    },
                    {
                        "endTime": 3101206,
                        "value": 650
                    },
                    {
                        "endTime": 3101207,
                        "value": 650
                    },
                    {
                        "endTime": 3101208,
                        "value": 4310
                    },
                    {
                        "endTime": 3101209,
                        "value": 4400
                    },
                    {
                        "endTime": 3101210,
                        "value": 9400
                    },
                    {
                        "endTime": 3101211,
                        "value": 5500
                    },
                    {
                        "endTime": 3101212,
                        "value": 1300
                    }
                ],
                "energyHarvested": [
                    {
                        "endTime": 3101205,
                        "value": 2400
                    },
                    {
                        "endTime": 3101206,
                        "value": 2500
                    },
                    {
                        "endTime": 3101207,
                        "value": 2320
                    },
                    {
                        "endTime": 3101208,
                        "value": 2340
                    },
                    {
                        "endTime": 3101209,
                        "value": 2440
                    },
                    {
                        "endTime": 3101210,
                        "value": 2500
                    },
                    {
                        "endTime": 3101211,
                        "value": 2560
                    },
                    {
                        "endTime": 3101212,
                        "value": 1900
                    }
                ],
                "powerProcessed": [
                    {
                        "endTime": 3101205,
                        "value": 0
                    },
                    {
                        "endTime": 3101206,
                        "value": 0
                    },
                    {
                        "endTime": 3101207,
                        "value": 0
                    },
                    {
                        "endTime": 3101208,
                        "value": 0
                    },
                    {
                        "endTime": 3101209,
                        "value": 0
                    },
                    {
                        "endTime": 3101210,
                        "value": 0
                    },
                    {
                        "endTime": 3101211,
                        "value": 0
                    },
                    {
                        "endTime": 3101212,
                        "value": 0
                    }
                ]
            },
            "statsMax": {
                "creepsLost1440": 8923,
                "creepsLost180": 1632,
                "creepsLost8": 226,
                "creepsProduced1440": 21797,
                "creepsProduced180": 2783,
                "creepsProduced8": 212,
                "energy1440": 12240476,
                "energy180": 1365753,
                "energy8": 94311,
                "energyConstruction1440": 12240476,
                "energyConstruction180": 1365753,
                "energyConstruction8": 94311,
                "energyControl1440": 12240476,
                "energyControl180": 1365753,
                "energyControl8": 94311,
                "energyCreeps1440": 12240476,
                "energyCreeps180": 1365753,
                "energyCreeps8": 94311,
                "energyHarvested1440": 12240476,
                "energyHarvested180": 1365753,
                "energyHarvested8": 94311,
                "power1440": 21422,
                "power180": 2708,
                "power8": 132,
                "powerProcessed1440": 21422,
                "powerProcessed180": 2708,
                "powerProcessed8": 132
            },
            "totals": {
                "creepsProduced": 503,
                "energyConstruction": 891,
                "energyControl": 11798,
                "energyCreeps": 33160,
                "energyHarvested": 18960
            }
        }));
    }
}
