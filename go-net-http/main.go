package main

import (
	"encoding/json"
	"fmt"
	"strconv"
	"net/http"
	"sync"
)

type User struct {
	Name string `json: "name"`

}

var userCache = make(map[int]User)

var cacheMutex sync.RWMutex

func main(){
	mux := http.NewServeMux()
	mux.HandleFunc("/", handleRoot)
	
	mux.HandleFunc("POST /users", createUser)

	mux.HandleFunc("GET /users/{id}", getUser)

	mux.HandleFunc("DELETE /users/{id}", deleteUser)
	fmt.Println("Server listening to :8080")
	http.ListenAndServe(":8080", mux)
}

func handleRoot(w http.ResponseWriter, r *http.Request){
	fmt.Fprintf(w, "Hellow world")

}

func createUser(w http.ResponseWriter, r *http.Request){
	var user User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil{
		http.Error(w, err.Error(), http.StatusBadGateway)
		return 
	}

	if user.Name == ""{
		http.Error(w, "name is required", http.StatusBadRequest)
		return 
	}
	
	cacheMutex.Lock()
	userCache[len(userCache)+1] = user
	cacheMutex.Unlock()

	w.WriteHeader(http.StatusNoContent)

}

func getUser(w http.ResponseWriter, r *http.Request){
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil{
		http.Error(w, err.Error(), http.StatusBadRequest)
		return 
	}
	cacheMutex.RLock()
	user, ok := userCache[id]
	cacheMutex.RUnlock()

	if !ok{
		http.Error(w, "User not found", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	j, err := json.Marshal(user)
	if err != nil{
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write(j)
	
}

func deleteUser(w http.ResponseWriter, r *http.Request){
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil{
		http.Error(w, err.Error(), http.StatusBadRequest)
		return 
	}
	if _, ok := userCache[id]; !ok{
		http.Error(w, "user not found", http.StatusInternalServerError)
		return
	}
	
	cacheMutex.Lock()
	delete(userCache, id)
	cacheMutex.Unlock()

	w.WriteHeader(http.StatusNoContent)
}
