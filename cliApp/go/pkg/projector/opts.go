package projector

import "github.com/hellflame/argparse"

type Opts struct {
	Args   []string
	Config string
	Pwd    string
}

func GetOptions() (*Opts, error) {
	parser := argparse.NewParser("projector", "gets all the value", &argparse.ParserConfig{
		WithHint: true, DisableDefaultShowHelp: true,
	})
	args := parser.Strings("a", "args", &argparse.Option{
		Positional: true,
		Default:    "",
		Required:   false,
	})
	config := parser.String("c", "config", &argparse.Option{Required: false, Default: ""})
	pwd := parser.String("p", "pwd", &argparse.Option{Required: false, Default: ""})

	err := parser.Parse(nil)
	if err != nil {
		return nil, err
	}

	return &Opts{
		Pwd:    *pwd,
		Config: *config,
		Args:   *args,
	}, nil

}
