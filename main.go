package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"strings"
)

type Config struct {
	Commands map[string]string `json:"commands"`
}

func NameAndFields(str string) (string, []string) {
	fields := strings.Fields(str)

	return fields[0], fields[1:]
}

func main() {
	var config Config
	f, err := ioutil.ReadFile("sstr")
	if err != nil {
		fmt.Println("No valid sstr config found in the current working directory.")
		os.Exit(1)
	}

	json := json.Unmarshal(f, &config)

	if json != nil {
		fmt.Println("failed to parse configuration file!")
		os.Exit(1)
	}

	t := flag.String("cmd", "", "command to run")

	flag.Parse()

	if x, found := config.Commands[*t]; found {
		name, args := NameAndFields(x)

		fmt.Printf("\nrunning command %s with args %s\n", name, args)

		exe, err := exec.LookPath(name)
		if err != nil {
			fmt.Println(exe, err)
			fmt.Println("couldn't find executable with that name!")
			os.Exit(1)
		}

		cmd := exec.Command(name, args...)
		cmd.Path = exe

		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Stderr = os.Stderr

		res := cmd.Run()
		if res != nil {
			fmt.Printf("\ncommand failed to run: %s", res.Error())
		} else {
			fmt.Printf("\ncommand %s ran successfully", *t)
		}

		os.Exit(0)
	} else if !found {
		fmt.Println("command not found!")
		os.Exit(1)
	}

	os.Exit(1)
}
