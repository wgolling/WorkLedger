mod controllers;
mod models;
mod views;
mod engine;
mod parser;
mod enums;

use engine::Engine;

pub fn run() {
    let mut e = Engine::new();
    e.run();
}