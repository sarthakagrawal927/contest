// all declarations with capital case are exported

package main

import "fmt"

type GoEnum = int
type FooStruct struct{}

const (
	Foo GoEnum = iota
	Bar
	Blast
)

func (f *FooStruct) thisOnError(code int) error {
	return fmt.Errorf("error %v", code)
}

func returnsError(code int) error {
	return fmt.Errorf("error %v", code)
}

func CreateFoo(fail bool) (*FooStruct, error) {
	if fail {
		return nil, fmt.Errorf("error %v", 5)
	}
	return &FooStruct{}, nil
}

func main() {
	err := returnsError(5)
	if err != nil {
		return
	}
	foo := FooStruct{}

	foo.thisOnError(5)
}
