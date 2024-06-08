package main

import (
	"fmt"
	"math/rand"
	"strings"
	"sync"
	"time"
)

type filozof struct {
	id          int
	lewy, prawy *widelec
	doZjedzenia int
}

type kolej chan struct{}

func newFilozof(id int, lewy, prawy *widelec) filozof {
	return filozof{id, lewy, prawy, doZjedzenia}
}

func (f *filozof) wezLewy() {
	f.lewy.Lock()
}

func (f *filozof) odlozLewy() {
	f.lewy.Unlock()
}

func (f *filozof) wezPrawy() {
	f.prawy.Lock()
}

func (f *filozof) odlozPrawy() {
	f.prawy.Unlock()
}

func (f *filozof) jedz() {
	f.doZjedzenia--
}

func (f *filozof) najedzony() bool {
	return f.doZjedzenia == 0
}

func (f *filozof) jedzenie(wg *sync.WaitGroup, kolej *kolejka) {
	defer wg.Done()

	spaces := strings.Repeat(" ", f.id)

	for !f.najedzony() {
		kolej.wez()

		fmt.Printf("%s[%d] mysli\n", spaces, f.id)
		f.mysli()
		f.wezLewy()
		fmt.Printf("%s[%d] bierze lewy widelec\n", spaces, f.id)
		f.wezPrawy()
		fmt.Printf("%s[%d] bierze prawy widelec\n", spaces, f.id)
		fmt.Printf("%s[%d] je (zostało %d obiadów)\n", spaces, f.id, f.doZjedzenia-1)
		f.mysli()
		f.jedz()
		f.odlozLewy()
		fmt.Printf("%s[%d] odklada lewy widelec\n", spaces, f.id)
		f.odlozPrawy()
		fmt.Printf("%s[%d] odklada prawy widelec\n", spaces, f.id)

		kolej.odloz()
	}
}

func (f *filozof) mysli() {
	t := time.Duration(rand.Int63n(randSecond))
	time.Sleep(t)
}
