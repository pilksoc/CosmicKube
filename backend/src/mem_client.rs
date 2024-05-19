use std::collections::{BTreeMap, HashMap};
use redis::Commands;
use thiserror::Error;
use uuid::Uuid;
use cosmic_kube::Coordinate;
use cosmic_kube::grid::Grid;
use cosmic_kube::player::Player;
use cosmic_kube::space::{Space, SpaceKind};

#[derive(Debug, Error)]
/// Errors while parsing the environment variable for the endpoint URL.
pub enum EnvVarParseError {
    #[error("Error while getting environment variable. Have you included an equals sign, and properly escaped it?")]
    /// Used when the environment variable hasn't been set properly (such as a missing equals sign).
    EnvironmentError(#[from] std::env::VarError),
    #[error("Failed to connect to the server, is the connection URL valid?")]
    /// Used when an error occurs when connecting or interacting with the Redis client
    /// TODO: check what errors the module actually puts out
    RedisError(#[from] redis::RedisError)
}

#[derive(Debug, Error)]
/// Errors while communicating with the REST cache server.
pub enum RedisError {
    #[error("Error while trying to reach REST server.")]
    /// Failed to establish connection to the REST server, or got another error (e.g. 404).
    ReqwestError(#[from] reqwest::Error),
    #[error("Could not parse JSON.")]
    /// Couldn't parse the JSON, usually because something else went wrong when trying to establish a connection to the server.
    SerdeError(#[from] serde_json::Error),
}

/// A bare-bones implementation of the client that interacts with the redis/valkey server.
#[derive(Debug)]
pub struct MemClient {
    /// The currently connected client.
    client: redis::Client
}
impl MemClient {
    /// Create a new client using an environment variable.
    ///
    /// # Errors
    /// Will error when the environment variable isn't defined correctly, or if the URL couldn't be parsed.
    pub fn new() -> Result<MemClient, EnvVarParseError> {
        let url_string = std::env::var("MEM_ENDPOINT")?;
        Ok(MemClient {
            client: redis::Client::open(url_string).unwrap()?,
        })
    }
    /// Create a new client using a given connection URL.
    ///
    /// # Errors
    ///
    /// Will error if the URL couldn't be parsed.
    pub fn from_url(url: &str) -> Result<MemClient, url::ParseError> {
        Ok(MemClient {
            client: redis::Client::open(url).unwrap()?,
        })
    }

    /// Insert an entity into the memory namespace
    ///
    /// # Errors
    ///
    /// Some redis error (TODO: find out what happens.)
    pub fn insert(&mut self, space: Space) {
        let mut con = self.client.get_connection().unwrap();

        /// If the entity is a player, update their reference in the player namespace
        if let SpaceKind::Player(player) = &space.contains {
            con.sadd(format!("grid:player:{uuid}", uuid=String::from(player.uuid)), &space.coordinate)?;
        }
        /// Store the kind of entity in the space
        con.set(format!("grid:spaces:{x}-{y}", x=space.coordinate[0], y=space.coordinate[1]), &("kind", String::from(space.contains)))?;
    }

    pub fn remove(&mut self, coordinate: Coordinate) -> Option<Space> {
        let mut con = self.client.get_connection().unwrap();



        if let Some(space) = self.spaces.remove(&coordinate) {
            if let SpaceKind::Player(player) = &space.contains {
                let _ = self.players.remove(&player.uuid);
            }            Some(space)
        }
        else {
            None
        }
    }
}