use game_native::{run, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    run().await;
}
