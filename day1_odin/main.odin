package main

import "core:fmt"
import "core:os"
import "core:strings"

starts_with :: proc(line: string, needle: string) -> bool {
  i :: 0
	for j := 0; j < len(needle); j += 1 {
		if i + j >= len(line) {return false}
		if needle[j] != line[i + j] {return false}
	}
	return true
}

get :: proc(line: string, s: int, e: int, inc: int) -> int {
	for i := s; i != e; i += inc {
		if starts_with(line[i:], "one") {return 1}
		if starts_with(line[i:], "two") {return 2}
		if starts_with(line[i:], "three") {return 3}
		if starts_with(line[i:], "four") {return 4}
		if starts_with(line[i:], "five") {return 5}
		if starts_with(line[i:], "six") {return 6}
		if starts_with(line[i:], "seven") {return 7}
		if starts_with(line[i:], "eight") {return 8}
		if starts_with(line[i:], "nine") {return 9}

		if line[i] == '0' {return 0}
		if line[i] == '1' {return 1}
		if line[i] == '2' {return 2}
		if line[i] == '3' {return 3}
		if line[i] == '4' {return 4}
		if line[i] == '5' {return 5}
		if line[i] == '6' {return 6}
		if line[i] == '7' {return 7}
		if line[i] == '8' {return 8}
		if line[i] == '9' {return 9}
	}

	return 0
}

main :: proc() {
	if len(os.args) < 2 {
		fmt.println("Usage: ./main file_path")
		os.exit(1)
	}

	file_path := os.args[1]

	data, ok := os.read_entire_file(file_path, context.allocator)

	if !ok {
		fmt.println("Error while reading file")
		os.exit(1)
	}

	defer delete(data, context.allocator)

	it := string(data)
	total := 0

	for line in strings.split_lines_iterator(&it) {
		D := get(line, 0, len(line), 1)
		U := get(line, len(line) - 1, -1, -1)

		total += 10 * D + U
	}

	fmt.println(total)
}
