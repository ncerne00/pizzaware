#!/usr/bin/env python3
import asyncio
import websockets
import sys

# Storage for active connections
clients = set()

# Message queue
message_queue = asyncio.Queue()

# Handler for new client connections
async def client_handler(websocket):
    # Register new client
    print(f"New client connected!", flush=True)
    clients.add(websocket)
    
    try:
        # Just keep the connection open
        while True:
            try:
                # Wait for a ping or message from client
                await asyncio.wait_for(websocket.recv(), timeout=10)
            except asyncio.TimeoutError:
                # Send a ping to keep connection alive
                await websocket.ping()
            except Exception as e:
                print(f"Client error: {e}", flush=True)
                break
    finally:
        # Unregister on disconnect
        clients.remove(websocket)
        print(f"Client disconnected. {len(clients)} clients remaining.", flush=True)

# Task to get input from console
async def input_handler():
    while True:
        # Get input in a non-blocking way
        message = await asyncio.get_event_loop().run_in_executor(
            None, lambda: input("Enter message: ")
        )
        
        # Debug output
        print(f"You entered: {message}", flush=True)
        
        # Put in queue for broadcasting
        await message_queue.put(message)
        
        if message.lower() == "exit":
            break

# Task to broadcast messages to all clients
async def broadcaster():
    while True:
        # Wait for a message from the input handler
        message = await message_queue.get()
        
        if message.lower() == "exit":
            print("Exiting broadcaster...", flush=True)
            break
            
        if not clients:
            print("No clients connected. Message not sent.", flush=True)
            continue
            
        # Debug output
        print(f"Broadcasting '{message}' to {len(clients)} clients", flush=True)
        
        # Send to all clients
        for client in clients.copy():  # Use copy to allow modification during iteration
            try:
                await client.send(message)
                print(f"Message sent to a client", flush=True)
            except Exception as e:
                print(f"Error sending to client: {e}", flush=True)
                # Client might be disconnected
                clients.discard(client)

async def main():
    # Start the WebSocket server
    server = await websockets.serve(client_handler, "localhost", 8765)
    print("WebSocket server started on ws://localhost:8765", flush=True)
    
    # Start input handler and broadcaster
    input_task = asyncio.create_task(input_handler())
    broadcast_task = asyncio.create_task(broadcaster())
    
    # Run until input handler exits
    await input_task
    
    # Cancel broadcaster
    broadcast_task.cancel()
    
    # Close server
    server.close()
    await server.wait_closed()
    print("Server shut down", flush=True)

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nServer interrupted", flush=True)
    sys.exit(0)