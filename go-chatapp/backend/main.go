package main

import (
	"fmt"
	"go-chatapp/websocket"
	"log"
	"net/http"
)

func main(){
	setupRoutes()
	http.ListenAndServe(":8080", nil)
}

// Define Websocket endpoint
func serveWs(w http.ResponseWriter, r *http.Request){
	fmt.Println(r.Host)

	// Now upgrade the connection to websocket 
	ws, err := websocket.Upgrade(w, r)
	if err != nil{
		log.Println(err)
		return
	}
	go websocket.Writer(ws)
	websocket.Reader(ws)
}

func setupRoutes(){
	fmt.Println("Chat App v0.01")
	http.HandleFunc("/", testServer)
	http.HandleFunc("/ws", serveWs)
}

func testServer(w http.ResponseWriter, r *http.Request){
	fmt.Fprint(w, "Simple Server Setup")	
}
