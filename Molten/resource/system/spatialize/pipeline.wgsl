struct Uniform {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    eye: vec3<f32>,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniform;

struct Input {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) effects: u32,
    @location(4) palette: u32,
}

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) world: vec3<f32>,
    @location(3) @interpolate(flat) effects: u32,
    @location(4) @interpolate(flat) palette: u32,
}

struct Fragment {
    @location(0) color: vec4<f32>,
    @location(1) mask: vec2<f32>,
}

@vertex
fn vertex(input: Input) -> Output {
    var output: Output;
    let world = vec4<f32>(input.position, 1.0);
    let view = uniforms.view * world;
    output.position = uniforms.projection * view;
    output.normal = input.normal;
    output.color = input.color;
    output.world = input.position;
    output.effects = input.effects;
    output.palette = input.palette;
    return output;
}

const OUTLINE: u32 = 1u;

@fragment
fn fragment(input: Output) -> Fragment {
    let light = normalize(uniforms.eye - input.world);
    let normal = normalize(input.normal);
    let diffuse = max(dot(normal, light), 0.0);
    let ambient = 0.3;
    let intensity = ambient + diffuse * 0.7;

    var output: Fragment;
    output.color = vec4<f32>(input.color.rgb * intensity, input.color.a);
    output.mask = vec2<f32>(
        f32((input.effects & OUTLINE) != 0u),
        f32(input.palette) / 255.0
    );
    return output;
}
