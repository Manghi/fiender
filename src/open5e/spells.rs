/*
 * This file is part of Fiender.
 *
 * Copyright (C) 2020 The Fiender Developers
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of  MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Spell {
    slug: String,
    name: String,
    desc: String,
    higher_level: String,
    page: String,
    range: String,
    components: String,
    material: String,
    ritual: String,
    duration: String,
    concentration: String,
    casting_time: String,
    level: String,
    level_int: isize,
    school: String,
    dnd_class: String,
    archetype: String,
    circles: String,
    document__slug: String,
    document__title: String,
    document__license_url: String,
}
