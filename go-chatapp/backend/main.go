package main

import (
	"fmt"
	"net/http"
	"log"
	"github.com/gorilla/websocket"
)

func main(){
	setupRoutes()
	http.ListenAndServe(":8080", nil)
}

// Define an upgrader 
var upgrader = websocket.Upgrader{
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
	// We also need to check origin, but right now we allow any connection
	CheckOrigin: func(r *http.Request) bool{ return true },
}

// Setup a reader that will listen for new messages being sent to websocket 
func reader(conn *websocket.Conn){
	for {
		// read message 
		messageType, p, err := conn.ReadMessage()
		if err != nil{
			log.Println(err)
			return 
		}
		fmt.Println(string(p))

		if err := conn.WriteMessage(messageType, p); err != nil{
			log.Println(err)
			return 
		}
	}
}

// Define Websocket endpoint
func serveWs(w http.ResponseWriter, r *http.Request){
	fmt.Println(r.Host)

	// Now upgrade the connection to websocket 
	ws, err := upgrader.Upgrade(w, r, nil)
	if err != nil{
		log.Println(err)
		return
	}
	reader(ws)
}

func setupRoutes(){
	fmt.Println("Chat App v0.01")
	http.HandleFunc("/", testServer)
	http.HandleFunc("/ws", serveWs)
}

func testServer(w http.ResponseWriter, r *http.Request){
	fmt.Fprint(w, "Simple Server Setup")	
}
