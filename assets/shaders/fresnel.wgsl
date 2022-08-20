#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct CoolMaterial {
    color: vec4<f32>,
};

struct Time {
    time: f32,
};


@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
var<uniform> material: CoolMaterial;
@group(1) @binding(3)
var<uniform> time: Time;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

fn refract(I: vec3<f32>, N: vec3<f32>, eta: f32) -> vec3<f32> {
    let k = max((1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I))), 0.0);
    return eta * I - (eta * dot(N, I) + sqrt(k)) * N;
}

let TAU: f32 = 6.28318530717958647692528676655900577;

fn dir_to_equirectangular(dir: vec3<f32>) -> vec2<f32> {
    let x = atan2(dir.z, dir.x) / TAU + 0.5; // 0-1
    let y = acos(dir.y) / PI; // 0-1
    return vec2<f32>(x, y);
}

let EPSILON: f32 = 1.0000001;
let MAX_STEPS: i32 = 500;
let MIN_DIST: f32 = 0.0;
let MAX_DIST: f32 = 25.0;

let AMBIENT: f32 = 0.1;
let EDGE_THICKNESS: f32 = 0.05;
let SHADES: f32 = 4.0;

//  fn March(direction: vec3<f32>, start: f32, stop: f32, edgeLength: ptr<function, f32>) -> f32 {
//      var depth: f32 = start;

//      let max_steps = 500;
//      for (var i: i32 = 0; i < max_steps; i = i + 1) {
//          let dist: f32 = length(depth * direction);
//          (*edgeLength) = min(dist, (*edgeLength));
//          if (dist < EPSILON) {		
//              return depth;
//          }

//          if (dist > (*edgeLength) && (*edgeLength) <= EDGE_THICKNESS) {		
//              return 0.;
//          }

//          depth = depth + (dist);
//          if (depth >= stop) {		
//              break;
//          }
//      }

//      return stop;
//  }

//  @fragment
//  fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
//      var N = normalize(in.world_normal);
//      var V = normalize(view.world_position.xyz - in.world_position.xyz);
//      let NdotV = max(dot(N, V), 0.0001);

//      var fresnel = clamp(1.0 - NdotV, 0.5, 1.0);
//      fresnel = pow(fresnel, 9.0) * 20.0;
//      var col = vec3(material.color.xyz) + fresnel;

//      return vec4(col, 0.0);
//  }

fn plot(st: vec2<f32>, pct: f32) -> f32 {
    return smoothstep(pct - 0.01, pct, st.y) -
           smoothstep(pct, pct+0.01, st.y);
}

fn rgb2hsb(c: vec3<f32>) -> vec3<f32> {
    var K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    var p = mix(vec4(c.bg, K.wz),
                 vec4(c.gb, K.xy),
                 step(c.b, c.g));
    var q = mix(vec4(p.xyw, c.r),
                 vec4(c.r, p.yzx),
                 step(p.x, c.r));
    var d: f32 = q.x - min(q.w, q.y);
    var e: f32 = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)),
                d / (q.x + e),
                q.x);
}

fn hsb2rgb(c: vec3<f32>) -> vec3<f32> {
    var rgb = (c.x * 6.0 + vec3(0.0, 4.0, 2.0)) % 6.0;
    var abs_rgb = abs(rgb - 3.0) - 1.0;

    //var clamped = clamp(abs_rgb, 0.0, 1.0);
    var rgb = abs_rgb*abs_rgb*(2.0*abs_rgb);

    return c.z * mix(vec3(1.0), rgb, c.y);
}


//  @fragment
//  fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

//  //  let x = cos(view.world_position.x + time.time);
//  //  var y = sin(time.time+in.uv.x) / 2.0;

//  //  var y = sin(in.uv.x + time.time * 0.4);
//  //  var color = vec3(y);

//  //  var pct = plot(in.uv.xy, y);
//  //  color = (1.0 - pct) * color + pct*vec3(0.0, 1.0, 0.0);

//  //  var colorA = vec3(0.149,0.141,0.912);
//  //  var colorB = vec3(1.000,0.933,0.224);

//  //  var pct = vec3(in.uv.x);
//  //  color = mix(colorA, colorB, pct);

//  //  color = mix(color, vec3(1.0, 0.0, 0.0), plot(in.uv.xy, pct.r));
//  //  color = mix(color, vec3(0.0, 1.0, 0.0), plot(in.uv.xy, pct.g));
//  //  color = mix(color, vec3(0.0, 0.0, 1.0), plot(in.uv.xy, pct.b));

//  //  let TWO_PI = 6.28318530718;

//  //  var color = vec3(0.0);
//  //  var toCenter = vec2(0.5)-in.uv.xy;
//  //  var angle = atan2(toCenter.y, toCenter.x) + time.time;
//  //  var radius = length(toCenter) * 2.0;
//  //  var x = angle / TWO_PI;
//  //  var f = x + 0.5;

//  //  color = hsb2rgb(vec3(f, radius, 0.5));

//  //  var left = smoothstep(in.uv.y, in.uv.x, in.uv.y);
//  //  var right = step(vec2(0.9), in.uv);
//  //  color = vec3(left, right.x, 1.0);


//      var color = vec3(0.5);
//      var d = 0.0;

//      var st = in.uv.xy * 2.0 - 1.0;

//      d = length(abs(st) - 0.3);

//      return vec4(vec3(fract(d * 10.0 + time.time), fract(d * 4.0 + time.time), 0.8), 1.0);
//  }

fn march(origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    let MAX_STEPS = 500;
    let MIN_DIST = 0.0;
    let MAX_DIST = 25.0;
    let EDGE_THICKNESS = 0.15;

    var edgeLength = MAX_DIST;
    var depth = MIN_DIST;

    for (var i = 0; i < MAX_STEPS; i++) {
        var dist = 1.0;
        edgeLength = min(dist, edgeLength);

        if (dist < EPSILON) {
            return depth;
        }

        if (dist < edgeLength && edgeLength <= EDGE_THICKNESS) {
            return 0.0;
        }

        depth += dist; // step?

        if (depth >= MAX_DIST) {
            break;
        }
    }


    return MAX_DIST;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

//  if (in.world_position.x == view.world_position.x) {
//      return vec4(1.0, 0.0, 0.0, 1.0);
//  }

//  var dist = march(vec3(in.uv.x), vec3(in.uv.y)); 
//  if (dist < EPSILON) {
//      return vec4(0.0, 0.5, 0.5, 1.0);
//  }

//  return vec4(vec3(0.0, 0.0, 1.0), 1.0);

    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    let NdotV = max(dot(N, V), .35);

    var fresnel = clamp(1.1 - NdotV, 0.0, 1.0);
    fresnel = pow(fresnel, 9.0 + sin(time.time * 2.5)) * 20.0;
    var col = vec3(material.color.xyz) * fresnel;


//    return vec4(col, 0.0);



    let layer = i32(in.world_position.x) & 0x3;
    var pbr_input: PbrInput = pbr_input_new();

    pbr_input.material.base_color = vec4(col, 0.0);

    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = prepare_normal(
        pbr_input.material.flags,
        in.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
        in.uv,
        in.is_front,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    return tone_mapping(pbr(pbr_input));
}
