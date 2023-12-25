#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;
@group(2) @binding(0) var<uniform> material_color: vec4f;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn random(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@fragment
fn fragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let x = (pos.x / 500.) - 1.2;
    let y = (pos.y / 500.) - 1.2;

    let color = vec3f(
        random(u32(pos.x / 20.) + u32(pos.y / 20.) * 444444u + u32(globals.time) * 555555u),
        random(u32(pos.x / 20. + globals.time) * 656463u + u32(pos.y / 20.) * 777777u + u32(globals.time) * 100000u),
        0.5
    );

    let dist = sqrt(x * x + y * y);

    return vec4f(color * (1.0 - dist), 1.);
}
