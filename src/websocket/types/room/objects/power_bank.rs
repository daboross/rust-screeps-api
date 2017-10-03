//! `StructurePowerBank` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// A power bank object, which can be attacked by creeps, and when killed will yield a harvest of power.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructurePowerBank {
        /// The game time at which this power bank will cease to exist, if it has not been killed by then.
        pub decay_time: u32,
        /// The amount of power held in this power bank - this amount will be dropped when the power bank's
        /// hits drops to 0.
        pub power: i32,
    }

    /// The update structure for a `StructurePowerBank`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructurePowerBankUpdate { ... }
}


#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::StructurePowerBank;

    #[test]
    fn parse_power_bank() {
        let json = json!({
            "_id": "59695d5c09c7343d8a4192fd",
            "decayTime": 20238724,
            "hits": 2000000,
            "hitsMax": 2000000,
            "power": 3186,
            "room": "W66N20",
            "type": "powerBank",
            "x": 24,
            "y": 40
        });

        let obj = StructurePowerBank::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructurePowerBank {
                room: RoomName::new("W66N20").unwrap(),
                x: 24,
                y: 40,
                id: "59695d5c09c7343d8a4192fd".to_owned(),
                decay_time: 20238724,
                hits: 2000000,
                hits_max: 2000000,
                power: 3186,
            }
        );
    }
}
