
use maat_graphics::math;
use maat_graphics::cgmath::{Vector2, Zero, InnerSpace};
use maat_input_handler::MappedKeys;

use crate::modules::controllers::GenericEntityController;
use crate::modules::entity::GenericEntity;

use rand::prelude::ThreadRng;

use maat_graphics::camera::OrthoCamera;

const DASH_COOLDOWN: f32 = 3.0;

pub struct PlayerEntityController {
  // stuff
  dash_cooldown: f32,
}

impl PlayerEntityController {
  pub fn new() -> PlayerEntityController {
    PlayerEntityController {
      dash_cooldown: 0.0,
    }
  }
}

impl GenericEntityController for PlayerEntityController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, keys: &MappedKeys,
            camera: &OrthoCamera, left_mouse: bool, mouse: Vector2<f32>, delta_time: f32) {
    self.dash_cooldown -= delta_time;
    if self.dash_cooldown <= 0.0 {
      self.dash_cooldown = 0.0;
    }
    
    let mut acceleration = Vector2::zero();
    
    if left_mouse {
      entity.fire(rng, delta_time);
    }
    
    if keys.r_pressed() {
      entity.reload();
    }
    
    if keys.w_pressed() {
      acceleration += Vector2::new(0.0, entity.speed());
    }
    if keys.s_pressed() {
      acceleration += Vector2::new(0.0, -entity.speed());
    }
    if keys.d_pressed() {
      acceleration += Vector2::new(entity.speed(), 0.0);
    }
    if keys.a_pressed() {
      acceleration += Vector2::new(-entity.speed(), 0.0);
    }
    
    let look_vector = math::normalise_vector2(entity.position()-camera.get_position() - mouse);
    let rot = look_vector.y.atan2(look_vector.x);
    entity.set_rotation(math::to_degrees(rot)+90.0);
    
    // add dash mechanic
    if self.dash_cooldown <= 0.0 {
      if keys.space_pressed() {
        let crnt_vel = entity.velocity();
        
        let vel_mag = crnt_vel.magnitude();
        
        entity.set_velocity(look_vector*-vel_mag);
        acceleration = look_vector*-vel_mag; // should be based on player speed
        self.dash_cooldown = DASH_COOLDOWN;
      }
    }
    
    // boost
    //let e_velocity = entity.velocity()
    //entity.add_force(e_velocity);
    //entity.apply_physics(delta_time);
    
    entity.add_force(acceleration);
    entity.apply_physics(delta_time);
  }
}
