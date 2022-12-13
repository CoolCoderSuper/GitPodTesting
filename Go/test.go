package main

import (
	"bufio"
	"fmt"
	"os"
)
func main(){
    println("Enter name: ")
    name := getUserInput()
	println(Hello(name))
}

func Hello(name string) string {
    return fmt.Sprintf("Hello, %v.", name)
}

func getUserInput() string {
    scanner := bufio.NewScanner(os.Stdin)
    input := ""
    for scanner.Scan() {
        curr := scanner.Text()
        if curr[0] == '\n' {
            break
        }else {
            input += curr
        }
    }
    return input
}