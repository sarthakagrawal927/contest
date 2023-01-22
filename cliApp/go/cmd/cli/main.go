package main

import (
	"cliApp/pkg/projector"
	"cliApp/pkg/utils"
	"encoding/json"
	"fmt"
)

func main() {
	opts, err := projector.GetOptions()
	utils.HandleError(err)
	config, err := projector.NewConfig(opts)
	utils.HandleError(err)

	proj := projector.NewProjector(config)
	if config.Operation == projector.Print {
		if len(config.Args) == 0 {
			data := proj.GetValueAll()
			jsonString, err := json.Marshal(data)
			utils.HandleError(err)
			fmt.Printf("%v", string(jsonString)) //jsonString is binary array
		} else if value, ok := proj.GetValue(config.Args[0]); ok {
			fmt.Printf("%v", value)
		}
	}
	if config.Operation == projector.Add {
		proj.SetValue(config.Args[0], config.Args[1])
		proj.Save()
	}
	if config.Operation == projector.Remove {
		proj.RemoveValue(config.Args[0])
		proj.Save()
	}
}
