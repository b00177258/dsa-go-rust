package main

import "fmt"

func main() {
	arr := []int{2, 1, 4, 23, 22, 12}
	for i := 0; i < len(arr)-1; i++ {

		for j := i; j < len(arr)-1; j++ {
			if arr[j] > arr[j+1] {
				arr[j+1], arr[j] = arr[j], arr[j+1]
			}

		}

	}
	fmt.Println(arr)
}
