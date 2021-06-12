// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use debug_interface::NodeDebugClient;
use diem_client::Client as JsonRpcClient;
use diem_config::{config::NodeConfig, network_id::NetworkId};
use diem_sdk::types::PeerId;
use std::collections::HashMap;
use url::Url;

/// A NodeId is intended to be a unique identifier of a Node in a Swarm. Due to VFNs sharing the
/// same PeerId as their Validator, another identifier is needed in order to distinguish between
/// the two.
pub struct NodeId(usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        NodeId(id)
    }

    /// Accessor for the name of the module
    pub fn as_inner(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub enum HealthCheckError {
    NotRunning,
    RpcFailure(anyhow::Error),
    Unknown(anyhow::Error),
}

impl std::fmt::Display for HealthCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HealthCheckError {}

/// Trait used to represent a running Validator or FullNode
pub trait Node {
    /// Return the PeerId of this Node
    fn peer_id(&self) -> PeerId;

    /// Return the NodeId of this Node
    fn node_id(&self) -> NodeId;

    /// Return the URL for the JSON-RPC endpoint of this Node
    fn json_rpc_endpoint(&self) -> Url;

    /// Return the URL for the debug-interface for this Node
    fn debug_endpoint(&self) -> Url;

    /// Return a reference to the Config this Node is using
    fn config(&self) -> &NodeConfig;

    /// Start this Node.
    /// This should be a noop if the Node is already running.
    fn start(&mut self) -> Result<()>;

    /// Stop this Node.
    /// This should be a noop if the Node isn't running.
    fn stop(&mut self) -> Result<()>;

    /// Restarts this Node by calling Node::Stop followed by Node::Start
    fn restart(&mut self) -> Result<()> {
        self.stop()?;
        self.start()
    }

    /// Clears this Node's Storage
    fn clear_storage(&mut self) -> Result<()>;

    /// Performs a Health Check on the Node
    fn health_check(&mut self) -> Result<(), HealthCheckError>;
}

/// Trait used to represent a running Validator
pub trait Validator: Node {
    fn check_connectivity(&self, expected_peers: usize) -> Result<bool> {
        if expected_peers == 0 {
            return Ok(true);
        }

        self.get_connected_peers(NetworkId::Validator, None)
            .map(|maybe_n| maybe_n.map(|n| n >= expected_peers as i64).unwrap_or(false))
    }
}

/// Trait used to represent a running FullNode
pub trait FullNode: Node {}

impl<T: ?Sized> NodeExt for T where T: Node {}

pub trait NodeExt: Node {
    /// Return JSON-RPC client of this Node
    fn json_rpc_client(&self) -> JsonRpcClient {
        JsonRpcClient::new(self.json_rpc_endpoint().to_string())
    }

    /// Return a NodeDebugClient for this Node
    fn debug_client(&self) -> NodeDebugClient {
        NodeDebugClient::from_url(self.debug_endpoint())
    }

    /// Restarts this Node by calling Node::Stop followed by Node::Start
    fn restart(&mut self) -> Result<()> {
        self.stop()?;
        self.start()
    }

    /// Query a Metric for from this Node
    fn get_metric(&self, metric_name: &str) -> Result<Option<i64>> {
        self.debug_client().get_node_metric(metric_name)
    }

    fn get_metric_with_fields(
        &self,
        metric_name: &str,
        fields: HashMap<String, String>,
    ) -> Result<Option<i64>> {
        let filtered: Vec<_> = self
            .debug_client()
            .get_node_metric_with_name(metric_name)?
            .into_iter()
            .flat_map(|map| map.into_iter())
            .filter_map(|(metric, metric_value)| {
                if fields
                    .iter()
                    .all(|(key, value)| metric.contains(&format!("{}={}", key, value)))
                {
                    Some(metric_value)
                } else {
                    None
                }
            })
            .collect();

        Ok(if filtered.is_empty() {
            None
        } else {
            Some(filtered.iter().sum())
        })
    }

    fn get_connected_peers(
        &self,
        network_id: NetworkId,
        direction: Option<&str>,
    ) -> Result<Option<i64>> {
        let mut map = HashMap::new();
        map.insert("network_id".to_string(), network_id.to_string());
        if let Some(direction) = direction {
            map.insert("direction".to_string(), direction.to_string());
        }
        self.get_metric_with_fields("diem_connections", map)
    }
}