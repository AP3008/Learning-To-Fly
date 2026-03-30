package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
)

func main(){
	r := gin.Default()
	r.GET("/", func(c *gin.Context){
		c.JSON(200, gin.H{
			"message": "Hello from go URL shortner", 
		})
	})

	if err := r.Run(":9808"); err != nil {
		panic(fmt.Sprintf("Failef to start the web server: ", err))
	}
}

