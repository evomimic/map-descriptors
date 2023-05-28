#![warn(warnings)]

// use futures::future;
// use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::sweettest::{SweetAgents, SweetCell, SweetConductor, SweetDnaFile};

use types_descriptor::HolonDescriptor;

const DNA_FILEPATH: &str = "../../workdir/map_descriptors.dna";

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_holontypes() {
    let (conductor, _agent, cell): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let testing_descriptors: Vec<HolonDescriptor> = conductor
        .call(&cell.zome("descriptors"), "get_all_holontypes", ())
        .await;

    println!("{:?}", testing_descriptors);
}

/// MOCK CONDUCTOR

async fn setup_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
    let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
        .await
        .unwrap();
    /*
    // let dna_path = std::env::current_dir()
    // .unwrap()
    // .join("../../../workdir/map-proto1.dna");

    // let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    */

    let mut conductor = SweetConductor::from_standard_config().await;

    let holo_core_agent = SweetAgents::one(conductor.keystore()).await;
    let app = conductor
        .setup_app_for_agent("app", holo_core_agent.clone(), &[dna.clone()])
        .await
        .unwrap();

    let cell = app.into_cells()[0].clone();

    let agent_hash = holo_core_agent.into_inner();
    let agent = AgentPubKey::from_raw_39(agent_hash).unwrap();

    (conductor, agent, cell)
}

//// SAVE

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_all_holons() {

//   let (conductor, _agent, cell1): (SweetConductor, AgentPubKey, SweetCell) =
//     setup_conductor().await;

//   let descriptors: Vec<HolonDescriptor> = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "get_all_holon_types")
//     .await;

//   println!(descriptors);

//   let action_hash1: ActionHash = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "create__hc_entry", descriptors[0])
//     .await;

//   let holon1: HolonDescriptor = conductor
//     .call(
//       &cell.zome("map_proto1"),
//       "get_entry_by_actionhash",
//       action_hash1,
//     )
//     .await;

//   let action_hash2: ActionHash = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "create_hc_entry", descriptors[1])
//     .await;

//   let holon2: HolonDescriptor = conductor
//   .call(
//     &cell.zome("map_proto1"),
//     "get_entry_by_actionhash",
//     action_hash2,
//   )
//   .await;

//   let action_hash3: ActionHash = conductor
//   .call(&cell.zome("hc_zome_coordination_holons"), "create_hc_entry", descriptors[2])
//   .await;

//   let holon3: HolonDescriptor = conductor
//     .call(
//       &cell.zome("map_proto1"),
//       "get_entry_by_actionhash",
//       action_hash3,
//     )
//     .await;

//   let holons: Vec<HolonDescriptor> = vec![holon1, holon2, holon3];

//   println!(holons);

//   assert_eq!(holons, descriptors);

// }
