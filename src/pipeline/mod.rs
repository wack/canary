use crate::{
    adapters::{DecisionEngine, Ingress, MockEngine, MockIngress, Monitor},
    stats::Observation,
};

// TODO: Add some more structure to this. Right now, I'm
//       just trying to get the general layout defined and
//       all of the actors wired up.
pub async fn setup_pipeline() {
    // • First, we create a monitor based on the configuration we've been given.
    //   It must use dynamic dispatch because we're not sure what kind of
    //   monitor it is.
    let _monitor: Option<Box<dyn Monitor<Item = Observation>>> = None;
    // • Repeat for the Ingress and the Engine.
    let _ingress: Box<dyn Ingress> = Box::new(MockIngress);
    let _engine: Box<dyn DecisionEngine> = Box::new(MockEngine);
    todo!();
}
