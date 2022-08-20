#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

// Shader: Pixelization
@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let PIXEL_SIZE = 6.0;

    let center_of_pixel = floor(PIXEL_SIZE / 2.0);
    let x_offset_within_pixel = position.x % PIXEL_SIZE;
    let y_offset_within_pixel = position.y % PIXEL_SIZE;

    // uncomment and use these for actual pixelization
//  let x_offset_from_center_of_pixel = center_of_pixel - x_offset_within_pixel;
//  let y_offset_from_center_of_pixel = center_of_pixel - y_offset_within_pixel;
    // skipping the center pixel selection makes a more refraction look
    let x_offset_from_center_of_pixel = x_offset_within_pixel;
    let y_offset_from_center_of_pixel = y_offset_within_pixel;

    let closest_center_pixel = vec2<f32>(position.x + x_offset_from_center_of_pixel, 
                                         position.y + y_offset_from_center_of_pixel);

    var uv = closest_center_pixel / vec2<f32>(view.width, view.height);

    return vec4<f32>(textureSample(texture, our_sampler, uv).xyz, 1.0);
}
