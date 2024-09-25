extends Camera3D

@export var speed := 25.0
@export var touch_speed := 2.5

var zoom := 1.0
var old_zoom := 1.0
var touch_movement := Vector2.ZERO

func _process(delta: float) -> void:
	var movement := Vector2.ZERO
	if Input.is_action_pressed("ui_up"):
		movement.y -= 1.0
	if Input.is_action_pressed("ui_down"):
		movement.y += 1.0
	if Input.is_action_pressed("ui_right"):
		movement.x += 1.0
	if Input.is_action_pressed("ui_left"):
		movement.x -= 1.0
	var total := movement.normalized() * speed * delta * zoom
	var touch_total := touch_movement * touch_speed * delta * zoom
	position.x += total.x + touch_total.x
	position.z += total.y + touch_total.y
	touch_movement = Vector2.ZERO
	
	if zoom != old_zoom:
		old_zoom = zoom
		position.y = 15.0 * zoom
		if position.y < 2.0:
			position.y = 2.0
		elif position.y > 100.0:
			position.y = 100.0

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				zoom /= 1.2
			elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				zoom *= 1.2
	if event is InputEventScreenPinch:
		if event.relative > 0.0:
			zoom /= 1.02
		elif event.relative < 0.0:
			zoom *= 1.02
	if event is InputEventSingleScreenDrag:
		touch_movement += event.relative * -1
