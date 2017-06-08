use random;
use glsl::icosahedron;
use chunk::{IntCoord, Chunk, Cell, CellPos, Root};

const ROOT_QUADS: u8 = 10;

pub struct Spec {
    seed: usize,
    radius: f64,
    root_resolution: IntCoord,
    chunk_resolution: IntCoord,
}

impl Spec {
    pub fn is_valid(&self) -> bool {
        let chunks_per_root_side = self.root_resolution / self.chunk_resolution;
        chunks_per_root_side * self.chunk_resolution == self.root_resolution
    }
}

pub struct Globe {
    spec: Spec,
    rand: random::RandomGenerator,
    chunk: Vec<Chunk>,
}

impl Globe {
    pub fn new(spec: Spec) -> Globe {
        assert!(spec.is_valid(), "Invalid globe spec!");
        let mut globe = Globe {
            rand: random::RandomGenerator::new(spec.seed),
            spec: spec,
            chunk: vec![],
        };
        globe.create_chunks();
        globe
    }

    pub fn new_example() -> Globe {
        Globe::new(Spec {
                       seed: 42,
                       radius: 1.2,
                       root_resolution: 256,
                       chunk_resolution: 16,
                   })
    }

    pub fn make_geometry(&self) -> (Vec<::Vertex>, Vec<u16>) {

        let temp_vertex_data: Vec<::Vertex> = icosahedron::VERTICES
            .iter()
            .map(|v| {
                     assert_eq!(3, v.len());
                     let distortion = (1.0 + self.rand.next(v) * 0.1) * self.spec.radius;
                     let (x, y, z) = ((v[0] * distortion) as f32,
                                      (v[1] * distortion) as f32,
                                      (v[2] * distortion) as f32);
                     ::Vertex::new([x, y, z], [0.0, 0.0, 0.0])
                 })
            .collect();

        let mut vertex_data: Vec<::Vertex> = Vec::new();
        let index_data: Vec<u16> = icosahedron::TRIANGLE_LIST
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                let mut new_triangle = vec![];
                for v in t.iter().cloned() {
                    let mut colored_vertex = temp_vertex_data[v].clone();
                    colored_vertex.a_color = icosahedron::RAINBOW[i / 2];
                    new_triangle.push(vertex_data.len());
                    vertex_data.push(colored_vertex);
                }
                new_triangle
            })
            .map(|v| v as u16)
            .collect();

        (vertex_data, index_data)
    }

    fn create_chunks(&mut self) {
        let chunks_per_root = self.spec.root_resolution / self.spec.chunk_resolution;
        for root in 0..ROOT_QUADS {
            let root = Root { index: root };
            for y in 0..chunks_per_root {
                for x in 0..chunks_per_root {
                    let origin = CellPos {
                        root: root.clone(),
                        x: x * chunks_per_root,
                        y: y * chunks_per_root,
                    };
                    self.build_chunk(origin);
                }
            }
        }
    }

    fn build_chunk(&mut self, origin: CellPos) {
        let mut cells: Vec<Cell> = vec![];
        let end_x = origin.x + self.spec.chunk_resolution;
        let end_y = origin.y + self.spec.chunk_resolution;
        for cell_y in origin.y..end_y {
            for cell_x in origin.x..end_x {
                let p = [1.0, 2.0, 3.0];
                let height = 1.0 + self.rand.next(&p) * 0.1;
                cells.push(Cell { height: height })
            }
        }
    }
}

