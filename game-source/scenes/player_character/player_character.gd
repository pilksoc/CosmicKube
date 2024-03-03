extends CharacterBody2D

@export var tile_size: float = 100
@export var is_moving: bool = false

@onready var wsClient: WebSocketClient = get_tree().current_scene.find_child('WebSocketClient')
@onready var uuid_util = preload('res://uuid.gd')

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

const spawnList = {
	"hydrogen": "5652c78a-8bfe-452a-ac29-53746cabfa40",
	"oxygen": "5585b1f1-8be4-498f-990c-46fadf68425f",
	"nitrogen": "33fc0e3a-fc48-4785-82fa-4a5d93bf0917",
	"calcium": "dee34cee-7ef5-46ce-b6d2-e7915669f21c",
	"iron": "b1951823-bf0f-47c2-82ae-9053443bda9b"
}

func _ready():
	position = position.snapped(Vector2.ONE * tile_size)
	position += Vector2.ONE * tile_size / 2
	
	wsClient.send({
		"initialised": true, 
		"player": {
			"name": "Player-" + uuid_util.v4(),
			"uuid": uuid_util.v4()
		},
		"coordinates": [position.x, position.y]
	})
	print(wsClient.get_message())

func _input(event):
	var vec = Input.get_vector("left", "right", "up", "down")

	if Input.is_key_pressed(KEY_SPACE):
		wsClient.send({})


	if vec.length() == 1:
		print(inputs_rev[vec])
		position += vec*100
		$AnimationPlayer.play(inputs_rev[vec])
	
