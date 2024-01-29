use extism_pdk::*;
use serde::*;

#[derive(Serialize)]
struct Mesh {
    pub f_vertex_indices: Vec<u32>,
    pub nverts: Vec<u32>,
    pub verts: Vec<f64>,
}

#[derive(Deserialize)]
struct CombinedInput {
  pub n_stacks: u32,
  pub n_slices: u32,
}

#[plugin_fn]
pub fn uv_sphere(Json(input): Json<CombinedInput>) -> FnResult<Json<Mesh>> {
    let n_stacks: u32 = input.n_stacks;
    let n_slices: u32 = input.n_slices;
    let mut nverts: Vec<u32> = vec![];
    let mut f_vertex_indices: Vec<u32> = vec![];
    let mut verts: Vec<f64> = vec![];

    //  add top vertex
    verts.push(0.0);
    verts.push(1.0);
    verts.push(0.0);

    let v0: u32 = (verts.len() / 3) as u32 - 1;

    // generate vertices per stack / slice
    for i in 0..(n_stacks - 1) {
        let phi = std::f64::consts::PI * (i + 1) as f64 / n_stacks as f64;
        for j in 0..(n_slices) {
            let theta = std::f64::consts::PI * 2.0 * j as f64 / n_slices as f64;
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            verts.push(x);
            verts.push(y);
            verts.push(z);
        }
    }
    // add bottom vertex
    verts.push(0.0);
    verts.push(-1.0);
    verts.push(0.0);

    let v1: u32 = (verts.len() / 3) as u32 - 1;

    // add top / bottom triangles
    for i in 0..n_slices {
        let i0 = i + 1;
        let i1 = (i + 1) % n_slices + 1;
        nverts.push(3);
        f_vertex_indices.push(v0);
        f_vertex_indices.push(i1);
        f_vertex_indices.push(i0);
        let i0 = i + n_slices * (n_stacks - 2) + 1;
        let i1 = (i + 1) % n_slices + n_slices * (n_stacks - 2) + 1;
        nverts.push(3);
        f_vertex_indices.push(v1);
        f_vertex_indices.push(i1);
        f_vertex_indices.push(i0);
    }

    // add quads per stack / slice
    for j in 0..(n_stacks - 2) {
        let j0 = j * n_slices + 1;
        let j1 = (j + 1) * n_slices + 1;
        for i in 0..n_slices {
            let i0 = j0 + i;
            let i1 = j0 + (i + 1) % n_slices;
            let i2 = j1 + (i + 1) % n_slices;
            let i3 = j1 + i;
            nverts.push(4);
            f_vertex_indices.push(i0);
            f_vertex_indices.push(i1);
            f_vertex_indices.push(i2);
            f_vertex_indices.push(i3);
        }
    }

    let output = Mesh {
        f_vertex_indices,
        nverts,
        verts
    };

    Ok(Json(output))
}
