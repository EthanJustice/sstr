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

func main() {
	var config Config
	f, err := ioutil.ReadFile("sstr.json")
	if err != nil {
		panic("No valid sstr config found in the current working directory.")
	}

	json := json.Unmarshal(f, &config)

	if json != nil {
		panic("failed to parse configuration file!")
	}

	t := flag.String("cmd", "", "command to run")

	flag.Parse()

	if x, found := config.Commands[*t]; found {
		fields := strings.Fields(x)

		fmt.Printf("\nrunning command %s with args %s\n", fields[0], fields[1:])

		exe, err := exec.LookPath(fields[0])
		if err != nil {
			fmt.Println(exe, err)
			panic("couldn't find executable with that name!")
		}
		args := fields[0:]

		cmd := exec.Command(fields[0], args...)
		cmd.Path = exe

		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Stderr = os.Stderr

		res := cmd.Run()
		if res != nil {
			fmt.Printf("\ncommand %s successfully ran", *t)
		} else {
			fmt.Printf("\ncommand failed to run: %s", res.Error())
		}
	} else if found == false {
		panic("command not found!")
	}
}
