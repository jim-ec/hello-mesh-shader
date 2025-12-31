enable wgpu_mesh_shader;

struct TaskPayload {
    color: vec3<f32>,
}

const N: u32 = 12;

const VERTEX_COUNT: u32 = N * N;
const TRI_COUNT: u32 = 2 * (N - 1) * (N - 1);

struct MeshOutput {
    @builtin(vertices) vertices: array<VertexOutput, VERTEX_COUNT>,
    @builtin(primitives) primitives: array<PrimitiveOutput, TRI_COUNT>,
    @builtin(vertex_count) vertex_count: u32,
    @builtin(primitive_count) primitive_count: u32,
}

struct PrimitiveOutput {
    @builtin(triangle_indices) indices: vec3<u32>,
    @builtin(cull_primitive) cull: bool,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

var<task_payload> payload: TaskPayload;

@task
@payload(payload)
@workgroup_size(1)
fn task() -> @builtin(mesh_task_size) vec3<u32> {
    payload.color = vec3(0.0, 0.0, 1.0);
    return vec3(1, 1, 1);
}

var<workgroup> out: MeshOutput;

@mesh(out)
@payload(payload)
@workgroup_size(1)
fn mesh() {
    out.vertex_count = VERTEX_COUNT;
    out.primitive_count = TRI_COUNT;

    for (var i = 0u; i < N; i++) {
        for (var j = 0u; j < N; j++) {
            let vertex_index = i * N + j;
            out.vertices[vertex_index].position = vec4(0.9 * (vec2(f32(i), f32(j)) / f32(N - 1) * 2.0 - 1.0), 0.0, 1.0);
            out.vertices[vertex_index].color = payload.color + vec3(f32(i), f32(j), 0.0) / f32(N - 1);
        }
    }

    for (var i = 0u; i < N - 1; i++) {
        for (var j = 0u; j < N - 1; j++) {
            // A - B
            // | \ |
            // C - D
            // #0: A C D
            // #1: D B A
            let a = (i + 0) * N + (j + 0);
            let b = (i + 0) * N + (j + 1);
            let c = (i + 1) * N + (j + 0);
            let d = (i + 1) * N + (j + 1);
            let quad_index = i * (N - 1) + j;
            out.primitives[2 * quad_index + 0].indices = vec3(a, c, d);
            out.primitives[2 * quad_index + 1].indices = vec3(d, b, a);
            out.primitives[2 * quad_index + 1].cull = true;
        }
    }
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1.0);
}
