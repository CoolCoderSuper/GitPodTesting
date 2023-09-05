package main

import (
	"container/list"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	items, err := os.ReadFile("items.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	data := list.New()
	current := 0
	elf := 0
	for _, line := range strings.Split(string(items), "\n") {
		if strings.TrimSpace(line) == "" {
			elf++
			data.PushFront(item{number: elf, calories: current})
			current = 0
		}else{
			cal, pErr := strconv.Atoi(line)
			if pErr != nil {
				fmt.Println("Error parsing line:", pErr)
				continue
			}
			current += cal
		}
	}
	for e := data.Front(); e != nil; e = e.Next() {
		item := e.Value.(item)
		fmt.Printf("Elf: %v, Calories: %v\n", item.number, item.calories)
	}
}