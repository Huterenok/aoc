package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

func beatDaCrazyRecordAndMultiply(input string) int64 {
	time, distance := retrieveTimeAndDistance(input)
	exactTime, exactDistance := joinNums(time), joinNums(distance)
	res := int64(0)
	for i := int64(1); i < exactTime; i++ {
		if (exactTime-i)*i > exactDistance {
			res += 1
		}
	}
	return res
}

func beatDaRecordsAndMultiply(input string) int64 {
	time, distance := retrieveTimeAndDistance(input)
	var res int64 = 1

	for i := 0; i < len(time); i++ {
		temp := int64(0)
		for j := 1; j < time[i]; j++ {
			if j*(time[i]-j) > distance[i] {
				temp += 1
			}
		}
		res *= temp
	}

	return res
}

func retrieveTimeAndDistance(input string) ([]int, []int) {
	matcher := regexp.MustCompile(`\d+`)
	matches := matcher.FindAllString(input, -1)

	res := make([]int, 0, len(matches))
	for _, match := range matches {
		num, err := strconv.Atoi(match)
		if err != nil {
			fmt.Println("Error converting string to int:", err)
			continue
		}
		res = append(res, num)
	}

	return res[:4], res[4:]
}

func joinNums(nums []int) int64 {
	var res int64 = 0
	for _, num := range nums {
		digits := int(math.Log10(float64(num))) + 1
		res = res*int64(math.Pow10(digits)) + int64(num)
	}
	return res
}

func main() {
	file, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println("RETARDED!!!")
	}
	input := string(file)

	fmt.Println(beatDaRecordsAndMultiply(input))
	fmt.Println(beatDaCrazyRecordAndMultiply(input))
}
