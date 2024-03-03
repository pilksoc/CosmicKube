extends CharacterBody2D


@export var tile_size: float = 100
@export var is_moving: bool = false

var inputs = {
	"right": Vector2.RIGHT,
	"left": Vector2.LEFT,
	"up": Vector2.UP,
	"down": Vector2.DOWN
}

var inputs_rev = {
	Vector2.RIGHT:"right",
	Vector2.LEFT: "left",
	Vector2.UP: "up",
	Vector2.DOWN: "down"
}

func _ready():
	position = position.snapped(Vector2.ONE * tile_size)
	position += Vector2.ONE * tile_size / 2

func _input(event):
	var vec = Input.get_vector("left", "right", "up", "down")
	if vec.length() == 1:
		position += vec*100
		$AnimationPlayer.play(inputs_rev[vec])
	
