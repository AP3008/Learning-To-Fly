package main

import (
	"fmt"
	"go-chatapp/pkg/websocket"
	"log"
	"net/http"
)

func main(){
	setupRoutes()
	http.ListenAndServe(":8080", nil)
}

// Define Websocket endpoint
func serveWs(pool *websocket.Pool, w http.ResponseWriter, r *http.Request){
	fmt.Println("Websocket Endpoint Hit")

	// Now upgrade the connection to websocket 
	conn, err := websocket.Upgrade(w, r)
	if err != nil{
		log.Println(err)
	}
	client := &websocket.Client{
		Conn: conn, 
		Pool: pool,
	}
	pool.Register <- client
	client.Read()
}

func setupRoutes(){
	fmt.Println("Chat App v0.01")
	pool := websocket.NewPool()
	go pool.Start()
	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request){
		serveWs(pool, w, r)
	})
}

