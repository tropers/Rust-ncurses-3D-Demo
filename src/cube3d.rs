// Not all matrix transformation functions are used in this demo
#![allow(dead_code)]

use std::ops;

const TRIANGLE_VERTEX_COUNT: usize = 3;
const VERTEX_INDEX_1: usize = 0;
const VERTEX_INDEX_2: usize = 1;
const VERTEX_INDEX_3: usize = 2;

#[derive(Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn round(&self) -> Vec2 {
        Vec2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
}

#[derive(Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn project(&self) -> Vec3 {
        let near: f32 = 0.1;
        let far: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32;
        unsafe {
            aspect_ratio = crate::SCREEN_HEIGHT / crate::SCREEN_WIDTH;
        }

        let fov_rad: f32 = 1.0 / f32::tan(fov * 0.5 / 180.0 * std::f32::consts::PI);

        let proj_matrix = Matrix4x4 {
            data: [
                [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                [0.0, fov_rad, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), 1.0],
                [0.0, 0.0, (-far * near) / (far - near), 0.0],
            ],
        };

        proj_matrix * self
    }

    fn rotate_x(&self, theta: f32) -> Vec3 {
        let rotate_x_matrix = Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, f32::cos(theta), f32::sin(theta), 0.0],
                [0.0, -f32::sin(theta), f32::cos(theta), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rotate_x_matrix * self
    }

    fn rotate_y(&self, theta: f32) -> Vec3 {
        let rotate_z_matrix = Matrix4x4 {
            data: [
                [f32::cos(theta), 0.0, f32::sin(theta), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-f32::sin(theta), 0.0, f32::cos(theta), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rotate_z_matrix * self
    }

    fn rotate_z(&self, theta: f32) -> Vec3 {
        let rotate_z_matrix = Matrix4x4 {
            data: [
                [f32::cos(theta), f32::sin(theta), 0.0, 0.0],
                [-f32::sin(theta), f32::cos(theta), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rotate_z_matrix * self
    }

    fn translate_x(&self, x: f32) -> Vec3 {
        Vec3 {
            x: self.x + x,
            y: self.y,
            z: self.z,
        }
    }

    fn translate_y(&self, y: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y + y,
            z: self.z,
        }
    }

    fn translate_z(&self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
}

#[derive(Clone)]
struct Vertex {
    position: Vec3,
}

impl Vertex {
    fn scale_into_screen_2d(&self) -> Vec2 {
        unsafe {
            Vec2 {
                x: (&self.position.x + 1.0) * 0.5 * crate::SCREEN_WIDTH,
                y: (&self.position.y + 1.0) * 0.5 * crate::SCREEN_HEIGHT,
            }
        }
    }
}

#[derive(Clone)]
struct Matrix4x4 {
    data: [[f32; 4]; 4],
}

impl ops::Mul<&Vec3> for Matrix4x4 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut return_vector = Vec3 {
            x: rhs.x * self.data[0][0]
                + rhs.y * self.data[1][0]
                + rhs.z * self.data[2][0]
                + self.data[3][0],
            y: rhs.x * self.data[0][1]
                + rhs.y * self.data[1][1]
                + rhs.z * self.data[2][1]
                + self.data[3][1],
            z: rhs.x * self.data[0][2]
                + rhs.y * self.data[1][2]
                + rhs.z * self.data[2][2]
                + self.data[3][2],
        };

        let w = rhs.x * self.data[0][3]
            + self.data[1][3]
            + rhs.z * self.data[2][3]
            + self.data[3][3];

        if w != 0.0 {
            return_vector.x /= w;
            return_vector.y /= w;
            return_vector.z /= w;
        }

        return_vector
    }
}

#[derive(Clone)]
struct Triangle {
    vertices: Vec<Vertex>,
}

impl Triangle {
    fn project(&self) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.project(),
                    }
                })
                .collect(),
        }
    }

    fn rotate_x(&self, theta: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.rotate_x(theta),
                    }
                })
                .collect(),
        }
    }

    fn rotate_y(&self, theta: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.rotate_y(theta),
                    }
                })
                .collect(),
        }
    }

    fn rotate_z(&self, theta: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.rotate_z(theta),
                    }
                })
                .collect(),
        }
    }

    fn translate_x(&self, x: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.translate_x(x),
                    }
                })
                .collect(),
        }
    }

    fn translate_y(&self, y: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.translate_y(y),
                    }
                })
                .collect(),
        }
    }

    fn translate_z(&self, z: f32) -> Triangle {
        Triangle {
            vertices: self
                .vertices
                .iter()
                .map(|vertex| -> Vertex {
                    Vertex {
                        position: vertex.position.translate_z(z),
                    }
                })
                .collect(),
        }
    }

    fn draw(&self, window: &pancurses::Window) {
        // Scale the vertexs x and y coordinates into the screen dimensions
        let scaled_vertices: Vec<Vec2> = self
            .vertices
            .iter()
            .map(|vertex: &Vertex| -> Vec2 { vertex.scale_into_screen_2d() })
            .collect();

        match scaled_vertices.len() {
            TRIANGLE_VERTEX_COUNT => {
                draw_line(
                    window,
                    &scaled_vertices[VERTEX_INDEX_1],
                    &scaled_vertices[VERTEX_INDEX_2],
                );
                draw_line(
                    window,
                    &scaled_vertices[VERTEX_INDEX_2],
                    &scaled_vertices[VERTEX_INDEX_3],
                );
                draw_line(
                    window,
                    &scaled_vertices[VERTEX_INDEX_1],
                    &scaled_vertices[VERTEX_INDEX_3],
                );
            }
            _ => {
                eprintln!(
                    "ERROR: Triangle is missing {} vertex / vertices!",
                    TRIANGLE_VERTEX_COUNT - scaled_vertices.len()
                );
                std::process::exit(1);
            }
        }
    }
}

struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    fn project(&self) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.project() })
                .collect(),
        }
    }

    fn rotate_x(&self, theta: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.rotate_x(theta) })
                .collect(),
        }
    }

    fn rotate_y(&self, theta: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.rotate_y(theta) })
                .collect(),
        }
    }

    fn rotate_z(&self, theta: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.rotate_z(theta) })
                .collect(),
        }
    }

    fn translate_x(&self, x: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.translate_x(x) })
                .collect(),
        }
    }

    fn translate_y(&self, y: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.translate_y(y) })
                .collect(),
        }
    }

    fn translate_z(&self, z: f32) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|triangle| -> Triangle { triangle.translate_z(z) })
                .collect(),
        }
    }

    fn draw(&self, window: &pancurses::Window) {
        self.triangles.iter().for_each(|triangle| {
            triangle.draw(window)
        });
    }
}

fn draw_line(window: &pancurses::Window, vec0: &Vec2, vec1: &Vec2) {
    // The vertices are rounded as a simple rasterization method
    let mut vec0_rounded = vec0.round();
    let vec1_rounded = vec1.round();

    let dx = (vec1_rounded.x - vec0_rounded.x).abs();
    let sx = match vec0_rounded.x < vec1_rounded.x {
        true => 1.0,
        false => -1.0,
    };

    let dy = -(vec1_rounded.y - vec0_rounded.y).abs();
    let sy = match vec0_rounded.y < vec1_rounded.y {
        true => 1.0,
        false => -1.0,
    };

    let mut error = dx + dy;

    loop {
        window.mvaddch(vec0_rounded.y as i32, vec0_rounded.x as i32, '*');

        if vec0_rounded.x == vec1_rounded.x && vec0_rounded.y == vec1_rounded.y {
            break;
        }

        let e2 = 2.0 * error;
        if e2 >= dy {
            if vec0_rounded.x == vec1_rounded.x {
                break;
            }
            error += dy;
            vec0_rounded.x += sx;
        }

        if e2 <= dx {
            if vec0_rounded.y == vec1_rounded.y {
                break;
            }
            error += dx;
            vec0_rounded.y += sy;
        }
    }
}

fn draw_text(window: &pancurses::Window, theta: f32) {
    let y: f32;
    let x: f32;
    let text = "= 3D CUBE =";

    unsafe {
        y = (f32::sin(theta).abs() * crate::SCREEN_HEIGHT).round();
        x = ((f32::sin(theta * 5.0) * crate::SCREEN_WIDTH / 2.0) + crate::SCREEN_WIDTH / 2.0)
            .round();
    }

    window.attron(pancurses::A_BOLD);
    window.mvaddstr(y as i32, x as i32, text);
    window.attroff(pancurses::A_BOLD);
}

fn create_cube() -> Mesh {
    return Mesh {
        triangles: Vec::from([
            // SOUTH
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            // EAST
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                ],
            },
            //NORTH
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                ],
            },
            // WEST
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            // TOP
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            // BOTTOM
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
            Triangle {
                vertices: vec![
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: -1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                    Vertex {
                        position: Vec3 {
                            x: 1.0,
                            y: -1.0,
                            z: -1.0,
                        },
                    },
                ],
            },
        ]),
    };
}

pub fn run_cube_demo(window: &pancurses::Window) {
    let cube = create_cube();

    let mut t: f32 = 0.0;
    let mut clear = true;

    while t < 10.0 {
        cube
            .rotate_y(t)
            .rotate_z(t)
            .translate_z(2.5)
            .project()
            .draw(window);

        draw_text(window, t);

        // Move cursor out of the way
        unsafe {
            window.mv(crate::SCREEN_HEIGHT as i32, crate::SCREEN_WIDTH as i32);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));

        t += 0.005;

        if t > 5.0 {
            clear = false;
        }

        window.refresh();
        if clear {
            window.erase();
        }
    }
}
