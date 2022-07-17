trait Movable {
  fn mobe_by(&mut self, x: i32, y: i32) -> ();
}

struct Player {
  name: String,
  x: i32,
  y: i32,
}

impl Movable for Player {
  fn mobe_by(&mut self, x: i32, y: i32) {
    println!("{} player is moving by {x} in x axys and {y} in y axis", self.name);
    self.x += x;
    self.y += y;
    println!("{} player current position is: ({}, {})", self.name, self.x, self.y);
  }
}

struct Object {
  material: String,
  x: i32,
  y: i32,
}

impl Movable for Object {
  fn mobe_by(&mut self, x: i32, y: i32) {
    println!("{} is moving by {x} on x axys and {y} on y axis", self.material);
    self.x += x;
    self.y += y;
    println!("{} player current position is: ({}, {})", self.material, self.x, self.y);
  }
}

fn move_entity(entity: &mut impl Movable) {
  entity.mobe_by(1, 2);
}

fn main() {
  let mut player = Player { name: "Test".to_owned(), x: 0, y: 0 };
  move_entity(&mut player);
  player.mobe_by(10, 20);

  let mut object = Object { material: "Metal".to_owned(), x: 0, y: 0 };
  move_entity(&mut object);
}