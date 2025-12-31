enable wgpu_mesh_shader;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct MeshOutput {
    @builtin(vertices) vertices: array<VertexOutput, 3>,
    @builtin(primitives) primitives: array<PrimitiveOutput, 1>,
    @builtin(vertex_count) vertex_count: u32,
    @builtin(primitive_count) primitive_count: u32,
}

struct PrimitiveOutput {
    @builtin(triangle_indices) indices: vec3<u32>,
    @builtin(cull_primitive) cull: bool,
}

var<workgroup> out: MeshOutput;

@mesh(out)
@workgroup_size(1)
fn mesh() {
    out.vertex_count = 3;
    out.primitive_count = 1;

    for (var i = 0u; i < 3u; i++) {
        // (0, 0) (0, 1) (1, 0)
        let id = (vec2(0x1u, 0x2u) >> vec2(i)) & vec2(1u);
        let uv = vec2<f32>(id);

        out.vertices[i].position = vec4(uv * 2.0 - 1.0, 0.0, 1.0);
        out.vertices[i].color = vec3(uv, 0.0);
    }

    out.primitives[0].indices = vec3(0, 1, 2);
    out.primitives[0].cull = false;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1.0);
}
