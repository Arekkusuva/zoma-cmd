use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

trait Provider {
    type Item;
    type Error;

    fn new() -> Self;
    fn run(chan: Sender<Self::Item>) -> Self::Error;
}
