//! `StructureRoad` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// A road structure - a structure that speeds up creeps without sufficient move parts.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRoad {
        /// The next game tick when this structure's hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a road structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRoadUpdate {
        - next_decay_time: u32,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde_json;
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureRoad;

    #[test]
    fn parse_road_and_update() {
        let json = json!({
            "_id": "58a1ec36947c6c2d324a2d39",
            "hits": 2600,
            "hitsMax": 5000,
            "nextDecayTime": 19894066,
            "notifyWhenAttacked": true,
            "room": "E4S61",
            "type": "road",
            "x": 14,
            "y": 20
        });

        let mut obj = StructureRoad::deserialize(json).unwrap();

        assert_eq!(obj, StructureRoad {
            room: RoomName::new("E4S61").unwrap(),
            x: 14,
            y: 20,
            id: "58a1ec36947c6c2d324a2d39".to_owned(),
            hits: 2600,
            hits_max: 5000,
            next_decay_time: 19894066,
            notify_when_attacked: true,
        });

        obj.update(
            serde_json::from_value(json!({
            // note: these are fake values, not a real update.
            "hits": 2000,
            "nextDecayTime": 20000000,
        })).unwrap(),
        );

        assert_eq!(obj, StructureRoad {
            room: RoomName::new("E4S61").unwrap(),
            x: 14,
            y: 20,
            id: "58a1ec36947c6c2d324a2d39".to_owned(),
            hits: 2000,
            hits_max: 5000,
            next_decay_time: 20000000,
            notify_when_attacked: true,
        });
    }
}
