package main

import (
	"fmt"
	"net/http"
)

func main(){
	setupRoutes()
	http.ListenAndServe(":8080", nil)
}

func setupRoutes(){
	http.HandleFunc("/", testServer)
}

func testServer(w http.ResponseWriter, r *http.Request){
	fmt.Fprint(w, "Simple Server Setup")	
}
