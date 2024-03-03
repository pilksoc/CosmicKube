extends Node2D
@export var on = true
@export var tile_size = 100 

func _process(delta):
	queue_redraw()

func _draw():
	if !on:
		pass
	
	var camera: CharacterBody2D = get_tree().current_scene.find_child('PlayerCharacter')

	var size = get_viewport_rect().size / 2
	var cam = camera.position
	
	for i in range(int((cam.x - size.x) / tile_size) - 1, int((size.x + cam.x) / tile_size) + 1):
		draw_line(Vector2(i * tile_size, cam.y + size.y + 100), Vector2(i * tile_size, cam.y - size.y - 100), "000000")
	for i in range(int((cam.y - size.y) / tile_size) - 1, int((size.y + cam.y) / tile_size) + 1):
		draw_line(Vector2(cam.x + size.x + 100, i * tile_size), Vector2(cam.x - size.x - 100, i * tile_size), "000000")

