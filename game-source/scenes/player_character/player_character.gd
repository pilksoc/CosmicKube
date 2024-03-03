extends CharacterBody2D


@export var tile_size: float = 100
@export var is_moving: bool = false
@export var animation_speed: float = 4

var inputs = {
	"right": Vector2.RIGHT,
	"left": Vector2.LEFT,
	"up": Vector2.UP,
	"down": Vector2.DOWN
}


func _ready():
	position = position.snapped(Vector2.ONE * tile_size)
	position += Vector2.ONE * tile_size / 2


func _input(event):
	if is_moving:
		pass
	for dir in inputs.keys():
		if event.is_action_pressed(dir):
			var tween = get_tree().create_tween()
			tween.tween_property(self, "position", position + (inputs[dir] * tile_size), 1.0/animation_speed).set_trans(Tween.TRANS_SINE)
			is_moving = true
			await tween.finished
			is_moving = false
	
