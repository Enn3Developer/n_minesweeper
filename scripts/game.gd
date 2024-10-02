extends Node3D

@onready var time_label := $Camera3D/Stats/Time
@onready var flags_label := $Camera3D/Stats/Flags

@export var cell_mesh: Mesh
@export var camera: Camera3D
@export var scene_path: String
@export var current_scene_path: String

signal processed

var grid: PackedByteArray
var showed_grid: PackedByteArray
var flagged_grid: PackedByteArray
var flagged := 0
var multimesh: RID
var instance: RID
var height: int
var width: int
var bombs: int
var generated := false
var start := 0
var losing := false

func _ready() -> void:
	GameSettings.emulate_mouse = false
	height = GameSettings.height
	width = GameSettings.width
	bombs = GameSettings.bombs
	flags_label.text = str(bombs) + " Flags"
	multimesh = RenderingServer.multimesh_create()
	RenderingServer.multimesh_set_mesh(multimesh, cell_mesh.get_rid())
	RenderingServer.multimesh_allocate_data(multimesh, width * height,
		RenderingServer.MULTIMESH_TRANSFORM_3D, false, true)
	for z in range(height):
		for x in range(width):
			var index := z * width + x
			var transform = Transform3D(Basis.IDENTITY, Vector3(x + 0.5, 0.0, z + 0.5))
			RenderingServer.multimesh_instance_set_transform(multimesh, index, transform)
			RenderingServer.multimesh_instance_set_custom_data(multimesh, index, Color(0.0, 0.0, 0.0, 0.0))
	instance = RenderingServer.instance_create()
	RenderingServer.instance_set_base(instance, multimesh)
	RenderingServer.instance_set_scenario(instance, get_world_3d().scenario)
	camera.position = Vector3(width / 2.0, 15.0, height / 2.0)
	var area := Area3D.new()
	var collision := CollisionShape3D.new()
	var shape := BoxShape3D.new()
	shape.size = Vector3(width, 0.0, height)
	collision.shape = shape
	collision.position = Vector3(width / 2.0, 0.0, height / 2.0)
	area.add_child(collision)
	area.connect("input_event", _area_on_input_event)
	add_child(area)

func _exit_tree() -> void:
	RenderingServer.free_rid(instance)
	RenderingServer.free_rid(multimesh)
	GameSettings.emulate_mouse = true

func _process(delta: float) -> void:
	emit_signal("processed")
	if losing: return
	if generated: 
		check_win()
		var time := (Time.get_ticks_msec() - start) / 1000.0
		var minutes: int = floorf(time / 60.0)
		var seconds := int(floorf(time)) % 60
		time_label.text = "%d:%02d" % [minutes, seconds]
	if Input.is_action_just_pressed("reset"):
		reset()

func _area_on_input_event(camera: Node, event: InputEvent, event_position: Vector3, normal: Vector3, shape_idx: int):
	if losing: return
	if event is InputEventMouseButton:
		if event.pressed:
			var position := Vector2i(floorf(event_position.x), floorf(event_position.z))
			if event.button_index == MOUSE_BUTTON_LEFT:
				click_show(position)
			elif event.button_index == MOUSE_BUTTON_RIGHT:
				click_flag(position)

func _input(event: InputEvent) -> void:
	if event is InputEventSingleScreenTap:
		Logger.info("received tap input")
		var touch_event: InputEventSingleScreenTap = event
		var mouse_event := InputEventMouseButton.new()
		mouse_event.pressed = true
		mouse_event.button_index = MOUSE_BUTTON_LEFT
		mouse_event.position = touch_event.position
		get_viewport().push_input(mouse_event, true)
		var new_event := mouse_event.duplicate(true)
		new_event.pressed = false
		get_viewport().push_input(new_event, true)
	elif event is InputEventSingleScreenLongPress:
		Logger.info("received long press input")
		var touch_event: InputEventSingleScreenLongPress = event
		var mouse_event := InputEventMouseButton.new()
		mouse_event.pressed = true
		mouse_event.button_index = MOUSE_BUTTON_RIGHT
		mouse_event.position = touch_event.position
		get_viewport().push_input(mouse_event, true)
		var new_event := mouse_event.duplicate(true)
		new_event.pressed = false
		get_viewport().push_input(new_event, true)

func click_show(position: Vector2i):
	Logger.info("received show input: " + str(position))
	var index := position.y * width + position.x
	if not generated:
		generate_grid(position)
		$Camera3D/StartGame.visible = false
		$AnimationPlayer.stop()
	if flagged_grid.decode_s8(index) == 1: return
	if grid.decode_s8(index) == -1:
		Logger.info("clicked on bomb")
		prepare_lose()
		if GameSettings.vibration: Input.vibrate_handheld(200, 0.7)
		return
	show_cell_and_neighbours(position)
	if GameSettings.vibration: Input.vibrate_handheld(500, 1.0)

func click_flag(position: Vector2i):
	Logger.info("received flag input: " + str(position))
	if not generated: return
	var index := position.y * width + position.x
	if showed_grid.decode_s8(index) == 1: return
	flag_cell(position)
	if GameSettings.vibration: Input.vibrate_handheld(500, 1.0)

func generate_grid(click_position: Vector2i):
	Logger.info("generating grid")
	grid = PackedByteArray()
	showed_grid = PackedByteArray()
	flagged_grid = PackedByteArray()
	grid.resize(width * height)
	showed_grid.resize(width * height)
	flagged_grid.resize(width * height)
	grid.fill(0)
	showed_grid.fill(0)
	flagged_grid.fill(0)
	var rng := RandomNumberGenerator.new()
	while bombs > 0:
		var x := rng.randi_range(0, width - 1)
		var y := rng.randi_range(0, height - 1)
		var index := y * width + x
		if (x == click_position.x and y == click_position.y) or grid.decode_s8(index) == -1: continue
		grid.encode_s8(index, -1)
		for x_offset in range(-1, 2):
			if x == 0 and x_offset == -1: continue
			if x == width - 1 and x_offset == 1: continue
			for y_offset in range(-1, 2):
				if y == 0 and y_offset == -1: continue
				if y == height - 1 and y_offset == 1: continue
				if x_offset == 0 and y_offset == 0: continue
				var index_offset := (y + y_offset) * width + (x + x_offset)
				if grid.decode_s8(index_offset) == -1: continue
				grid.encode_s8(index_offset, grid.decode_s8(index_offset) + 1)
		bombs -= 1
	generated = true
	start = Time.get_ticks_msec()
	Logger.info("grid generated")

func prepare_lose():
	losing = true
	var end := Time.get_ticks_msec()
	for x in range(width):
		for y in range(height):
			var index := y * width + x
			var value := grid.decode_s8(index)
			if value != -1: continue
			show_cell(Vector2i(x, y))
	await get_tree().create_timer(2.0).timeout
	end_game(false, end)

func show_cell_and_neighbours(cell_position: Vector2i):
	var positions: Array[Vector2i] = []
	positions.append(cell_position)
	var cells_showed := 0
	while positions.size() > 0:
		cells_showed += 1
		if cells_showed >= GameSettings.speed:
			cells_showed = 0
			await processed
		var position: Vector2i = positions.pop_front()
		var index := position.y * width + position.x
		if showed_grid.decode_s8(index) == 1: continue
		if flagged_grid.decode_s8(index) == 1: continue
		if grid.decode_s8(index) == -1: continue
		show_cell(position)
		if grid.decode_s8(index) != 0: continue
		for x_offset in range(-1, 2):
			if position.x == 0 and x_offset == -1: continue
			if position.x == width - 1 and x_offset == 1: continue
			for y_offset in range(-1, 2):
				if position.y == 0 and y_offset == -1: continue
				if position.y == height - 1 and y_offset == 1: continue
				if x_offset == 0 and y_offset == 0: continue
				var index_offset := (position.y + y_offset) * width + (position.x + x_offset)
				if grid.decode_s8(index_offset) == -1: continue
				if showed_grid.decode_s8(index_offset) == 1: continue
				positions.append(Vector2i(position.x + x_offset, position.y + y_offset))

func show_cell(cell_position: Vector2i):
	var index := cell_position.y * width + cell_position.x
	var value := grid.decode_s8(index)
	var cell: float
	if value == 0:
		cell = 1.0
	elif value == -1:
		cell = 2.0
	else:
		cell = 3.0 + value
	RenderingServer.multimesh_instance_set_custom_data(multimesh, index, Color(0.0, cell, 0.0, 0.0))
	showed_grid.encode_s8(index, 1)

func flag_cell(cell_position: Vector2i):
	var index := cell_position.y * width + cell_position.x
	if showed_grid.decode_s8(index) == 1: return
	var value := flagged_grid.decode_s8(index)
	var cell: float
	if value == 0:
		if flagged == GameSettings.bombs: return
		value = 1
		flagged += 1
		cell = 3.0
	else:
		value = 0
		flagged -= 1
		cell = 0.0
	flagged_grid.encode_s8(index, value)
	flags_label.text = str(GameSettings.bombs - flagged) + " Flags"
	RenderingServer.multimesh_instance_set_custom_data(multimesh, index, Color(0.0, cell, 0.0, 0.0))

func check_win():
	if flagged != GameSettings.bombs: return
	for x in range(width):
		for y in range(height):
			var index := y * width + x
			var value := grid.decode_s8(index)
			if value != -1 and showed_grid.decode_s8(index) == 0: return
			if value == -1 and flagged_grid.decode_s8(index) == 0: return
	end_game(true)

func end_game(win: bool, end: int = 0):
	if end == 0: end = Time.get_ticks_msec()
	GameSettings.win = win
	GameSettings.timer = end - start
	get_tree().change_scene_to_file(scene_path)

func reset():
	get_tree().change_scene_to_file(current_scene_path)

func _on_reset_pressed() -> void:
	reset()
