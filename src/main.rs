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

mod open5e;

use open5e::monster::Monster;
use open5e::spells::Spell;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EndPointResults<T> {
    count: isize,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<T>,
}

/*
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut base_url = "https://api.open5e.com/monsters/".to_owned();


    let mut page_num = 1;
    loop {
        println!("Pages scanned: {}", page_num);

        let res = reqwest::get(&base_url).await?;
        let body = res.json::<EndPointResults<Monster>>().await?;

        if let Some(next_page) = body.next  {
            base_url = next_page;
            page_num += 1;
        } else {
            break;
        }
    }

    Ok(())
}
*/

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut base_url = "https://api.open5e.com/spells/".to_owned();

    let mut page_num = 1;
    loop {
        println!("Pages scanned: {}", page_num);

        let res = reqwest::get(&base_url).await?;
        let body = res.json::<EndPointResults<Spell>>().await?;

        if let Some(next_page) = body.next {
            base_url = next_page;
            page_num += 1;
        } else {
            break;
        }
    }

    Ok(())
}
