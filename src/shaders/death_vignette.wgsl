#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct DeathParams {
    alpha: f32,
    resolution: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> params: DeathParams;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // pixel座標 -> 0〜1
    let uv = in.position.xy / params.resolution;

    // 中央基準
    let centered = abs(uv - vec2<f32>(0.5, 0.5));

    // 端方向
    let edge = max(centered.x, centered.y);

    // 端だけ
    let vignette = pow(
        smoothstep(0.4, 0.5, edge),
        4.0
    );

    let color = vec3<f32>(0.5, 0.0, 0.0);

    return vec4<f32>(
        color,
        vignette * params.alpha
    );
}
