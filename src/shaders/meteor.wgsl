#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct MeteorUniform {
    velocity_dir: vec2<f32>,
    speed_squared: f32,
    world_position: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> params: MeteorUniform;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    // UV (0~1)
    let uv = in.uv;

    // 中心を(0,0)へ
    let centered = uv - vec2<f32>(0.5f, 0.5f);

    // 円形距離
    let dist = length(centered);

    // 円形マスク
    let circle = 1.0f - smoothstep(
        0.45f,
        0.5f,
        dist
    );

    // 円外透明
    if circle <= 0.001f {
        discard;
    }

    // ===== world空間方向 =====

    // pixelのworld座標
    let pixel_world = in.world_position.xy;

    // 中心→pixel
    let world_dir = normalize(
        pixel_world
            - params.world_position
    );

    // 前方強調
    let front = max(
        dot(
            world_dir,
            normalize(params.velocity_dir)
        ),
        0.0f
    );

    // speed調整
    let speed = sqrt(params.speed_squared);

    // 発光強度
    let glow = pow(front, 6.0f)
        * speed
        * 0.02f;

    // エッジ発光
    let edge = smoothstep(
        0.25f,
        0.5f,
        dist
    );

    // 隕石本体色
    let base_color = vec3<f32>(
        0.0f,
        0.4f,
        0.9f
    );

    // 青白い発光
    let glow_color = vec3<f32>(
        0.4f,
        0.8f,
        1.5f
    );

    // 中央暗め
    let center_shade = 1.0f - dist * 0.8f;

    // 最終色
    let color = base_color * center_shade
        + glow_color * glow
        + glow_color * edge * glow * 0.5f;

    // alpha
    let alpha = circle
        * (0.8f + glow * 0.5f);

    return vec4<f32>(
        color,
        alpha
    );
}
