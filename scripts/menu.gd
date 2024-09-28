extends Control

@onready var width_value := $Settings/Width/Value
@onready var height_value := $Settings/Height/Value
@onready var bombs_value := $Settings/Bombs/Value

@onready var height_slider := $Settings/Height/HeightSlider
@onready var width_slider := $Settings/Width/WidthSlider
@onready var bombs_slider := $Settings/Bombs/BombsSlider

func _ready() -> void:
	$TitleBar/Version.text = "v" + ProjectSettings.get("application/config/version")
	$Buttons.visible = true
	$Settings.visible = false
	if OS.has_feature("web") or OS.has_feature("mobile"):
		$Buttons/Exit.visible = false
	height_slider.value = GameSettings.height
	width_slider.value = GameSettings.width
	bombs_slider.value = GameSettings.bombs

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

func _on_easy_pressed() -> void:
	height_slider.value = 10
	width_slider.value = 10
	bombs_slider.value = 8
	height_slider.editable = false
	width_slider.editable = false
	bombs_slider.editable = false

func _on_normal_pressed() -> void:
	height_slider.value = 20
	width_slider.value = 20
	bombs_slider.value = 40
	height_slider.editable = false
	width_slider.editable = false
	bombs_slider.editable = false

func _on_hard_pressed() -> void:
	height_slider.value = 30
	width_slider.value = 30
	bombs_slider.value = 120
	height_slider.editable = false
	width_slider.editable = false
	bombs_slider.editable = false

func _on_custom_pressed() -> void:
	height_slider.editable = true
	width_slider.editable = true
	bombs_slider.editable = true
