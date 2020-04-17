
use crate::modules::objects::{ObjectData, GenericObject};
use crate::modules::buffs::Buff;

use maat_graphics::cgmath::Vector2;

use std::f32::consts::PI;

pub struct Loot {
  data: ObjectData,
  buff: Box<dyn Buff>,
  blueprint: bool,
  wave_time: f32,
  base_size: Vector2<f32>,
}

impl Loot {
  pub fn new(pos: Vector2<f32>, buff: Box<dyn Buff>) -> Loot {
    let (texture, idx, rows) = buff.sprite_details();
    Loot {
      data: ObjectData::new_spritesheet(pos, Vector2::new(48.0, 48.0), texture, idx, rows),
      buff,
      blueprint: false,
      wave_time: 0.0,
      base_size: Vector2::new(48.0, 48.0),
    }
  }
  
  pub fn get_buff(&self) -> &Box<dyn Buff> {
    &self.buff
  }
  
  pub fn is_blueprint(&self) -> bool {
    self.blueprint
  }
}

impl GenericObject for Loot {
  fn o_data(&self) -> &ObjectData {
    &self.data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn animation_update(&mut self, delta_time: f32) {
    self.wave_time += delta_time*2.0;
    if self.wave_time > 2.0*PI {
      self.wave_time -= 2.0*PI;
    }
    
    let sin = self.wave_time.sin();
    self.set_rotation(15.0*sin);
    
    self.set_size(self.base_size + self.base_size*(sin*0.2+0.05));
  }
}

