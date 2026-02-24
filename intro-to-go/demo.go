package main

import (
	"fmt"
	"math"
)

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
	var num float64 = 7.8235
	fmt.Printf("%10.2f%%",num)

	str := fmt.Sprintf("%10.2f%%",num) // Stores a String

	fmt.Println(str)

	/*
	+ = addition
	- = subtraction 
	* = multiply
	/ = divide 
	% = modulus 
	-- = decrement
	++ = increment
	*/
	// Operations on different types
	a := uint8(7)
	b := 80000
	// Just take smaller val and cast that to larger vals type
	sum := int(a) + b
	// Note: whenever doing arithmetic we get the type of both inputs as the output. 
	fmt.Println(sum) 

	//to use exponents you have to use 
	eight := math.Pow(2,3)
	fmt.Println(eight)
	//num_str := "1234"
	//actual_num, err := strconv.Atoi(num_str) //Catching an error if there is one in err
	//actual_num, err := strconv.ParseInt(x, 10, 32) //ParseInt(num, the base the number is in, the bits needed)
	
	// Conditionals
	/*
	<
	>
	<=
	≥
	==
	!=
	*/
	xa := uint8(8)
	ya := 10
	za := xa < uint8(ya) // Comparisons need to be the same type
	fmt.Println(za)

	// Switch cases
	j := 1
	switch j {
	case 1: 
		fmt.Println(j)
	default:
		fmt.Println("default")
	}

	switch j {
	case 1:
		fmt.Println(j)
		fallthrough
	case 2:
		fmt.Println(j-1)
		fallthrough
	default:
		fmt.Println("default")
	}

	for idx := 0; idx < 10; idx++{
		fmt.Println(idx)
	}

	// There are no while loops in Go, so you have to implement them using for loops
	
	ab := -1
	for true{
		ab++
		if a == 10{
			break 
		}
	}

	// Accessing strings by indexes does not work how you might think 
	// It acesses slices of bytes
	
	// So to access the char at an index you have to do something like

	st := "Hello"
	char := string(str[0])
	println(st, char);
	//Don't use this, doesn't work properly 
	for in := 0; in < len(st); in++{
		fmt.Printf("%c", in)
	}

	// When using non ascii chars you have to use the range keyword when looping over a string

	for _, ch := range st{ // The first var is the index, the other is the value / char @ that index
		fmt.Printf("%c", ch)
	}

	var arr[2]int //An array of size 2 that stores ints
	var arr2 = [2]int{1,3} // Array literal 
	var arr3 = [...][3]int {{1,2,3},{4,5,6},{7,8,9}}

	// Slices
	var arr4 = [5]int{1,2,3,4,5}
	var sl = arr4[1:3] // This creates a slice of the entire array. 
	sl[2] = 7
	sl = sl[:4] // You are allowed to take a slice of a slice and increase the capacity 
	fmt.Println(arr4, sl, cap(sl))
	fmt.Println(arr,arr2,arr3)
	// For slices[i:n]
	//pointer -> arr[i]
	//length -> n - i
	//capacity -> elements I could increase the slice to 

	// Using slices to increase size

	sl1 := []string{"hello", "world"}
	for ctr:=0;ctr<10;ctr++{
		sl1 = append(sl1, "hola") //We don't have enough space to add the element to the slice, so we double the size of the underlying array, and append the new option. 
		fmt.Println(sl1, len(sl1), cap(sl1))
	}
	
	sl2 := make([]int, 10, 20)//Made an int arr slice with size 10 and capacity 20

	fmt.Println(sl2)

	//Iterating over Slices
	sl3 := []string{"hello", "world", "hi"}
	for i, value := range sl3{
		fmt.Println(i, value)
	}

	//If we pass a slice into a function unlike an array we are not using a copy and it actually mutates that value. This is because a slice has a pointer to the underlying array. 

	test(sl3)

	// Maps

	// var syntax
	var mp map[string]int = map[string]int{"a": 1}  
	// or 
	m := map[int]int{1:2, 2:3}
	mp2 := map[string][]int{"a" : {1,2,3}}
	delete(mp, "a")
	//value, ok := mp["a"]

	callFunc(doubleNumber)

	// Or you can use anonymous functions

	callFunc(func(x int) int{
		return x*3
	})

	sum1 := sum_1([]int{1,2,3,4,5}...)
	sum2 := sum_1(1,2,3,4,5)
	fmt.Println(s)
		
	p1 := Person{"Adam", 20} //You can either put elements in the correct order of the fields or you can name the fields explicitly

	// var syntax 
	var p3 Person = Person{
		"adam", 
		20,
	}
	p2 := Person{
		age : 20,
		name : "Adam",
	}
	fmt.Println(p1)

}

// Srtucts 

type Person struct {
	name string 
	age uint 
}

func test(arr []string){
	arr[0] = "change this"
}

// Functions

func add(num1 int, num2 int) int{ // a private function that takes in two ints, and returns an int
	return num1 + num2
}

// You can return multiple things
func divide(num1 int, num2 int) (float64, string){
	if num2 == 0{
		fmt.Println("Cannot divide by zero")
		return -1.0, "err"
	} 
	return float64(num1)/float64(num2), "success"
}

func callFunc(callable func(int) int) int{
	return callable(10)
}

func doubleNumber(num int) int{
	return 2*num
}

func getFunc(str string) func(string) string{
	return func(str2 string) string{
		return str + str2
	}
}

//Variatic Function: A function that takes in multipe arguments, This is read as a slice of ints
func sum_1(nums ...int) int{
	var sum int 
	for _, val := range nums{
		sum += val
	}
	return sum
}

func getName(p Person) string{
	return p.name
}

// To create a function that works the same for every struct we create we can declare a method

func (p Person) getName() string{
	return p.name
}
// Now this function works across all person structs
