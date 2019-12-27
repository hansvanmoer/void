/*
 * This file is part of Void.
 *
 * Void is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Void is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Void.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

/**
 * All objects and functions related to the game map
 */

use crate::galaxy::{Galaxy, GalaxyBuilder};
use crate::system::{System, SystemBuilder};

use std::rc::Rc;
use std::cell::RefCell;

pub fn create_galaxy() -> Rc<RefCell<Galaxy>>{
    let mut builder = GalaxyBuilder::new();
    for _i in 0..1 {
	builder.add_system(create_system());
    }
    builder.done()
}

fn create_system() -> Rc<RefCell<System>>{
    let mut builder = SystemBuilder::new(0.0, 0.0);
    builder.done()
}
