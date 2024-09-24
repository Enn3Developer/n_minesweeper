extends Control

@onready var state_label := $VBoxContainer/State

func _ready() -> void:
	var state_text = "You " + ("won" if GameSettings.win else "lost") + " in " + String.num(float(GameSettings.timer) / 1000.0, 1) + "s!"
	state_label.text = state_text

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			get_tree().change_scene_to_file("res://scenes/menu.tscn")
	elif event is InputEventScreenTouch:
		if event.pressed:
			get_tree().change_scene_to_file("res://scenes/menu.tscn")
