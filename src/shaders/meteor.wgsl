#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct MeteorUniform {
    velocity_dir: vec2<f32>,
    speed_squared: f32,
    world_position: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> params: MeteorUniform;

// -------------------------
// hash noise
// -------------------------

fn hash(p: vec2<f32>) -> f32 {
    return fract(
        sin(
            dot(
                p,
                vec2<f32>(127.1, 311.7)
            )
        ) * 43758.5453
    );
}

// -------------------------
// main
// -------------------------

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    // =====================================================
    // local UV
    // =====================================================

    let uv = in.uv;

    let centered = uv - vec2<f32>(0.5, 0.5);

    let dist = length(centered);

    // =====================================================
    // 円形
    // =====================================================

    let circle = 1.0
        - smoothstep(
        0.42,
        0.5,
        dist
    );

    // =====================================================
    // world方向
    // =====================================================

    let velocity_dir = normalize(params.velocity_dir);

    let pixel_world = in.world_position.xy;

    let world_dir = normalize(
        pixel_world
            - params.world_position
    );

    // 前方向
    let front = max(
        dot(
            world_dir,
            velocity_dir
        ),
        0.0
    );

    // 後方向
    let back = max(
        dot(
            world_dir,
            -velocity_dir
        ),
        0.0
    );

    // =====================================================
    // speed
    // =====================================================

    let speed = sqrt(params.speed_squared);

    let speed_factor = clamp(
        speed * 0.02,
        0.0,
        1.5
    );

    // =====================================================
    // 隕石表面
    // =====================================================

    let noise = hash(
        floor(uv * 10.0)
    );

    // クレーター
    let crater = smoothstep(
        0.25,
        0.0,
        distance(
            fract(uv * 4.0),
            vec2<f32>(0.5)
        )
    );

    // 岩色
    let rock = vec3<f32>(
        0.08,
        0.09,
        0.11
    );

    // 青鉱物
    let mineral = vec3<f32>(
        0.05,
        0.2,
        0.45
    );

    // ベース表面
    var surface = mix(
        rock,
        mineral,
        noise * 0.35
    );

    // クレーター暗化
    surface *= 1.0
        - crater * 0.3;

    // 中央少しだけ明るい
    let center_shade = 1.15
        - dist * 1.1;

    surface *= center_shade;

    // =====================================================
    // 外周
    // =====================================================

    // 外周だけ強調
    let edge = smoothstep(
        0.18,
        0.48,
        dist
    );

    // =====================================================
    // 摩擦熱 glow
    // =====================================================

    // 前面を広く
    let burn_front = smoothstep(
        0.0,
        0.8,
        front
    );

    // 「前方向 × 外周」
    let burn = burn_front
        * edge;

    // ノイズ揺らぎ
    let burn_noise = 0.7
        + noise * 0.6;

    // 発光
    let glow = burn
        * burn_noise
        * speed_factor
        * 0.45;

    // 高温青白
    let glow_color = vec3<f32>(
        0.3,
        0.7,
        1.3
    );

    // =====================================================
    // 尾
    // =====================================================

    // 後方方向
    let trail_dir = pow(back, 3.5);

    // 外側ほど強い
    let trail_edge = smoothstep(
        0.15,
        0.55,
        dist
    );

    // 尾
    let trail = trail_dir
        * trail_edge
        * speed_factor
        * 0.35;

    let trail_color = vec3<f32>(
        0.12,
        0.45,
        1.0
    );

    // =====================================================
    // 最終色
    // =====================================================

    let color = surface
        + glow_color * glow
        + trail_color * trail;

    // =====================================================
    // alpha
    // =====================================================

    let trail_alpha = trail
        * smoothstep(
        1.0,
        0.3,
        dist
    );

    let alpha = max(
        circle,
        trail_alpha * 0.5
    );

    return vec4<f32>(
        color,
        alpha
    );
}
