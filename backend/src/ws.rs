use crate::ws::gen_json::create_response;
use cosmic_kube::{modify_gamestate::remove_player, CLIENTS, Client, Coordinate};
use futures::{FutureExt, StreamExt};
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
mod gen_json;

pub async fn client_connection(ws: WebSocket) {
    println!("establishing client connection... {:?}", ws); //debug

    // splitting the WebSocket stream object into separate 'Sink' and 'Stream' objects.
    // This lets us split up the logic of sending and recieving tasks
    // 'Stream' lets us recieve messages from the client
    // 'Sink' letes us establish a connection from the unbounded channel
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    // creates an unbounded channel. It is configured to send messages to the client.
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    // TODO: fix me!
    // https://stackoverflow.com/questions/54503625/why-does-calling-tokiospawn-result-in-the-panic-spawnerror-is-shutdown-tru
    // 'spawns' a new task, that stays alive until the client has disconnected.
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    // creating a new uuid to use as the key in the 'clients' hashmap, and a new instance of a 'client'
    // this might be clapped
    let uuid = Uuid::new_v4().simple().to_string();

    // we randomly generate the initial position of the player.
    // reduced to 20 for debugging purposes, for the live game we should set this back to grid size (2048)
    // To make it explicit to the compiler that `rng` is only used for a short time, put it in a scope.
    let random_initial_pos: Coordinate;
    {
        let mut rng = rand::thread_rng();
        random_initial_pos = [rng.gen_range(0..20), rng.gen_range(0..20)];
    }

    let new_client = Client {
        client_id: uuid.clone(),
        //the client_sender object is stored within this new client instance so that we can send messages to this connected client in other parts of the code
        sender: Some(client_sender),
        last_position: random_initial_pos,
    };

    //obtains a lock on the client list and inserts the new client into the hashmap using the uuid as the key.
    add_player(uuid.clone(), new_client).await;
    // creates a loop that handles incoming messages from the client
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg).await;
    }

    // as the above will keep running as long as the client is active, when we exit the loop we can safely remove this client instance from the hashmap, after we have removed its position from the grid.
    call_remove_player(&uuid).await;
    println!("{uuid} disconnected"); //debug
}

async fn add_player(uuid: String, new_client: Client) {
    CLIENTS.lock().await.insert(uuid, new_client);
}

async fn call_remove_player(uuid: &str) {
    remove_player(CLIENTS.lock().await.get(uuid).unwrap().last_position).await;
    CLIENTS.lock().await.remove(uuid);
}

// ->recieve client game info <- send back client game state
// wwwwwwwwwwwwwwwwwwwww i am so tired
async fn client_msg(client_id: &str, msg: Message) {
    //println!("received message from {}: {:?}", client_id, msg); //debug

    let Ok(message) = msg.to_str() else { return };

    //println!("{message}");

    let locked = CLIENTS.lock().await;
    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                let _ = sender.send(Ok(Message::text(create_response(message, client_id).await)));
            }
        }
        None => {
            eprintln!("Couldn't find game client in client hashmap! Client ID: {client_id}");
            return
        },
    }
}
