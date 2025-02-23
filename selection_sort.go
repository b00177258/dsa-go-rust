package main

import "fmt"

func main() {
	arr := []int{11, 2, 12, 1, 23, 4, 24, 5}
	for i := 0; i < len(arr)-1; i++ {
		min_index := i
		for j := i + 1; j < len(arr); j++ {

			if arr[min_index] > arr[j] {
				min_index = j

			}

		}
		arr[min_index], arr[i] = arr[i], arr[min_index]
	}
	fmt.Println(arr)
}
