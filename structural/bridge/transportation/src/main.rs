extern crate transportation;

use transportation::delivery::{ Express, Normal };
use transportation::transport::{ Transport, Plane, Train };

fn main() {
    let mut plane = Plane::new(Box::new(Express{}));
    plane.load();
    plane.carry();
    plane.unload();

    plane.set_delivery(Box::new(Normal{}));
    plane.load();
    plane.carry();
    plane.unload();

    let mut train = Train::new(Box::new(Express{}));
    train.load();
    train.carry();
    train.unload();

    train.set_delivery(Box::new(Normal{}));
    train.load();
    train.carry();
    train.unload();
}
