packag main

import "fmt"

type Fib struct {
	a, b int
}

func (f *Fib) Next() int {
	a, b := f.a, f.b
	f.a = b
	f.b = b + a
	return a
}

func main() {
	fib := Fib{0, 1}
	for i := 0; i < 10; i++ {
		fmt.Println(fib.Next())
	}
}
