shader_type spatial;
render_mode unshaded;

uniform bool use_billboard;
void vertex() {
	if (use_billboard){
        MODELVIEW_MATRIX = VIEW_MATRIX * mat4(INV_VIEW_MATRIX[0], INV_VIEW_MATRIX[1], INV_VIEW_MATRIX[2], MODEL_MATRIX[3]);
	}
}

instance uniform bool effect_enabled = false;
uniform sampler2D albedo_texture : source_color;

void fragment() {
  vec4 tex = texture(albedo_texture, UV);
	if (effect_enabled) {
		ALBEDO = tex.rgb * vec3(1.0, 0.3, 0.3);
	} else {
		ALBEDO = tex.rgb;
	}
	ALPHA = tex.a;
}
