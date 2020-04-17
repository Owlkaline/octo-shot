
use maat_graphics::cgmath::{Vector2, Zero};

use crate::modules::objects::{GenericObject, ObjectData};
use crate::modules::entity::{GenericEntity, EntityData};
use crate::modules::loot::{Loot, LootTable, LootTableData};

const DEFAULT_BASIC_BULLET_SPEED: f32 = 1200.0*2.0;

use rand::prelude::ThreadRng;

pub struct BasicBullet {
  o_data: ObjectData,
  e_data: EntityData,
  l_data: LootTableData,
  animation_tick: f32,
}

impl BasicBullet {
  pub fn new(pos: Vector2<f32>, life_time: f32, friendly: bool) -> BasicBullet {
    
    BasicBullet {
      o_data: ObjectData::new_spritesheet(pos, Vector2::new(36.0, 36.0), "basic_bullet_spritesheet".to_string(), 0, 3),
      e_data: EntityData::new_for_bullet().set_bullet_alignment(friendly)
                                          .set_base_speed(DEFAULT_BASIC_BULLET_SPEED)
                                          .set_base_hit_points(20)
                                          .set_base_life_time(life_time)
                                          .set_base_damage(1)
                                          .finish(), // damage per hitpoint
      l_data: LootTableData::new(),
      animation_tick: 0.0,
    }
  }
  
  pub fn set_angle(mut self, angle: f32) -> BasicBullet {
    self.set_rotation(angle);
    self
  }
}


impl GenericObject for BasicBullet {
  fn o_data(&self) -> &ObjectData {
    &self.o_data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.o_data
  }
  
  fn animation_update(&mut self, delta_time: f32) {
    self.animation_tick += delta_time;
    if self.animation_tick > 0.1 {
      self.animation_tick -= 0.1;
      self.set_sprite_idx((self.sprite_idx() + 1)%(self.sprite_rows()*self.sprite_rows()));
      //self.sprite_idx();
      //self.o_mut_data().sprite_idx = (self.o_data().sprite_idx + 1)%(self.o_data().sprite_rows*self.o_data().sprite_rows);
    }
  }
}

impl LootTable for BasicBullet {
  fn l_data(&self) -> &LootTableData {
    &self.l_data
  }
  
  fn l_mut_data(&mut self) -> &mut LootTableData {
    &mut self.l_data
  }
  
  fn drop_loot(&self, _rng: &mut ThreadRng) -> Vec<Loot> {
    Vec::new()
  }
}

impl GenericEntity for BasicBullet {
  fn e_data(&self) -> &EntityData {
    &self.e_data
  }
  
  fn e_mut_data(&mut self) -> &mut EntityData {
    &mut self.e_data
  }
  
  fn update(&mut self, delta_time: f32) {
    self.animation_update(delta_time);
  }
  
  fn bullet_spawn_locations(&self) -> Vector2<f32> {
    Vector2::zero()
  }
}

