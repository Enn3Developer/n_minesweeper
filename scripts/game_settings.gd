extends Node

var height := 20:
	set(value):
		height = max(2, min(value, 200))
	get:
		return height
var width := 20:
	set(value):
		width = max(2, min(value, 200))
	get:
		return width
var bombs := 40:
	set(value):
		bombs = max(1, min(value, width*height - 1))
	get:
		return bombs

var timer := 0
var win := false
