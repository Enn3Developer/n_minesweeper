extends Camera3D

@export var speed := 25.0

var zoom := 1.0
var old_zoom := 1.0

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
	position.x += total.x
	position.z += total.y
	
	if zoom != old_zoom:
		old_zoom = zoom
		position.y = 15.0 * zoom

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				zoom /= 1.2
			elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				zoom *= 1.2
