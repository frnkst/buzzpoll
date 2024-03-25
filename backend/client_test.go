package main

import (
	"testing"
)

func TestBla(t *testing.T) {
	name := "Gladys"
	want := "Gladys"
	if want != name {
		t.Fatalf(`Strings are not the same`)
	}
}
