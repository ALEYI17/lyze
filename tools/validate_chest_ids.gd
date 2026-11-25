extends SceneTree

const LEVELS_DIRECTORY := "res://scenes/levels"
const DEFAULT_CHEST_ID := 0

var chest_ids: Dictionary = {}
var error_count := 0

func _initialize() -> void:
	call_deferred("_validate_levels")

func _validate_levels() -> void:
	var scene_paths: Array[String] = []
	_collect_level_scenes(LEVELS_DIRECTORY, scene_paths)
	scene_paths.sort()

	for scene_path in scene_paths:
		var packed_scene := ResourceLoader.load(scene_path) as PackedScene
		if packed_scene == null:
			push_error("Could not load level scene: %s" % scene_path)
			error_count += 1
			continue

		_scan_packed_scene(packed_scene, scene_path, "", [], false)

	if error_count > 0:
		push_error("Chest ID validation failed with %d error(s)." % error_count)
		quit(1)
	else:
		print(
			"Chest ID validation passed: %d chest(s) across %d level scene(s)."
			% [chest_ids.size(), scene_paths.size()]
		)
		quit(0)

func _collect_level_scenes(directory_path: String, scene_paths: Array[String]) -> void:
	var directory := DirAccess.open(directory_path)
	if directory == null:
		push_error("Could not open levels directory: %s" % directory_path)
		error_count += 1
		return

	directory.list_dir_begin()
	var entry := directory.get_next()
	while not entry.is_empty():
		if entry != "." and entry != ".." and not entry.begins_with("."):
			var entry_path := directory_path.path_join(entry)
			if directory.current_is_dir():
				_collect_level_scenes(entry_path, scene_paths)
			elif entry.ends_with(".tscn"):
				scene_paths.append(entry_path)
		entry = directory.get_next()
	directory.list_dir_end()

func _scan_packed_scene(
	packed_scene: PackedScene,
	level_path: String,
	prefix: String,
	scene_stack: Array[String],
	skip_root: bool,
) -> void:
	var resource_path := packed_scene.resource_path
	if scene_stack.has(resource_path):
		push_error("Cyclic scene instance while scanning %s: %s" % [level_path, resource_path])
		error_count += 1
		return

	var next_stack := scene_stack.duplicate()
	next_stack.append(resource_path)
	var state := packed_scene.get_state()
	for index in range(state.get_node_count()):
		var relative_path := String(state.get_node_path(index))
		var node_path := relative_path if prefix.is_empty() else prefix.path_join(relative_path)
		var properties := _get_node_properties(state, index)
		var instance := state.get_node_instance(index)
		var is_chest := String(state.get_node_type(index)) == "ChestNode"
		var chest_default_id := DEFAULT_CHEST_ID

		if instance != null:
			var instance_state := instance.get_state()
			if instance_state.get_node_count() > 0 and String(instance_state.get_node_type(0)) == "ChestNode":
				is_chest = true
				var template_properties := _get_node_properties(instance_state, 0)
				chest_default_id = int(template_properties.get("chest_id", DEFAULT_CHEST_ID))

		if is_chest and not (skip_root and index == 0):
			var chest_id := int(properties.get("chest_id", chest_default_id))
			_register_chest_id(chest_id, level_path, node_path)

		if instance != null:
			_scan_packed_scene(instance, level_path, node_path, next_stack, true)

func _get_node_properties(state: SceneState, node_index: int) -> Dictionary:
	var properties: Dictionary = {}
	for property_index in range(state.get_node_property_count(node_index)):
		var property_name := String(state.get_node_property_name(node_index, property_index))
		properties[property_name] = state.get_node_property_value(node_index, property_index)
	return properties

func _register_chest_id(chest_id: int, level_path: String, node_path: String) -> void:
	if chest_id < 0:
		push_error("Chest ID cannot be negative: %s:%s has ID %d" % [level_path, node_path, chest_id])
		error_count += 1
		return

	if chest_ids.has(chest_id):
		var previous: Dictionary = chest_ids[chest_id]
		push_error(
			"Duplicate chest ID %d: %s:%s and %s:%s"
			% [chest_id, previous["scene"], previous["node"], level_path, node_path]
		)
		error_count += 1
	else:
		chest_ids[chest_id] = {"scene": level_path, "node": node_path}
