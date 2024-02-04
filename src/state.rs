use crate::utils::traits::BackendConnector;
pub struct State<B: BackendConnector> {
    backends: Vec<B>,
}
