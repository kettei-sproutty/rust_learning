#[allow(dead_code)]

#[derive(Debug)]
struct Player {
  name: String,
  x: i32,
  y: i32,
  is_moving: bool,
}

impl Default for Player {
  fn default() -> Self {
    Self {
      name: "Player".to_owned(),
      x: 0,
      y: 0,
      is_moving: false,
    }
  }
}

fn main() {
  let player = Player::default();
  println!("{:?}", player)
}