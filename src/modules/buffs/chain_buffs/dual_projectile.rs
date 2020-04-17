use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};
use crate::modules::entity::bullets::BasicBullet;

use crate::modules::loot::LootRarity;

use maat_graphics::math;
use maat_graphics::cgmath::Vector2;

#[derive(Clone)]
pub struct DualProjectileBuff {
  data: BuffData,
}

impl DualProjectileBuff {
  pub fn new() -> DualProjectileBuff {
    DualProjectileBuff {
      data: BuffData::new(17, 5, LootRarity::Common),
    }
  }
}

impl Buff for DualProjectileBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, _data: &mut StatModifier) {
    
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    // should use 
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    entity.mut_weapon().add_to_active_chain_as_primary(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    bullet.mut_weapon().add_buff(Box::new(self.clone()));
    
    let pos = bullet.position();
    let angle = bullet.rotation();
    
    let perp_dir = Vector2::new(math::to_radians(angle).cos(), math::to_radians(angle).sin());
    let left_pos = pos + perp_dir*-30.0;
    let right_pos = pos + perp_dir*30.0;
    
    bullet.set_position(left_pos);
    
    let life_time = bullet.life_time();
    let friendly = bullet.style().alignment().unwrap().is_friendly();
    
    Some(Box::new(BasicBullet::new(right_pos, life_time, friendly).set_angle(angle)))
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    /*
    bullet.fire(Box::new(self.clone()));
    
    let pos = bullet.position()+Vector2::new(10.0, 10.0);
    let life_time = bullet.life_time();
    let friendly = bullet.style().alignment().unwrap().is_friendly();
    let angle = bullet.rotation();
    Some(Box::new(BasicBullet::new(pos, life_time, friendly).set_angle(angle)))*/
    
    Vec::new() 
  }
}
