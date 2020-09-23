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
extern crate clap;

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

mod md;
mod open5e;

use md::ToMarkdown;

use open5e::monster::Monster;
use open5e::spells::Spell;
use open5e::SearchType;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EndPointResults<T> {
    count: isize,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<T>,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("monster")
                .short("m")
                .long("monster")
                .conflicts_with("spell")
                .help("Search for a monster by name"),
        )
        .arg(
            Arg::with_name("spell")
                .short("s")
                .long("spell")
                .conflicts_with("monster")
                .help("Search for a spell by name"),
        )
        .arg(
            Arg::with_name("name")
                .help("The name of a spell or monster")
                .required(true),
        )
        .get_matches();

    let mut base_url: String = "https://api.open5e.com/".to_owned();
    let mut search_type = SearchType::Monster;

    if matches.is_present("spell") {
        search_type = SearchType::Spell;
        base_url.push_str("spells/");
    } else {
        // default to monster
        base_url.push_str("monsters/");
    }

    // unwrap safe b/c the name argument is required per above
    let search_arg = matches
        .value_of("name")
        .unwrap()
        .to_lowercase()
        .replace(" ", "-");

    base_url.push_str(&search_arg);
    dbg!(&base_url);

    let res = reqwest::get(&base_url).await?;
    res.error_for_status_ref()?;

    let json_data = match search_type {
        SearchType::Monster => res.json::<Monster>().await?.to_md(),
        SearchType::Spell => res.json::<Spell>().await?.to_md(),
    };

    println!("{}", json_data);

    Ok(())
}
