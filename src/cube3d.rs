// Not all matrix transformation functions are used in this demo
#![allow(dead_code)]

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

#[derive(Clone)]
struct Triangle {
    vertices: Vec<Vertex>,
}

struct Mesh {
    triangles: Vec<Triangle>,
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
            error = error + dy;
            vec0_rounded.x = vec0_rounded.x + sx;
        }

        if e2 <= dx {
            if vec0_rounded.y == vec1_rounded.y {
                break;
            }
            error = error + dx;
            vec0_rounded.y = vec0_rounded.y + sy;
        }
    }
}

fn multiply_matrix4x4_vec3(matrix: &Matrix4x4, vec: &Vec3) -> Vec3 {
    let mut return_vector = Vec3 {
        x: vec.x * matrix.data[0][0]
            + vec.y * matrix.data[1][0]
            + vec.z * matrix.data[2][0]
            + matrix.data[3][0],
        y: vec.x * matrix.data[0][1]
            + vec.y * matrix.data[1][1]
            + vec.z * matrix.data[2][1]
            + matrix.data[3][1],
        z: vec.x * matrix.data[0][2]
            + vec.y * matrix.data[1][2]
            + vec.z * matrix.data[2][2]
            + matrix.data[3][2],
    };

    let w = vec.x * matrix.data[0][3]
        + matrix.data[1][3]
        + vec.z * matrix.data[2][3]
        + matrix.data[3][3];

    if w != 0.0 {
        return_vector.x /= w;
        return_vector.y /= w;
        return_vector.z /= w;
    }

    return_vector
}

fn project_vec3(vec: &Vec3) -> Vec3 {
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

    multiply_matrix4x4_vec3(&proj_matrix, vec)
}

fn rotate_vec3_x(vec: &Vec3, theta: f32) -> Vec3 {
    let rotate_x_matrix = Matrix4x4 {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f32::cos(theta), f32::sin(theta), 0.0],
            [0.0, -f32::sin(theta), f32::cos(theta), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    multiply_matrix4x4_vec3(&rotate_x_matrix, &vec)
}

fn rotate_vec3_y(vec: &Vec3, theta: f32) -> Vec3 {
    let rotate_z_matrix = Matrix4x4 {
        data: [
            [f32::cos(theta), 0.0, f32::sin(theta), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-f32::sin(theta), 0.0, f32::cos(theta), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    multiply_matrix4x4_vec3(&rotate_z_matrix, &vec)
}

fn rotate_vec3_z(vec: &Vec3, theta: f32) -> Vec3 {
    let rotate_z_matrix = Matrix4x4 {
        data: [
            [f32::cos(theta), f32::sin(theta), 0.0, 0.0],
            [-f32::sin(theta), f32::cos(theta), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    multiply_matrix4x4_vec3(&rotate_z_matrix, &vec)
}

fn translate_vec3_x(vec: &Vec3, x: f32) -> Vec3 {
    Vec3 {
        x: vec.x + x,
        y: vec.y,
        z: vec.z,
    }
}

fn translate_vec3_y(vec: &Vec3, y: f32) -> Vec3 {
    Vec3 {
        x: vec.x,
        y: vec.y + y,
        z: vec.z,
    }
}

fn translate_vec3_z(vec: &Vec3, z: f32) -> Vec3 {
    Vec3 {
        x: vec.x,
        y: vec.y,
        z: vec.z + z,
    }
}

fn project_triangle(triangle: &Triangle) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: project_vec3(&vertex.position),
                }
            })
            .collect(),
    }
}

fn rotate_triangle_x(triangle: &Triangle, theta: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: rotate_vec3_x(&vertex.position, theta),
                }
            })
            .collect(),
    }
}

fn rotate_triangle_y(triangle: &Triangle, theta: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: rotate_vec3_y(&vertex.position, theta),
                }
            })
            .collect(),
    }
}

fn rotate_triangle_z(triangle: &Triangle, theta: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: rotate_vec3_z(&vertex.position, theta),
                }
            })
            .collect(),
    }
}

fn translate_triangle_x(triangle: &Triangle, x: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: translate_vec3_x(&vertex.position, x),
                }
            })
            .collect(),
    }
}

fn translate_triangle_y(triangle: &Triangle, y: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: translate_vec3_y(&vertex.position, y),
                }
            })
            .collect(),
    }
}

fn translate_triangle_z(triangle: &Triangle, z: f32) -> Triangle {
    Triangle {
        vertices: triangle
            .vertices
            .iter()
            .map(|vertex| -> Vertex {
                Vertex {
                    position: translate_vec3_z(&vertex.position, z),
                }
            })
            .collect(),
    }
}

fn draw_triangle(window: &pancurses::Window, triangle: &Triangle) {
    // Scale the vertexs x and y coordinates into the screen dimensions
    let scaled_vertices: Vec<Vec2> = triangle
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

fn project_mesh(mesh: &Mesh) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { project_triangle(&triangle) })
            .collect(),
    }
}

fn rotate_mesh_x(mesh: &Mesh, theta: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { rotate_triangle_x(&triangle, theta) })
            .collect(),
    }
}

fn rotate_mesh_y(mesh: &Mesh, theta: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { rotate_triangle_y(&triangle, theta) })
            .collect(),
    }
}

fn rotate_mesh_z(mesh: &Mesh, theta: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { rotate_triangle_z(&triangle, theta) })
            .collect(),
    }
}

fn translate_mesh_x(mesh: &Mesh, x: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { translate_triangle_x(&triangle, x) })
            .collect(),
    }
}

fn translate_mesh_y(mesh: &Mesh, y: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { translate_triangle_y(&triangle, y) })
            .collect(),
    }
}

fn translate_mesh_z(mesh: &Mesh, z: f32) -> Mesh {
    Mesh {
        triangles: mesh
            .triangles
            .iter()
            .map(|triangle| -> Triangle { translate_triangle_z(&triangle, z) })
            .collect(),
    }
}

fn draw_mesh(window: &pancurses::Window, mesh: &Mesh) {
    mesh.triangles.iter().for_each(|triangle| {
        draw_triangle(window, &triangle);
    });
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

pub fn run_cube_demo(window: &pancurses::Window) {
    let cube = Mesh {
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

    let mut t: f32 = 0.0;
    let mut clear = true;

    while t < 10.0 {
        let rot_mesh_y = rotate_mesh_y(&cube, t);
        let rot_mesh_z = rotate_mesh_z(&rot_mesh_y, t);
        let trans_mesh_z = translate_mesh_z(&rot_mesh_z, 2.5);
        let proj_mesh = project_mesh(&trans_mesh_z);

        draw_mesh(&window, &proj_mesh);
        draw_text(&window, t);

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
