package main

import (
	"cliApp/pkg/projector"
	"cliApp/pkg/utils"
	"fmt"
)

func main() {
	opts, err := projector.GetOptions()
	utils.HandleError(err)
	config, err := projector.NewConfig(opts)
	utils.HandleError(err)

	fmt.Printf("opts: %+v", config)
}
