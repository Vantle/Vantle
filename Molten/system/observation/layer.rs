use hypergraph::Hypergraph;
use record::warn;
use stream::Update;
use tokio::sync::mpsc::Sender;

pub fn emit<T>(sender: &Sender<Update>, graph: &Hypergraph<T>, trigger: &str)
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    match state::capture(graph, trigger) {
        Ok(snapshot) => {
            if let Err(e) = sender.try_send(Update::Snapshot(snapshot)) {
                warn!("snapshot: {}", e);
            }
        }
        Err(e) => warn!("capture: {}", e),
    }
}
