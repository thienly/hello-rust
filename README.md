# hello-rust
`hello-rust` is simple application with warp for webapi and libp2p for peer to peer communication.It's really funny to leaning something new. :)   
## Describe
The application is contains two main thread. 
- Web Server.
- Peer to Peer listener.  
They communicate through `mpsc` tokio module.
The data are stored in memory as a `Arc<RwLock<HashMap<String,Pokemon>`
##How to run
- Checkout source code.
- Install rust if you don't have it. rust will include `cargo` which is used to run the application.
- Run application through terminal.  
 `export PORT={port_number}; cargo run`; `the port_number` is web server port that serves incoming request.
- Run it to others terminal.  
 
##Example
- Open two terminals for running the application.
- `export PORT=3030; cargo run` -> for 1st terminal.  
- `export PORT=3031; cargo run` -> for 2nd terminal.
- Open another terminal for testing.
- Submit data for 1st terminal via curl. ` 
curl --location --request POST 'http://localhost:3030/pokemons' \
--header 'Content-Type: application/json' \
--data-raw '{
  "name":"pokemon3",
  "color": "Blue",
  "eye_num": 5,
  "nose_num": 3,
  "mouth_num": 3
}'
`   
- Checking data.  
`curl --location --request GET 'http://localhost:3031/pokemons'` 
The list of pokemons is returned via.