package projector_test

import (
	"cliApp/pkg/projector"
	"reflect"
	"testing"
)

func getOpts(args []string) *projector.Opts {
	return &projector.Opts{
		Args:   args,
		Config: "",
		Pwd:    "",
	}
}

func handleError(err error, t *testing.T) {
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}
}

func testConfig(t *testing.T, args []string, expectedArgs []string, operation projector.Operation) {
	opts := getOpts(args)

	config, err := projector.NewConfig(opts)
	handleError(err, t)

	if !reflect.DeepEqual(config.Args, expectedArgs) {
		t.Errorf("expected args to be %+v string but got %+v", expectedArgs, config.Args)
	}

	if config.Operation != operation {
		t.Errorf("operation expected %v but got %v", operation, config.Operation)
	}
}

func TestConfigPrint(t *testing.T) {
	testConfig(t, []string{}, []string{}, projector.Print)
}

func TestConfigPrintKey(t *testing.T) {
	testConfig(t, []string{"Hi"}, []string{"Hi"}, projector.Print)
}

func TestConfigAddKeyValue(t *testing.T) {
	testConfig(t, []string{"add", "foo", "bar"}, []string{"foo", "bar"}, projector.Add)
}

func TestConfigRemoveKey(t *testing.T) {
	testConfig(t, []string{"rm", "foo"}, []string{"foo"}, projector.Remove)
}
