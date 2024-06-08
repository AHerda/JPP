package main

import (
	"sync"
)

const (
	randSecond       = 1e9
	filozofowieIlosc = 5
	doZjedzenia      = 3
)

type widelec struct {
	sync.Mutex
}

type kolejka chan struct{}

func newKolejka(n int) *kolejka {
	kol := make(kolejka, n)
	for i := 0; i < n; i++ {
		kol <- struct{}{}
	}
	return &kol
}

func (k *kolejka) wez() {
	<-*k
}

func (k *kolejka) odloz() {
	*k <- struct{}{}
}

func main() {
	widelce := make([]*widelec, filozofowieIlosc)
	for i := 0; i < filozofowieIlosc; i++ {
		widelce[i] = new(widelec)
	}

	kolej := newKolejka(filozofowieIlosc - 1)
	filozofowie := make([]filozof, filozofowieIlosc)
	for i := 0; i < filozofowieIlosc; i++ {
		filozofowie[i] = newFilozof(i, widelce[i], widelce[(i+1)%filozofowieIlosc])
	}

	var wg sync.WaitGroup
	for _, f := range filozofowie {
		wg.Add(1)
		go f.jedzenie(&wg, kolej)
	}

	wg.Wait()
}
