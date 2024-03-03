extends CharacterBody2D


@export var tile_size: float = 100
@export var is_moving: bool = false

signal player_did_move(new_pos: Vector2)


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
	if !event.is_action("down") and ! event.is_action("up") and  !event.is_action("left") and ! event.is_action("right"):
		return
	
	var vec = Input.get_vector("left", "right", "up", "down")
	if vec.length() == 1:
		position += vec*100
		emit_signal('player_did_move', position)
		$AnimationPlayer.play(inputs_rev[vec])
	

func make_player_state(make_player_state:int):
	return JSON.stringify({"name":"eeeeeeeeeee"})

func _on_player_did_move(new_pos):
	WebSockets.send(make_player_state(new_pos))
	print(await WebSockets.message_received)
