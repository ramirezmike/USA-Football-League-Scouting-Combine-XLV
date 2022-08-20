#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct CustomMaterial {
    color: vec4<f32>,
};

struct Time {
    time: f32,
};

struct XScrollSpeed {
    value: f32,
};

struct YScrollSpeed {
    value: f32,
};

struct Scale {
    value: f32,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
var<uniform> material: CustomMaterial;
@group(1) @binding(3)
var<uniform> time: Time;
@group(1) @binding(4)
var<uniform> x_scroll_speed: XScrollSpeed;
@group(1) @binding(5)
var<uniform> y_scroll_speed: YScrollSpeed;
@group(1) @binding(6)
var<uniform> scale: Scale;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let x_speed = x_scroll_speed.value;
    let y_speed = y_scroll_speed.value;
    let scale_value = scale.value;

    let uv = vec2((time.time * x_speed + in.uv.x / scale_value) % 1.0, (time.time * y_speed + in.uv.y / scale_value) % 1.0);
    var texture_sample = textureSample(texture, texture_sampler, uv);
    if (texture_sample.x == 0.0 && texture_sample.y == 0.0) {
        discard;
    }

    return material.color * texture_sample;
}
