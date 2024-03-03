extends CharacterBody2D

@export var tile_size: float = 100
@export var is_moving: bool = false

@onready var wsClient: WebSocketClient = get_tree().current_scene.find_child('WebSocketClient')
@onready var container: VBoxContainer = get_tree().current_scene.find_child('CanvasLayer').find_child('SpawnListScroll').find_child('SpawnListContainer')
@onready var inventoryContainer: VBoxContainer = get_tree().current_scene.find_child('CanvasLayer').find_child('Inventory').find_child('InventoryList')
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

var inventory = {}

var selectedItem = ""

const spawnList = {
	"hydrogen": "5652c78a-8bfe-452a-ac29-53746cabfa40",
	"oxygen": "5585b1f1-8be4-498f-990c-46fadf68425f",
	"nitrogen": "33fc0e3a-fc48-4785-82fa-4a5d93bf0917",
	"calcium": "dee34cee-7ef5-46ce-b6d2-e7915669f21c",
	"iron": "b1951823-bf0f-47c2-82ae-9053443bda9b",
	"aluminium": "4941ce02-5992-4de3-9ead-23d9e7de5f0c",
	"uranium": "43375a4b-5950-4341-9d20-1b5086ddf9db",
	"sodium": "c0025eb6-15e1-4d0b-b9de-f08617ace76a",
	"chlorine": "45327e56-9ac9-46a2-8624-bd5c20d251f4",
	"light": "d9f02822-6f1b-4d99-8c71-ceba6af0f1e3",
	"time": "078e8865-0c37-466b-a219-b7122a0613cd",
	"silicon": "e59c2fbe-8975-4d20-a406-5a98df7d2455",
	"water": "1df74708-1124-4c05-a3f4-8ce2d14702a6",
	"salt": "a8bced32-f63a-4903-8932-a6d81e36c04f",
	"air": "2d88a7d2-5f23-4ca9-8c12-1550ac36490a",
	"dirt": "76952fb5-aa9d-4eda-a79a-8086ff35f5f1"
}

func _ready():
	position = position.snapped(Vector2.ONE * tile_size)
	position += Vector2.ONE * tile_size / 2
	
	for i in spawnList:
		var btn = Button.new()
		btn.text = i;
		btn.connect("pressed", spawnButtonPress.bind(spawnList[i]))
		container.add_child(btn)
	
	wsClient.send({
		"initialised": true, 
		"player": {
			"name": "Player-" + uuid_util.v4(),
			"uuid": uuid_util.v4()
		},
		"coordinates": [position.x, position.y]
	})
	print(wsClient.get_message())

func spawnButtonPress(uuid):
	print("added " + uuid)
	
	for n in inventoryContainer.get_children():
		n.remove_child(n)
		n.queue_free()
	
	if uuid in inventory:
		inventory[uuid] += 1
	else:
		inventory[uuid] = 1
	
	for k in inventory.keys():
		var lab = Button.new()
		lab.text = spawnList.find_key(k) + " x" + str(inventory[k])
		lab.connect("pressed", itemSelectButton.bind(k))
		inventoryContainer.add_child(lab)

func itemSelectButton(uuid):
	selectedItem = uuid
	print("New selected Item ", selectedItem)

func _input(event):
	var vec = Input.get_vector("left", "right", "up", "down")

	if Input.is_key_pressed(KEY_SPACE):
		wsClient.send({})


	if vec.length() == 1:
		print(inputs_rev[vec])
		position += vec*100
		$AnimationPlayer.play(inputs_rev[vec])
	
