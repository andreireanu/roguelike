use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component, Clone)]
 
pub struct Attributes{
     laziness: i32,
     hubris: i32, 
}

pub fn register_components(world : &mut World ){
     world.register::<Attributes>();
}