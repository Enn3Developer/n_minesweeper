extends Control

@onready var state_label := $VBoxContainer/State

func _ready() -> void:
	var state_text = "You " + ("won" if GameSettings.win else "lost") + " in " + String.num(float(GameSettings.timer) / 1000.0, 1) + "s!"
	state_label.text = state_text

func _on_play_again_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game_2d.tscn")

func _on_return_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/menu.tscn")
