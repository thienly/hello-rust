use async_std::{io, task};
use libp2p::{
    identity,
    PeerId,
    floodsub,
    Swarm,
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent, MdnsConfig},
    swarm::{NetworkBehaviourEventProcess, SwarmEvent},
    core::Multiaddr,
};
use futures::{future, prelude::*};
use libp2p::swarm::{ExpandedSwarm};
use std::{
    io::BufRead,
    task::{Context, Poll},
    error::Error
};
use libp2p_swarm_derive::NetworkBehaviour;
use crate::models::{Pokemon, Pokemons};
use std::str::FromStr;
use tokio::sync::mpsc::Receiver;

#[derive(NetworkBehaviour)]
struct MyBehavior {
    flood_sub: Floodsub,
    msdn: Mdns,
    #[behaviour(ignore)]
    #[allow(dead_code)]
    dbs: Pokemons
}
impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehavior{
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(message) => {
                let str = String::from_utf8_lossy(&message.data);
                let result = serde_json::from_str::<Pokemon>(&str);
                let data = result.unwrap();
                if  !self.dbs.items.write().contains_key(&data.name){
                    self.dbs.items.write().insert(data.name.clone(), data.clone());
                }
                println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
            }
            FloodsubEvent::Subscribed { .. } => {}
            FloodsubEvent::Unsubscribed { .. } => {}
        }
    }
}
impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer,_) in list {
                    println!("new peer is found {:?}", peer);
                    self.flood_sub.add_node_to_partial_view(peer)
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer,_) in expired_list {
                    println!("new peer is removed {:?}", peer);
                    self.flood_sub.remove_node_from_partial_view(&peer);
                }

            }
        }
    }
}

pub async fn run(rx: &mut Receiver<Pokemon>, dbs: Pokemons) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
    let transport = libp2p::development_transport(local_key).await;
    let floodsub_topic = floodsub::Topic::new("chat");


    let mut swarm = {
        let mdns = task::block_on(Mdns::new(MdnsConfig::default())).unwrap();
        let mut behavior = MyBehavior {
            flood_sub: Floodsub::new(local_peer_id),
            msdn: mdns,
            dbs
        };
        behavior.flood_sub.subscribe(floodsub_topic.clone());
        Swarm::new(transport.unwrap(), behavior, local_peer_id)
    };
    let multiaddr = Multiaddr::from_str("/ip4/0.0.0.0/tcp/0").unwrap();
    let id = swarm.listen_on(multiaddr);
    if id.is_err() {
        println!("Swarm can not started");
    } else {
        println!("Swarm is starting");
    }

   task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => {
                    if let SwarmEvent::NewListenAddr { address, .. } = event {
                        println!("Listening on {:?}", address);
                    }
                }
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => break,
            }
        }
       loop {
           match rx.poll_recv(cx) {
               Poll::Ready(Some(message)) => {
                   println!("{}", message.name);
                   let result = serde_json::to_string(&message.clone());
                   let string = result.unwrap();
                   swarm.behaviour_mut().flood_sub.publish(floodsub_topic.clone(), string.as_bytes())
               }
               Poll::Ready(None) => return Poll::Ready(()),
               Poll::Pending => break,
           }
       }
        Poll::Pending
    }));
}