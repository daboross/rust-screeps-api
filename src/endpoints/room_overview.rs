//! Interpreting room overview results.

use data::{self, Badge};
use error::{ApiError, Result};
use std::marker::PhantomData;
use EndpointResult;

/// Room overview raw result.
#[derive(Deserialize, Clone, Hash, Debug)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    owner: Option<OwnerResponse>,
    stats: Option<RoomStatsResponse>,
    stats_max: Option<RoomTotalStatsResponse>,
}

#[derive(Deserialize, Clone, Hash, Debug)]
struct OwnerResponse {
    username: String,
    badge: Badge,
}

#[derive(Deserialize, Clone, Hash, Debug)]
#[serde(rename_all = "camelCase")]
struct RoomStatsResponse {
    energy_harvested: Vec<StatPointResponse>,
    energy_construction: Vec<StatPointResponse>,
    energy_creeps: Vec<StatPointResponse>,
    energy_control: Vec<StatPointResponse>,
    creeps_produced: Vec<StatPointResponse>,
    creeps_lost: Vec<StatPointResponse>,
}

#[derive(Deserialize, Copy, Clone, Hash, Debug)]
#[serde(rename_all = "camelCase")]
struct StatPointResponse {
    value: u32,
    end_time: u32,
}

#[derive(Deserialize, Copy, Clone, Hash, Debug)]
#[serde(rename_all = "camelCase")]
struct RoomTotalStatsResponse {
    energy_8: u32,
    energy_180: u32,
    energy_1440: u32,
    energy_construction_8: u32,
    energy_construction_180: u32,
    energy_construction_1440: u32,
    energy_control_8: u32,
    energy_control_180: u32,
    energy_control_1440: u32,
    energy_creeps_8: u32,
    energy_creeps_180: u32,
    energy_creeps_1440: u32,
    creeps_produced_8: u32,
    creeps_produced_180: u32,
    creeps_produced_1440: u32,
    creeps_lost_8: u32,
    creeps_lost_180: u32,
    creeps_lost_1440: u32,
}

/// A single statistics point, representing a quantity for data over an interval of time.
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Debug)]
pub struct StatPoint {
    /// The amount of whatever quantity this stat point is for
    pub amount: u32,
    /// The end time that this stat point is for.
    pub end_time: u32,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

impl From<StatPointResponse> for StatPoint {
    fn from(stat: StatPointResponse) -> StatPoint {
        StatPoint {
            amount: stat.value,
            end_time: stat.end_time,
            _phantom: PhantomData,
        }
    }
}

/// Total stats over a specific time period.
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Debug)]
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
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Various statistics about a single room, returned as a result from `room_overview` calls.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
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
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

impl EndpointResult for RoomOverview {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RoomOverview> {
        let Response {
            ok,
            owner,
            stats,
            stats_max,
            ..
        } = raw;
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
            energy_harvested: stats.energy_harvested.into_iter().map(Into::into).collect(),
            energy_spent_construction: stats
                .energy_construction
                .into_iter()
                .map(Into::into)
                .collect(),
            energy_spent_creeps: stats.energy_creeps.into_iter().map(Into::into).collect(),
            energy_spent_control: stats.energy_control.into_iter().map(Into::into).collect(),
            creep_parts_produced: stats.creeps_produced.into_iter().map(Into::into).collect(),
            creep_parts_lost: stats.creeps_lost.into_iter().map(Into::into).collect(),
            total_stats: vec![
                TotalStats {
                    time_period: 8,
                    energy_harvested: stats_max.energy_8,
                    energy_spent_creeps: stats_max.energy_creeps_8,
                    energy_spent_control: stats_max.energy_control_8,
                    energy_spent_construction: stats_max.energy_construction_8,
                    creep_parts_produced: stats_max.creeps_produced_8,
                    creep_parts_lost: stats_max.creeps_lost_8,
                    _phantom: PhantomData,
                },
                TotalStats {
                    time_period: 180,
                    energy_harvested: stats_max.energy_180,
                    energy_spent_creeps: stats_max.energy_creeps_180,
                    energy_spent_control: stats_max.energy_control_180,
                    energy_spent_construction: stats_max.energy_construction_180,
                    creep_parts_produced: stats_max.creeps_produced_180,
                    creep_parts_lost: stats_max.creeps_lost_180,
                    _phantom: PhantomData,
                },
                TotalStats {
                    time_period: 1440,
                    energy_harvested: stats_max.energy_1440,
                    energy_spent_creeps: stats_max.energy_creeps_1440,
                    energy_spent_control: stats_max.energy_control_1440,
                    energy_spent_construction: stats_max.energy_construction_1440,
                    creep_parts_produced: stats_max.creeps_produced_1440,
                    creep_parts_lost: stats_max.creeps_lost_1440,
                    _phantom: PhantomData,
                },
            ],
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RoomOverview;
    use serde_json;
    use EndpointResult;

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
