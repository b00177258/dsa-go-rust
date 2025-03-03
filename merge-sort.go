package main

import "fmt"

func merge(left []int, right []int) []int {
	result := []int{}
	i, j := 0, 0
	for i < len(left) && j < len(right) {

		if left[i] < right[j] {

			result = append(result, left[i])
			i++

		} else {
			result = append(result, right[j])
			j++
		}
	}

	for i < len(left) {

		result = append(result, left[i])
		i++
	}
	for j < len(right) {

		result = append(result, right[j])
		j++

	}

	return result

}

func mergesort(arr []int) []int {

	if len(arr) <= 1 {

		return arr

	}

	mid := len(arr) / 2

	left := mergesort(arr[:mid])

	right := mergesort(arr[mid:])

	return merge(left, right)

}

func main() {
	arr := []int{6, 3, 8, 5, 2, 7, 4, 1}
	fmt.Println("Original array:", arr)
	sortedArr := mergesort(arr)
	fmt.Println("Sorted array:", sortedArr)

}
