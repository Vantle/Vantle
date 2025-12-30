struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct Configuration {
    dimensions: vec2<f32>,
    width: f32,
    padding: f32,
}

@group(0) @binding(0)
var mask: texture_2d<f32>;

@group(0) @binding(1)
var sampler_mask: sampler;

@group(0) @binding(2)
var<uniform> config: Configuration;

@group(0) @binding(3)
var<storage, read> palette: array<vec4<f32>, 256>;

@vertex
fn vertex(@builtin(vertex_index) index: u32) -> Output {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );

    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0)
    );

    var output: Output;
    output.position = vec4<f32>(positions[index], 0.0, 1.0);
    output.uv = uvs[index];
    return output;
}

@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let texel = vec2<f32>(1.0 / config.dimensions.x, 1.0 / config.dimensions.y);
    let center = textureSample(mask, sampler_mask, input.uv);

    var neighbor = 0.0;
    var index = 0.0;

    for (var dy: i32 = -2; dy <= 2; dy = dy + 1) {
        for (var dx: i32 = -2; dx <= 2; dx = dx + 1) {
            if (dx == 0 && dy == 0) {
                continue;
            }
            let offset = vec2<f32>(f32(dx), f32(dy)) * texel * config.width;
            let sample = textureSample(mask, sampler_mask, input.uv + offset);
            if (sample.r > neighbor) {
                neighbor = sample.r;
                index = sample.g;
            }
        }
    }

    let entry = palette[u32(index * 255.0 + 0.5)];
    let color = entry.rgb;
    let width = entry.w;

    let edge = neighbor * (1.0 - center.r);
    let threshold = 0.5 * config.width / width;
    let alpha = smoothstep(0.0, threshold, edge);

    if (alpha < 0.01) {
        discard;
    }

    return vec4<f32>(color, alpha);
}
