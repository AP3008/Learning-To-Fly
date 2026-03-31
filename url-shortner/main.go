package main

import (
	"fmt"
	"url-shortner/store"
)

func main() {
	store.InitializeStore()
	fmt.Println("URL Shortener started")
}
