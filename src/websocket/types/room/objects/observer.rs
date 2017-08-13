//! `StructureObserver` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// An observer structure - a structure that give each player room information on one other room
    /// within 10 room distance each tick.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureObserver {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current room this observer is observing.
        #[serde(rename = "observeRoom")]
        pub observed: Option<RoomName>,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a `StructureObserver`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureObserverUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        #[serde(rename = "observeRoom")]
        - observed: Option<RoomName>,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde_json;
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureObserver;

    #[test]
    fn parse_observer_and_update() {
        let json = json!({
            "_id": "582587dd871f73007b177b0f",
            "hits": 500,
            "hitsMax": 500,
            "notifyWhenAttacked": true,
            "observeRoom": "E5N20",
            "room": "E9N23",
            "type": "observer",
            "user": "561e4d4645f3f7244a7622e8",
            "x": 20,
            "y": 18
        });

        let mut obj = StructureObserver::deserialize(json).unwrap();

        assert_eq!(obj, StructureObserver {
            room: RoomName::new("E9N23").unwrap(),
            x: 20,
            y: 18,
            id: "582587dd871f73007b177b0f".to_owned(),
            hits: 500,
            hits_max: 500,
            notify_when_attacked: true,
            disabled: false,
            observed: Some(RoomName::new("E5N20").unwrap()),
            user: "561e4d4645f3f7244a7622e8".to_owned(),
        });

        obj.update(
            serde_json::from_value(json!({
            "observeRoom": "E4N20"
        })).unwrap(),
        );

        assert_eq!(obj, StructureObserver {
            room: RoomName::new("E9N23").unwrap(),
            x: 20,
            y: 18,
            id: "582587dd871f73007b177b0f".to_owned(),
            hits: 500,
            hits_max: 500,
            notify_when_attacked: true,
            disabled: false,
            observed: Some(RoomName::new("E4N20").unwrap()),
            user: "561e4d4645f3f7244a7622e8".to_owned(),
        });

        obj.update(
            serde_json::from_value(json!({
            "observeRoom": null
        })).unwrap(),
        );

        assert_eq!(obj, StructureObserver {
            room: RoomName::new("E9N23").unwrap(),
            x: 20,
            y: 18,
            id: "582587dd871f73007b177b0f".to_owned(),
            hits: 500,
            hits_max: 500,
            notify_when_attacked: true,
            disabled: false,
            observed: None,
            user: "561e4d4645f3f7244a7622e8".to_owned(),
        });
    }
}
