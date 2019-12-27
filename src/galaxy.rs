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

use crate::system::System;

use std::rc::Rc;
use std::cell::RefCell;

/**
 * Models a galaxy
 */
#[derive(Debug)]
pub struct Galaxy{
    systems: Vec<Rc<RefCell<System>>>
}

impl Galaxy{

    fn new() -> Galaxy{
	Galaxy{systems: Vec::new()}
    }
    
}

/**
 * A builder for a galaxy
 */
pub struct GalaxyBuilder{
    galaxy: Rc<RefCell<Galaxy>>
}

impl GalaxyBuilder{

    /**
     * Creates a new galaxy builder
     */
    pub fn new() -> GalaxyBuilder{
	GalaxyBuilder{galaxy: Rc::from(RefCell::from(Galaxy::new()))}
    }

    /**
     * Adds a system
     */
    pub fn add_system(& mut self, system: Rc<RefCell<System>>){
	self.galaxy.borrow_mut().systems.push(system);
    }

    /**
     * Finalizes the current galaxy and returns it
     */
    pub fn done(& mut self) -> Rc<RefCell<Galaxy>>{
	self.galaxy.clone()
    }
}
