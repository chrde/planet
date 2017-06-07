// const phi: f64 = (1.0 + 5.0f64.sqrt()) / 2.0;
// const s: f64 = (1.0 + phi.powi(2)).sqrt().recip();
// https://stackoverflow.com/questions/20850279/incorrectly-rendering-isocahedron-in-opengl
const PHI: f64 = 1.618033988749895;
const S: f64 = 0.5257311121191336;
const X: f64 = PHI * S;

pub const VERTICES: [[f64; 3]; 12] = [[X, S, 0.0],
                                      [-X, S, 0.0],
                                      [X, -S, 0.0],
                                      [-X, -S, 0.0],
                                      [S, 0.0, X],
                                      [S, 0.0, -X],
                                      [-S, 0.0, X],
                                      [-S, 0.0, -X],
                                      [0.0, X, S],
                                      [0.0, -X, S],
                                      [0.0, X, -S],
                                      [0.0, -X, -S]];

pub const TRIANGLE_LIST: [[usize; 3]; 20] =
    [[0, 8, 4], [1, 10, 7], [2, 9, 11], [7, 3, 1], [0, 5, 10], [3, 9, 6], [3, 11, 9], [8, 6, 4],
     [2, 4, 9], [3, 7, 11], [4, 2, 0], [9, 4, 6], [2, 11, 5], [0, 10, 8], [5, 0, 2], [10, 5, 7],
     [1, 6, 8], [1, 8, 10], [6, 1, 3], [11, 7, 5]];
