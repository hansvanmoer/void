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

mod config;

use config::Config;

use log::info;
/*
 * Application entry point
 */
fn main() {
    let mut config = Config::new();
    config.load();
    simple_logger::init_with_level(config.verbosity()).unwrap();
    info!("application started");
}

