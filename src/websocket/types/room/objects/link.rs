//! `StructureLink` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// A link structure - a structure that can be filled with energy, then instantly send energy to other links
    /// in the same room.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLink {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// The number of ticks till this link can be used to send energy again.
        pub cooldown: i32,
        /// A record of all actions this structure performed last tick.
        pub action_log: StructureLinkActions,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - cooldown: i32,
        - action_log: StructureLinkActions,
        - notify_when_attacked: bool,
    }
}

with_update_struct! {
    /// A struct describing a room's reservation.
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkActions {
        /// The x,y position the link last transfered energy to.
        pub transfer_energy: Option<EnergyTransferTarget>,
    }

    /// The update structure for StructureLinkActions.
    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkActionsUpdate { ... }
}

with_update_struct! {
    /// A struct describing the destination of a link's energy transfer.
    ///
    /// TODO: share this structure as a more generic thing when creep actionLog is implemented.
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    pub struct EnergyTransferTarget {
        /// The in-room x position of this target.
        pub x: u16,
        /// The in-room x position of this target.
        pub y: u16,
    }

    /// The update structure for `EnergyTransferTarget`.
    #[derive(Deserialize, Clone, Debug)]
    pub struct EnergyTransferTargetUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde_json;
    use serde::Deserialize;

    use data::RoomName;

    use super::{StructureLink, StructureLinkActions, EnergyTransferTarget};

    #[test]
    fn parse_link_and_updates() {
        let json = json!({
            "_id": "57fdb3ea3dad49a17265ecea",
            "actionLog": {
                "transferEnergy": null
            },
            "cooldown": 3,
            "energy": 100,
            "energyCapacity": 800,
            "hits": 1000,
            "hitsMax": 1000,
            "notifyWhenAttacked": true,
            "room": "E17N55",
            "type": "link",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 9,
            "y": 6
        });

        let mut obj = StructureLink::deserialize(json).unwrap();

        assert_eq!(obj, StructureLink {
            room: RoomName::new("E17N55").unwrap(),
            x: 9,
            y: 6,
            id: "57fdb3ea3dad49a17265ecea".to_owned(),
            energy: 100,
            energy_capacity: 800,
            hits: 1000,
            hits_max: 1000,
            notify_when_attacked: true,
            disabled: false,
            cooldown: 3,
            action_log: StructureLinkActions {
                transfer_energy: None,
            },
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });

        obj.update(serde_json::from_value(json!({
            "cooldown": 2
        }))
            .unwrap());

        obj.update(serde_json::from_value(json!({
            "cooldown": 1
        }))
            .unwrap());

        obj.update(serde_json::from_value(json!({
            "cooldown": 0
        }))
            .unwrap());

        assert_eq!(obj.cooldown, 0);

        obj.update(serde_json::from_value(json!({
            "actionLog": {
                "transferEnergy": {
                    "x": 9,
                    "y": 18
                }
            },
            "cooldown": 11,
            "energy": 0
        }))
            .unwrap());

        assert_eq!(obj.action_log, StructureLinkActions {
            transfer_energy: Some(EnergyTransferTarget {
                x: 9,
                y: 18,
            })
        });

        obj.update(serde_json::from_value(json!({
            "actionLog": {
                "transferEnergy": null
            },
            "cooldown": 10,
            "energy": 50
        }))
            .unwrap());

        assert_eq!(obj, StructureLink {
            room: RoomName::new("E17N55").unwrap(),
            x: 9,
            y: 6,
            id: "57fdb3ea3dad49a17265ecea".to_owned(),
            energy: 50,
            energy_capacity: 800,
            hits: 1000,
            hits_max: 1000,
            notify_when_attacked: true,
            disabled: false,
            cooldown: 10,
            action_log: StructureLinkActions {
                transfer_energy: None,
            },
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }
}
