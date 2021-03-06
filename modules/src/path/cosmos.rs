use super::{ChannelPath, ClientStatePath, ConnectionPath, ConsensusStatePath};

pub fn connection_path(path: &ConnectionPath) -> String {
    format!("connection/{}", path.connection_id)
}

pub fn consensus_state_path(path: &ConsensusStatePath) -> String {
    format!("consensusState/{}/{}", path.client_id, path.height)
}

pub fn client_state_path(path: &ClientStatePath) -> String {
    format!("clientState/{}", path.client_id)
}

pub fn channel_path(path: &ChannelPath) -> String {
    format!("ports/{}/channels/{}", path.port_id, path.channel_id)
}
