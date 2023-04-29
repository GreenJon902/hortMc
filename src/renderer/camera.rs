use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;

use gl::types::{GLsizeiptr, GLuint};
use std140::{mat3x3, vec2, vec3};

use crate::renderer::sgl;

fn make_yaw_pitch_roll_matrix(yaw: f32, pitch: f32, roll: f32) -> mat3x3 {
    let yaw = yaw.to_radians();
    let pitch = pitch.to_radians();
    let roll = roll.to_radians();

    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();
    let cos_pitch = pitch.cos();
    let sin_pitch = pitch.sin();
    let cos_roll = roll.cos();
    let sin_roll = roll.sin();

    let rotation_matrix = mat3x3(
        vec3(cos_yaw * cos_roll + sin_yaw * sin_pitch * sin_roll,
             sin_roll * cos_pitch,
             -sin_yaw * cos_roll + cos_yaw * sin_pitch * sin_roll),
        vec3(-cos_yaw * sin_roll + sin_yaw * sin_pitch * cos_roll,
             cos_roll * cos_pitch,
             sin_yaw * sin_roll + cos_yaw * sin_pitch * cos_roll
        ),
        vec3(
            sin_yaw * cos_pitch,
            -sin_pitch,
            cos_yaw * cos_pitch
        )
    );

    return rotation_matrix;
}

#[std140::repr_std140]
#[derive(Debug)]
struct CameraBuffer { // All values are duplicates of Camera
pos: vec3,
    rot: mat3x3,
    fov: vec2
}

#[derive(Debug)]
pub struct Camera {
    pos: vec3, // Y is up
    pitch: f32,
    yaw: f32,  // 0,0,0 Would be looking towards positive Z
    roll: f32,
    fov: vec2,

    buffer_id: GLuint,
    buffer: CameraBuffer
}

impl Camera {
    pub fn new(pos: vec3, pitch: f32, yaw: f32, roll: f32, fov: vec2) -> Camera {
        let mut buffer_id: GLuint = 0;

        sgl::GenBuffers(1, &mut buffer_id);
        sgl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id);
        sgl::BufferData(gl::UNIFORM_BUFFER, size_of::<CameraBuffer>() as GLsizeiptr, ptr::null(),
                       gl::DYNAMIC_DRAW);
        sgl::BindBufferBase(gl::UNIFORM_BUFFER, 0, buffer_id);

        let camera = Camera { pos, pitch, yaw, roll, fov, buffer_id, buffer: CameraBuffer {
            pos, rot: make_yaw_pitch_roll_matrix(yaw, pitch, roll), fov}};

        return camera;
    }

    pub fn look_rel(&mut self, rel_yaw: f32, rel_pitch: f32, rel_roll: f32) {
        println!("rot {} {} {}", self.yaw, self.pitch, self.roll);
        self.yaw += rel_yaw;
        self.pitch += rel_pitch;
        self.roll += rel_roll;
    }

    pub fn move_rel(&mut self, rel_x: f32, rel_y: f32, rel_z: f32) {
        println!("pos {} {} {}", self.pos.0, self.pos.1, self.pos.2);
        self.pos.0 += rel_x;
        self.pos.1 += rel_y;
        self.pos.2 += rel_z;
    }

    pub fn update(&mut self) {  // Update data on the gpu
        self.buffer.pos = self.pos;
        self.buffer.fov = self.fov;
        self.buffer.rot = make_yaw_pitch_roll_matrix(self.yaw, self.pitch, self.roll);
        sgl::BindBuffer(gl::UNIFORM_BUFFER, self.buffer_id);
        sgl::BufferSubData(gl::UNIFORM_BUFFER, 0, size_of::<CameraBuffer>() as GLsizeiptr,
                           ptr::addr_of!(self.buffer) as *const CameraBuffer as *const c_void);
    }
}

impl Default for Camera {
    fn default() -> Camera {
        return Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 0.0, vec2(90.0, 90.0))
    }
}