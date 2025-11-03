package main

import "fmt"

type Song struct {
	Artist    string  `form:"artist" binding:"required"`
	Name      string  `form:"song" binding:"required"`
	Extension *string `form:"ext"`
}

func (s Song) BuildFileName() string {
	return fmt.Sprintf("%s.%s", s.Name, *s.Extension)
}

func (s Song) BuildKey() string {
	return fmt.Sprintf("%s/%s.%s", s.Artist, s.Name, *s.Extension)
}
