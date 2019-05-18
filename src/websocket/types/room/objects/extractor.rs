//! `StructureExtractor` data description.
use crate::data::RoomName;

with_structure_fields_and_update_struct! {
    /// An extractor structure - a structure that can be used to harvest minerals.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtractor {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtractorUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::StructureExtractor;

    #[test]
    fn parse_extractor_sample() {
        let json = json!({
            "_id": "5cb56020f7d8904f5df7a1ae",
            "hits": 500,
            "hitsMax": 500,
            "notifyWhenAttacked": true,
            "room": "E8S29",
            "type": "extractor",
            "user": "5ca80c8f3c33e30c8e85555d",
            "x": 29,
            "y": 40,
        });

        let obj = StructureExtractor::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureExtractor {
                room: RoomName::new("E8S29").unwrap(),
                x: 29,
                y: 40,
                id: "5cb56020f7d8904f5df7a1ae".to_owned(),
                hits: 500,
                hits_max: 500,
                notify_when_attacked: true,
                disabled: false,
                user: "5ca80c8f3c33e30c8e85555d".to_owned(),
            }
        );
    }
}
