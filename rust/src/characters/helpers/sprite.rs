use godot::{classes::{CharacterBody3D, Sprite3D}, prelude::*};

pub fn get_sprite(body: Gd<CharacterBody3D>) -> Option<Gd<Sprite3D>> {
    let sprite_handle = body.get_node_or_null("Sprite3D")?;

    let Ok(sprite) = sprite_handle.try_cast::<Sprite3D>() else {
        return None;
    };

    Some(sprite)
}

pub fn set_shader_true(sprite: &mut Gd<Sprite3D>) {
    sprite.set_instance_shader_parameter("effect_enabled", &true.to_variant());
}

pub fn set_shader_false(sprite: &mut Gd<Sprite3D>) {
    sprite.set_instance_shader_parameter("effect_enabled", &false.to_variant());
}

