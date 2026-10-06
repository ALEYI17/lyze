shader_type spatial;
render_mode unshaded;

instance uniform bool effect_enabled = false;
uniform sampler2D albedo_texture : source_color, filter_nearest;

void vertex() {
    MODELVIEW_MATRIX = VIEW_MATRIX * mat4(
        INV_VIEW_MATRIX[0],
        INV_VIEW_MATRIX[1],
        INV_VIEW_MATRIX[2],
        MODEL_MATRIX[3]
    );

    MODELVIEW_MATRIX[0] *= length(MODEL_MATRIX[0].xyz);
    MODELVIEW_MATRIX[1] *= length(MODEL_MATRIX[1].xyz);
    MODELVIEW_MATRIX[2] *= length(MODEL_MATRIX[2].xyz);
}

void fragment() {
  vec4 tex = texture(albedo_texture, UV);
	if (effect_enabled) {
		ALBEDO = tex.rgb * vec3(1.0, 0.3, 0.3);
	} else {
		ALBEDO = tex.rgb;
	}
	ALPHA = tex.a;
}
