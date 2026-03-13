// vim: fdm=indent fdn=1
//
// Functions for managing the network communication protocol

use crate::net::{list::get_peer_list, remote};
use std::collections::HashSet;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, interval, timeout};

const PEER_REFRESH_RATE: u64 = 8;
const PEER_REFRESH_CANCEL_TIMEOUT: u64 = 2;
const MAX_PEERS: usize = 4;

static REFRESH_TASK_MUTEX: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();

/**
 * Spawns or restarts the peer refresh task
 */
pub async fn refresh_peers() {
    let mutex = REFRESH_TASK_MUTEX.get_or_init(|| Mutex::new(None));
    let mut lock = mutex.lock().await;

    // check for an existing task
    if let Some(handle) = lock.take() {
        handle.abort();
        // wait for task to finish but use a timeout
        match timeout(Duration::from_secs(PEER_REFRESH_CANCEL_TIMEOUT), handle).await {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Warning: Previous task is stuck! Spawning new one anyway.");
            }
        }
    }

    let spawned_task_handle = tokio::spawn(async {
        let mut i = interval(Duration::from_secs(PEER_REFRESH_RATE));
        i.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            i.tick().await;
            refresh_peers_task().await;
        }
    });

    *lock = Some(spawned_task_handle);
}

/**
 * Refresh peers list
 */
async fn refresh_peers_task() {
    let peers = get_peer_list().await.unwrap_or(HashSet::new());

    // TODO validate current peer list, drop unresponsive peers

    if peers.len() < MAX_PEERS {
        let mut new_peers: HashSet<String> = HashSet::new();
        for p in peers {
            let more_peers = remote::request_peers(p).await.unwrap_or(Vec::new());
            if !more_peers.is_empty() {
                println!("{}", more_peers.join(":"));
                for mp in more_peers {
                    new_peers.insert(mp);
                }
            }
        }
    }

    // TODO validate new peers
}
