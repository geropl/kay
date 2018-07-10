extern crate kay;
extern crate kay_simple_example_common;

use kay::{ActorSystem, Networking};
use kay_simple_example_common::counter;

fn main() {
    println!("Creating actor system...");
    let mut system = ActorSystem::new(Networking::new(1, vec!["wsclient", "localhost:9999"]));
    counter::setup(&mut system);

    println!("Connecting to network...");
    system.networking_connect();

    let mut world = system.world();

    let counter_id = counter::CounterID::spawn(13, &mut world);
    counter::ServerLoggerID::spawn(counter_id, &mut world);

    system.process_all_messages();

    loop {
        system.networking_send_and_receive();

        system.process_all_messages();

        if system.shutting_down || system.panic_happened {
            break;
        }

        system.networking_finish_turn();
    }
}
