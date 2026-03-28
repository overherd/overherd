// vim: fdm=indent fdn=1
//
// Functions for managing the network communication protocol

use crate::net::INSTANCE_ID;
use crate::net::list::update_peer_list;
use crate::net::{list::get_peer_list, remote};
use std::collections::HashSet;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, interval, timeout};

const PEER_REFRESH_RATE: u64 = 8;
const PEER_REFRESH_CANCEL_TIMEOUT: u64 = 2;
const MAX_PEERS: usize = 4;
const PEER_ROTATION: usize = MAX_PEERS / 2;

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

    let mut valid_peers: HashSet<String> = HashSet::new();
    // check which current peers are still reacheable
    for p in peers {
        match remote::send_request_id(&p).await {
            Ok(_) => {
                valid_peers.insert(p);
            }
            Err(err) => eprintln!("{}", err),
        };
    }

    // if we have space for more peers
    // request more peers
    // TODO perform this concurrently and pick random peers from response
    'all_peers: for p in &valid_peers.clone() {
        let more_peers = remote::send_request_peers(p).await.unwrap_or(Vec::new());
        if more_peers.is_empty() {
            continue;
        }
        println!("  > {}", more_peers.join(":"));
        for mp in more_peers {
            match remote::send_request_id(&mp).await {
                Ok(uuid) => {
                    // if it's not our id we add it to the valid_peers list
                    if uuid != *INSTANCE_ID.get_or_init(|| String::from("")) {
                        valid_peers.insert(mp);
                    }
                }
                Err(err) => eprintln!("{}", err),
            };
            if valid_peers.len() >= MAX_PEERS {
                break 'all_peers;
            }
        }
    }
    let _ = update_peer_list(valid_peers.into_iter().collect()).await;
}
