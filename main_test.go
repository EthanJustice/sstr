package main

import (
	"os"
	"os/exec"
	"testing"
)

func TestNameArgs(t *testing.T) {
	name, args := NameAndFields("sstr -cmd echo")

	if name != "sstr" {
		t.Fatal("binary name not parsed correctly")
	}

	if len(args) != 2 || args[0] != "-cmd" || args[1] != "echo" {
		t.Fatal("args not parsed correctly")
	}
}

func TestRead(t *testing.T) {
	cmd := exec.Command("sstr", "-cmd", "echo")
	cmd.Path = "./sstr.exe"

	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stderr = os.Stderr

	err := cmd.Run()

	if err != nil {
		t.Fatalf("command not executed successfully: %s", err.Error())
	}
}
