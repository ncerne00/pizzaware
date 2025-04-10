# pizzaware 🍕
- Malware intended to coerce victims into buying you a pizza.
- FOR LEGAL REASONS I DO NOT CONDONE THE SPREAD OF THIS MALWARE, THIS IS JUST FOR FUN

## Features
- Installs itself as a startup app in the windows registry to launch on reboot
- Kills other processes which could terminate the malware, like task manager
- Displays popups to annoy the user
- Changes the desktop background to a chef that gets increasingly more deep fried as time goes on
- Plays annoying music with increasing speed and crescendo
- Websockets-based kill switch when pizza is delivered and custom messages to provoke the victim with

## Usage
- First, set up the websockets server which is used to send messages to the malware. 
```sh
python3 websockets_server/ws_server.py
```
- Ensure the malware is pointing to your websockets server before you build or run it.
```sh
cargo build --release
```
or 
```sh
cargo run
```
- To kill the malware, simply issue a "stop" message via the websockets server:
```sh
stop
You entered: stop
Broadcasting 'stop' to 1 clients
```