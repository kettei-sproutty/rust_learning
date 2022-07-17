#[allow(dead_code)]

trait Movable {
  fn move_by(&self, x: i32, y: i32) -> ();
}

struct Player {
  name: String,
  x: i32,
  y: i32
}

impl Movable for Player {
  fn move_by(&self, x: i32, y: i32) -> () {
    println!("{} player is moving by ({x}x, {y}y)", self.name)
  }
}

fn move_entity<T: Movable>(entity: &T, x: i32, y: i32) {
  entity.move_by(x, y);
}

fn move_entity_where<T>(entity: &T, x: i32, y: i32)
  where
    T: Movable
{
  entity.move_by(x, y);
}

fn main() {
  let player = Player { name: "Player".to_owned(), x: 0, y: 0 };
  move_entity(&player, 1, 2);
  move_entity_where(&player, 2, 1);
}