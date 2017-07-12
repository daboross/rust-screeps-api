//! `StructureRampart` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// A rampart structure - a structure that has a large amount of possible hit points, and is uniquely
    /// walkable only for the owner.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRampart {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// The next game tick when this roads hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// If true, creeps not owned by the owner of this structure can also walk on it.
        #[serde(default, rename = "isPublic")]
        pub public: bool,
    }

    /// The update structure for a rampart structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRampartUpdate {
        - user: String,
        - next_decay_time: u32,
        - notify_when_attacked: bool,
        #[serde(rename = "isPublic")]
        - public: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureRampart;

    #[test]
    fn parse_rampart() {
        let json = json!({
            "_id": "58e5ae786dace5c319d5b7ee",
            "hits": 7181701,
            "hitsMax": 10000000,
            "nextDecayTime": 20179250,
            "notifyWhenAttacked": true,
            "room": "W73N43",
            "type": "rampart",
            "user": "576b572e366187105908ad57",
            "x": 29,
            "y": 35,
        });

        let obj = StructureRampart::deserialize(json).unwrap();

        assert_eq!(obj, StructureRampart {
            room: RoomName::new("W73N43").unwrap(),
            x: 29,
            y: 35,
            id: "58e5ae786dace5c319d5b7ee".to_owned(),
            hits: 7181701,
            hits_max: 10000000,
            next_decay_time: 20179250,
            notify_when_attacked: true,
            user: "576b572e366187105908ad57".to_owned(),
            public: false,
        });
    }

    #[test]
    fn parse_friendly_rampart() {
        let json = json!({
            "_id": "58a2895a578de3836ea89fbb",
            "hits": 97490601,
            "hitsMax": 300000000,
            "isPublic": true,
            "nextDecayTime": 19894001,
            "notifyWhenAttacked": true,
            "room": "E4S61",
            "type": "rampart",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 20,
            "y": 14,
        });

        let obj = StructureRampart::deserialize(json).unwrap();

        assert_eq!(obj, StructureRampart {
            room: RoomName::new("E4S61").unwrap(),
            x: 20,
            y: 14,
            id: "58a2895a578de3836ea89fbb".to_owned(),
            hits: 97490601,
            hits_max: 300000000,
            public: true,
            next_decay_time: 19894001,
            notify_when_attacked: true,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }
}
