//! `StructureTerminal` data description.
use super::super::resources::ResourceType;
use super::super::resources::Store;
use crate::data::RoomName;

with_base_fields_and_update_struct! {
    /// A terminal structure - a structure that has a medium capacity for storing multiple resources,
    /// and can spend energy to send any resources to another room with a terminal instantly.
    // Note: there's a field "send" which seems to always be null. It will be non-existent on a
    // terminal which has never sent any resources, and then will be updated to "null" on the first send.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTerminal {
        /// The current number of hit-points this structure has.
        #[serde(default)]
        pub hits: i32,
        /// The maximum number of hit-points this structure has.
        #[serde(default)]
        pub hits_max: i32,
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// Total capacity for this structure.
        #[serde(rename = "storeCapacity")]
        pub capacity: i32,
        /// The game time at which this terminal will next be able to send minerals.
        #[serde(default)]
        pub cooldown_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTerminalUpdate {
        - hits: i32,
        - hits_max: i32,
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        #[serde(rename = "energyCapacity")]
        - capacity: i32,
        - cooldown_time: u32,
        - notify_when_attacked: bool,
        - store: Store,
    }
}

impl StructureTerminal {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::StructureTerminal;
    use crate::websocket::types::room::resources::ResourceType;

    #[test]
    fn parse_terminal() {
        let json = json!({
            "store": {
                "XLHO2": 0,
                "XZH2O": 0,
                "energy": 1321,
            },
            "_id": "59675fbfb554456b195342e7",
            "cooldownTime": 20236024,
            "energyCapacity": 300000,
            "hits": 3000,
            "hitsMax": 3000,
            "notifyWhenAttacked": true,
            "room": "W65N19",
            "type": "terminal",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 30,
            "y": 35
        });

        let obj = StructureTerminal::deserialize(json).unwrap();

        match obj {
            StructureTerminal {
                capacity: 300000,
                hits: 3000,
                hits_max: 3000,
                notify_when_attacked: true,
                cooldown_time: 20236024,
                disabled: false,
                x: 30,
                y: 35,
                ref user,
                ref id,
                ref store,
                ..
            } if user == "57874d42d0ae911e3bd15bbc"
                && id == "59675fbfb554456b195342e7"
                && *store == store! {Energy: 1321} =>
            {
                ()
            }

            other => panic!(
                "expected pre-set StructureTerminal to match, found {:#?}",
                other
            ),
        }

        assert_eq!(
            {
                let mut contents = obj.resources().collect::<Vec<_>>();
                contents.sort();
                contents
            },
            {
                let mut expected = vec![(ResourceType::Energy, 1321)];
                expected.sort();
                expected
            }
        );
    }

    #[test]
    fn parse_with_null_cooldown() {
        let json = json!({
            "_id": "5c78d9793b60c54624032734",
            "energy": 0,
            "energyCapacity": 300000,
            "hits": 3000,
            "hitsMax": 3000,
            "notifyWhenAttacked": true,
            "room": "E8S22",
            "type": "terminal",
            "user": "5867dc91c04c074e4f1bdd08",
            "x": 18,
            "y": 17,
        });

        let obj = StructureTerminal::deserialize(json).unwrap();

        assert_eq!(obj.cooldown_time, 0);
    }
}
