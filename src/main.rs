use libp2p::{
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("Local Peer ID is: {new_peer_id:?}");

    let transport = libp2p::development_transport(new_key).await?;
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(remote_peer) = env::args().nth(1) {
        let remote_peer: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer.clone())?;
        println!("Dialed remote peer: {remote_peer:?}");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {address}")
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => peers
                .into_iter()
                .for_each(|(peer, addr)| println!("discovered {peer} {addr}")),
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => expired
                .into_iter()
                .for_each(|(peer, addr)| println!("expired {peer} {addr}")),
            _ => (),
        }
    }
}
