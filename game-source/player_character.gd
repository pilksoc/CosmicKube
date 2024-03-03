extends CharacterBody2D


@export var move_speed: float = 400


func _physics_process(delta):
	var input_dir =  Vector2(
		Input.get_action_strength("right") - Input. get_action_strength("left"),
		Input.get_action_strength("down")  - Input.get_action_strength("up")	
	)
	
	velocity = input_dir*move_speed
	
	move_and_slide()
