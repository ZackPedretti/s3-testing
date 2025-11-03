package main

type Song struct {
	Artist    string  `form:"artist" binding:"required"`
	Name      string  `form:"song" binding:"required"`
	Extension *string `form:"ext"`
}
