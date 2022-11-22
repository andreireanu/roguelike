use specs::prelude::*; 
use bracket_lib::prelude::*; 

pub struct Map{
    rooms: Vec<Rect>,
} 

const MIN_WIDTH: i32 = 2;
const MAX_WIDTH: i32 = 9;
const MIN_HEIGHT: i32 = 2;
const MAX_HEIGHT: i32 = 7;

impl Map {
    pub fn generate (ecs: &mut World, width : i32, height: i32) -> Self {
        let mut rooms = Vec::new();
        let rng = ecs.get_mut::<RandomNumberGenerator>().unwrap();
        while rooms.len() < 5 {
              let x1 = rng.range(0, width - MAX_WIDTH);
              let y1 = rng.range(0, height - MAX_HEIGHT );
              let w = MIN_WIDTH + rng.range(0, MAX_WIDTH - MIN_WIDTH);
              let h = MIN_HEIGHT  + rng.range(0, MAX_HEIGHT - MIN_HEIGHT);
              let new_room = Rect::with_size(x1, y1, w, h);
              let mut intersect = false;
              for r in &rooms {
                if new_room.intersect(&r){
                    intersect = true;
                };
              }
              rooms.push(new_room);
        }  
        Self { rooms}
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        for room in &self.rooms {
            for col in room.x1..room.x2 {
                for row in room.y1..room.y2 {
                    ctx.print(room.x1, row, &format!("{}", ".".repeat((room.x2 - room.x1) as usize)));
                }
            }
        }
    }
}