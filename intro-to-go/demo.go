package main

import "fmt"

func main(){
	fmt.Println("Hello, World!")	
	//Data types

	//uint32 - unsigned int
	//int32 - integer
	//float64 - float
	//byte = int8 - can be a char ex. 'a', or a 32 bit number
	//rune = int32 
	//bool - boolean
	//string - String (double quotation marks only: "hello")
	//nil = null

	var x string = "hello, world"
	// var default_string string //Equal to the default value for a string wich is ""

	//Implicit assignment operator
	y := 3 
	fmt.Println(y)
	fmt.Println(x)
	fmt.Printf("%T", y) // prints out the type of y 

	//Type casting 
	z := uint32(0)
	fmt.Println("", z)
	
	//Things that don't works because it's unsigned vs signed
	l := -10
	s := uint64(l)
	fmt.Println(l ,s)
}
