#![warn(warnings)]

// use futures::future;
// use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::sweettest::{SweetAgents, SweetCell, SweetConductor, SweetDnaFile};
use types_descriptor::holon_descriptor::HolonDescriptor;

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

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_all_holon_descriptor_entries() {
    let (conductor, _agent, cell1): (SweetConductor, AgentPubKey, SweetCell) =
        setup_conductor().await;

    let descriptors: Vec<HolonDescriptor> = conductor
        .call(&cell1.zome("descriptors"), "get_all_holontypes", ())
        .await;

    println!("{:?}", descriptors);

    let action_hashes: Vec<ActionHash> = Vec::new();

    for descriptor in descriptors {
        let hash = conductor
            .call(
                &cell1.zome("descriptors"),
                "commit_holon_descriptor",
                descriptor,
            )
            .await;
        action_hashes.push(hash);
    }

    let holons: Vec<HolonDescriptor> = Vec::new();

    for hash in action_hashes {
        let entry = conductor
            .call(&cell1.zome("map_proto1"), "get_entry_by_actionhash", hash)
            .await;
    }
    holons.push(entry);

    println!("{:?}", holons);

    assert_eq!(holons, descriptors);
}
