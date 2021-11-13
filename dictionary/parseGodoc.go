package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

// ParseHTMLPage will take as input an HTML page as a string.
// and returns every Go function with its documentation as a map.
func ParseHTMLPage(page string) map[string]string {
	// First, split into a slice of lines
	lines := strings.Split(page, "\n")
	for _, l := range lines {
		processLine(l)
	}
}

var m = map[string]string{
	"ParseHTMLPage": "Insert doc of this function here.",
	"OtherFunction": "Different doc here.",
}

func processLine(s string) {
	// Check if line is a function declaration.
	// If so, put select it, and the comment above it.
}

func parseComment(s string) string {}
