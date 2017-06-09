use random;
use glsl::icosahedron;
use chunk::{IntCoord, Chunk, Cell, CellPos, Root, ROOT_QUADS};
use ::nalgebra::{Point2, Point3};

type Pt2 = Point2<f64>;
type Pt3 = Point3<f64>;

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
            let root = Root::new(root);
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

    pub fn project_to_world(root: Root, mut pt_in_root_quad: Pt2) -> Pt3 {
      // TODO : use half-edge-mesh to calculate this automatically instead of
      // relying on hardcoded order of icosahedron triangles
      // TODO : think about keeping positive x in [a,b,c,d] and negative x in [c,d,e,f]
      // and similar for y coordinates
      /*
      
          *a

      *b      *c

          *d      *e

              *f
      Assumptions:
      -5 root quads (0, 4),
      -i_north triangle contains [a, b, c]
      -i_south triangle contains [d, e, f]
      -inner triangles: [d, b, c], [d, e, c]
      */
      let (i_north, i_south) = (root.index() as usize * 2, root.index() as usize * 2 +11 );
      assert!(i_north > 0 && i_north < 10 && i_north % 2 == 0);
      assert!(i_south > 10 && i_south < 19 && i_south % 2 == 1);
      let (north_triangle, south_triangle) = (icosahedron::TRIANGLE_LIST[i_north], icosahedron::TRIANGLE_LIST[i_south]);
      //TODO Pt3::from(slice) Pt3 seems broken atm
      //TODO assert correct order of intermediate triangles
      let (a, b, c) = Globe::triangle_to_points(i_north);
      let (d, e, f) = Globe::triangle_to_points(i_south);
      let point = if pt_in_root_quad.x + pt_in_root_quad.y < 0.5 {
        //triangle abc
        a + (b-a)*pt_in_root_quad.x + (c-a)*pt_in_root_quad.y
      } else if pt_in_root_quad.y < 0.5 {
        //triangle bcd
        pt_in_root_quad.x = pt_in_root_quad.x - 0.5;
        pt_in_root_quad.y = pt_in_root_quad.y - 0.5;
          d + (c - d) * pt_in_root_quad.x + (b - d) * pt_in_root_quad.y
      } else if pt_in_root_quad.x + pt_in_root_quad.y < 1.0 {
        pt_in_root_quad.y = pt_in_root_quad.y - 0.5;
        c + (d -c) * pt_in_root_quad.x + (e - c) * pt_in_root_quad.y
        //triangle cde
      } else {
        unreachable!()
        //triangle def
      };
      point
    }

    fn triangle_to_points(triangle_index: usize) -> (Pt3, Pt3, Pt3) {
      let triangle = icosahedron::TRIANGLE_LIST[triangle_index];
      let x: Vec<Pt3> = triangle.into_iter().map(|p| {
        let vertex = icosahedron::VERTICES[p.clone()];
        Pt3::new(vertex[0], vertex[1], vertex[2])
      }).collect();
      (x[0], x[1], x[2])
    }

}

