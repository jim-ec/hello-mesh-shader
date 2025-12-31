struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // (0, 0) (0, 1) (1, 0)
    let id = (vec2(0x1u, 0x2u) >> vec2(vertex_index)) & vec2(1u);
    let uv = vec2<f32>(id);

    var out: VertexOutput;
    out.position = vec4<f32>(uv, 0.0, 1.0);
    out.color = vec4<f32>(uv, 0.0, 1.0);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
