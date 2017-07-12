use data::RoomName;


with_structure_fields_and_update_struct! {
    /// A wall structure - a structure that has a large amount of possible hit points.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWall {
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a wall structure.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWallUpdate {
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureWall;

    #[test]
    fn parse_wall() {
        let json = json!({
            "_id": "584a5d89cbe27a302e4ba889",
            "hits": 7222461,
            "hitsMax": 300000000,
            "notifyWhenAttacked": true,
            "room": "W73N43",
            "type": "constructedWall",
            "x": 47,
            "y": 24
        });

        let obj = StructureWall::deserialize(json).unwrap();

        assert_eq!(obj, StructureWall {
            room: RoomName::new("W73N43").unwrap(),
            x: 47,
            y: 24,
            id: "584a5d89cbe27a302e4ba889".to_owned(),
            hits: 7222461,
            hits_max: 300000000,
            notify_when_attacked: true,
        });
    }
}
