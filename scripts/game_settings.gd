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

var emulate_mouse := true

func _input(event: InputEvent) -> void:
	if event is InputEventScreenTouch and emulate_mouse:
		var touch_event: InputEventScreenTouch = event
		var mouse_event := InputEventMouseButton.new()
		mouse_event.position = touch_event.position
		mouse_event.pressed = touch_event.pressed
		mouse_event.button_index = MOUSE_BUTTON_LEFT
		get_viewport().push_input(mouse_event, true)
	elif event is InputEventScreenDrag and emulate_mouse:
		var drag_event: InputEventScreenDrag = event
		var mouse_event := InputEventMouseMotion.new()
		mouse_event.pressure = drag_event.pressure
		mouse_event.relative = drag_event.relative
		mouse_event.pen_inverted = drag_event.pen_inverted
		mouse_event.screen_relative = drag_event.screen_relative
		mouse_event.screen_velocity = drag_event.screen_velocity
		mouse_event.tilt = drag_event.tilt
		mouse_event.velocity = drag_event.velocity
		mouse_event.position = drag_event.position
		mouse_event.global_position = drag_event.position
		mouse_event.window_id = drag_event.window_id
		get_viewport().push_input(mouse_event, true)
