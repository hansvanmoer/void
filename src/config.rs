/**
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

use clap::{App, Arg};
use log::Level;

/*
 * Struct holding all application configuration
 */
pub struct Config{
    verbosity: Level
}

impl Config{

    /*
     * Creates a new configuration object
     */
    pub fn new() -> Config{
	Config{verbosity: Level::Info}
    }

    pub fn load(& mut self){
	let matches = App::new("void")
	    .about("a space game")
	    .arg(
		Arg::with_name("verbosity")
		    .help("sets application verbosity. pass 'DEBUG', 'INFO' (the default), 'WARNING' or 'ERROR'")
		    .short("v")
		    .default_value("INFO")
	    )
	    .get_matches();
	self.verbosity = match matches.value_of("verbosity").unwrap() {
	    "DEBUG" | "debug" => Level::Debug,
	    "INFO" | "info" => Level::Info,
	    "WARNING" | "warning" => Level::Warn,
	    "ERROR" | "error" => Level::Error,
	    _ => Level::Info
	};
    }

    pub fn verbosity(& self) -> Level{
	self.verbosity
    }
}
