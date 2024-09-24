extends Control

@onready var width_value := $Settings/Width/Value
@onready var height_value := $Settings/Height/Value
@onready var bombs_value := $Settings/Bombs/Value

@onready var bombs_slider := $Settings/Bombs/BombsSlider

func _ready() -> void:
	$TitleBar/Version.text = "v" + ProjectSettings.get("application/config/version")
	$Buttons.visible = true
	$Settings.visible = false

func _on_play_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game.tscn")

func _on_settings_pressed() -> void:
	$Buttons.visible = false
	$Settings.visible = true

func _on_exit_pressed() -> void:
	get_tree().quit()

func _on_back_pressed() -> void:
	$Buttons.visible = true
	$Settings.visible = false

func _on_width_slider_value_changed(value: float) -> void:
	GameSettings.width = int(value)
	width_value.text = str(GameSettings.width)
	bombs_slider.max_value = GameSettings.width * GameSettings.height - 1

func _on_height_slider_value_changed(value: float) -> void:
	GameSettings.height = int(value)
	height_value.text = str(GameSettings.height)
	bombs_slider.max_value = GameSettings.width * GameSettings.height - 1

func _on_bombs_slider_value_changed(value: float) -> void:
	GameSettings.bombs = int(value)
	bombs_value.text = str(GameSettings.bombs)
