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

use crate::vector::Vector;

use std::cell::RefCell;
use std::rc::Rc;

/**
 * Models a star system
 */
#[derive(Debug)]
pub struct System{
    pos: Vector
}

impl System{

    /**
     * Creates a star system
     */
    pub fn new(pos: Vector) -> System{
	System{pos}
    }
    
}

pub struct SystemBuilder{
    system: Rc<RefCell<System>>
}

impl SystemBuilder{

    /**
     * Creates a new systemat the supplied position
     */
    pub fn new(x: f64, y: f64) -> SystemBuilder{
	SystemBuilder{system: Rc::from(RefCell::from(System::new(Vector{x, y})))}
    }

    /**
     * Finalizes the system and returns it
     */
    pub fn done(& mut self) -> Rc<RefCell<System>>{
	self.system.clone().clone()
    }
    
}
