package main

import (
	"fmt"
	"net/http"
)

func main(){
	mux := http.NewServeMux()
	fmt.Println("Starting app @ lcoalhost:8080")
	mux.HandleFunc("/", root)
	http.ListenAndServe(":8080",mux) 

}

func root(w http.ResponseWriter, r *http.Request){
	fmt.Fprint(w, "testing testing")
}
