use std::mem;
use random;
use glsl::icosahedron;
use chunk::{IntCoord, Chunk, Cell, CellPos, Root, ROOT_QUADS};
use nalgebra;
use nalgebra::{Point2, Point3, Vector3};
use rand;

type Pt2 = Point2<f64>;
type Pt3 = Point3<f64>;

pub struct Spec {
    seed: usize,
    radius: f64,
    cells_per_chunk: IntCoord,
    chunks_per_root_side: IntCoord,
    resolution: IntCoord,
}

pub struct Globe {
    spec: Spec,
    rand: random::RandomGenerator,
    chunk: Vec<Chunk>,
}

impl Globe {
    pub fn new(spec: Spec) -> Globe {
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
                       chunks_per_root_side: 4,
                       resolution: 3,
                       cells_per_chunk: 1,
                   })
    }

    // fn split_triangle(&self, triangle: &[[f64;2]; 3]) -> Vec<[Pt2; 3]> {
    fn split_triangle(&self, triangle: &[Pt2; 3]) -> Vec<[Pt2; 3]> {

        let mut triangles: Vec<[Pt2; 3]> = vec![];
        let a = triangle[0];
        let b = triangle[1];
        let c = triangle[2];
        let ab = nalgebra::center(&a, &b);
        let ac = nalgebra::center(&a, &c);
        let cb = nalgebra::center(&c, &b);
        triangles.push([a, ac, ab]);
        triangles.push([ab, b, cb]);
        triangles.push([ac, cb, ab]);
        triangles.push([ac, cb, c]);
        triangles
    }

    pub fn make_geometry(&self) -> (Vec<::Vertex>, Vec<u16>) {
        let mut triangles: Vec<[Pt2; 3]> = vec![];
        for triangle in icosahedron::VERTEX_IN_ROOT.iter() {
            let a = Pt2::from_coordinates(triangle[0].into());
            let b = Pt2::from_coordinates(triangle[1].into());
            let c = Pt2::from_coordinates(triangle[2].into());
            triangles.push([a, b, c]);
        }
        for _ in 0..self.spec.resolution {
            let mut temp = mem::replace(&mut triangles, vec![]);
            for triangle in &temp {
                triangles.extend(self.split_triangle(triangle));
            }
        }

        let mut vertex_data: Vec<::Vertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();
        for root_index in 0..ROOT_QUADS {
            for triangle in triangles.iter() {
                for point in triangle.iter() {
                    let pt3 = Globe::project_to_world(&Root::new(root_index), point);
                    let mut root_color = icosahedron::RAINBOW[root_index as usize];
                    for mut color_channel in root_color.iter_mut() {
                        *color_channel *= rand::random();
                    }
                    let vertex = ::Vertex::new([pt3[0] as f32, pt3[1] as f32, pt3[2] as f32],
                                               root_color);
                    let vertex_count = vertex_data.len();
                    vertex_data.push(vertex);
                    index_data.push(vertex_count as u16);
                }
            }
        }
        (vertex_data, index_data)
    }

    fn make_geometry_chunk(&self,
                           origin: CellPos,
                           vertex_data: &mut Vec<::Vertex>,
                           index_data: &mut Vec<u16>) {
        let chunks_in_two_triangles = self.spec.chunks_per_root_side / 2; // 1 root = 4 triangles
        let pt_a = icosahedron::VERTEX_IN_ROOT[0][0];
        let pt_f = icosahedron::VERTEX_IN_ROOT[3][2];
        let length_in_x = pt_f[0] - pt_a[0];
        let length_in_y = pt_f[1] - pt_a[1];
        let chunk_length_in_x = length_in_x / chunks_in_two_triangles as f64;
        let chunk_length_in_y = length_in_y / chunks_in_two_triangles as f64;
        for x in 0..chunks_in_two_triangles {
            for y in 0..chunks_in_two_triangles {
                //create cells instead of single vertex
                let pt3 = Globe::project_to_world(&origin.root,
                                                  &Pt2::new(chunk_length_in_x * x as f64,
                                                            chunk_length_in_y * y as f64));
                let root_color = icosahedron::RAINBOW[origin.root.index() as usize];
                let vertex = ::Vertex::new([pt3[0] as f32, pt3[1] as f32, pt3[2] as f32],
                                           root_color);
                let vertex_count = vertex_data.len();
                vertex_data.push(vertex);
                index_data.push(vertex_count as u16);
            }
        }
    }

    fn create_chunks(&mut self) {
        for root in 0..ROOT_QUADS {
            let root = Root::new(root);
            for y in 0..self.spec.chunks_per_root_side {
                for x in 0..self.spec.chunks_per_root_side {
                    let origin = CellPos {
                        root: root.clone(),
                        x: x * self.spec.chunks_per_root_side,
                        y: y * self.spec.chunks_per_root_side,
                    };
                    self.build_chunk(origin);
                }
            }
        }
    }

    fn build_chunk(&mut self, origin: CellPos) {
        let mut cells: Vec<Cell> = vec![];
        let end_x = origin.x + self.spec.cells_per_chunk;
        let end_y = origin.y + self.spec.cells_per_chunk;
        for cell_y in origin.y..end_y {
            for cell_x in origin.x..end_x {
                let p = [1.0, 2.0, 3.0];
                let height = 1.0 + self.rand.next(&p) * 0.1;
                cells.push(Cell { height: height })
            }
        }
    }

    pub fn project_to_world(root: &Root, pt_in_root_quad: &Pt2) -> Pt3 {
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
        let mut pt_in_root_quad = pt_in_root_quad.clone();
        let (i_north, i_south) = (root.index() as usize * 2, root.index() as usize * 2 + 11);
        assert!(i_north < 9 && i_north % 2 == 0);
        assert!(i_south > 9 && i_south < 20 && i_south % 2 == 1);
        //TODO Pt3::from(slice) Pt3 seems broken atm
        //TODO assert correct order of intermediate triangles
        let (a, b, c) = Globe::icosahedron_triangle_to_points(i_north);
        let (d, e, f) = Globe::icosahedron_triangle_to_points(i_south);
        let point: Pt3 = if pt_in_root_quad.x + pt_in_root_quad.y < 1.0 {
            //triangle abc
            a + (b - a) * pt_in_root_quad.x + (c - a) * pt_in_root_quad.y
        } else if pt_in_root_quad.y < 1.0 {
            //triangle bcd
            pt_in_root_quad.x = 1.0 - pt_in_root_quad.x;
            pt_in_root_quad.y = 1.0 - pt_in_root_quad.y;
            d + (c - d) * pt_in_root_quad.x + (b - d) * pt_in_root_quad.y
        } else if pt_in_root_quad.x + pt_in_root_quad.y < 2.0 {
            //triangle cde
            pt_in_root_quad.y -= 1.0;
            c + (d - c) * pt_in_root_quad.x + (e - c) * pt_in_root_quad.y
        } else {
            //triangle def
            pt_in_root_quad.y -= 1.0;
            pt_in_root_quad.x = 1.0 - pt_in_root_quad.x;
            pt_in_root_quad.y = 1.0 - pt_in_root_quad.y;
            f + (e - f) * pt_in_root_quad.x + (d - f) * pt_in_root_quad.y
        };
        Pt3::from_coordinates(nalgebra::normalize(&point.coords))
    }

    fn icosahedron_triangle_to_points(triangle_index: usize) -> (Pt3, Pt3, Pt3) {
        let triangle = icosahedron::TRIANGLE_LIST[triangle_index];
        let x: Vec<Pt3> = triangle
            .into_iter()
            .map(|p| {
                     let vertex = icosahedron::VERTICES[p.clone()];
                     Pt3::new(vertex[0], vertex[1], vertex[2])
                 })
            .collect();
        (x[0], x[1], x[2])
    }
}

//TODO more tests
#[cfg(test)]
mod project_to_first_triangle {
    use super::*;
    use glsl::icosahedron::{S, X};
    #[test]
    fn in_vertex() {
        let x = Globe::project_to_world(&Root::new(0), Pt2::new(0.0, 0.0));
        assert_eq!(Pt3::new(-S, X, 0.0), x);
    }
}
