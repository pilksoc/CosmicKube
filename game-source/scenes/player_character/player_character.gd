extends CharacterBody2D


@export var tile_size: float = 100
@export var is_moving: bool = false
@export var animation_speed: float = 4

func _process(delta):
	queue_redraw()

func _ready():
	position = position.snapped(Vector2.ONE * tile_size)
	position += Vector2.ONE * tile_size / 2


func _input(event):
	if is_moving:
		pass
	var tween = get_tree().create_tween() 
	
	var input_vec =  Vector2(
		event.get_action_strength("right") - event. get_action_strength("left"),
		event.get_action_strength("down")  - event.get_action_strength("up")
	)
	
	tween.tween_property(self, 'position', position + (input_vec*tile_size),1.0/animation_speed).set_trans(Tween.TRANS_SINE)
	is_moving = true
	await tween.finished
	is_moving = false
	
	
	move_and_slide()
