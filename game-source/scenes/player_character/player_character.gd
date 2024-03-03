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
	var vec = Input.get_vector("left","right","up","down")
	 
	if !is_moving and vec.length()==1:
		position += vec*100
	
