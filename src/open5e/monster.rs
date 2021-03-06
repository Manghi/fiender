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

use serde::{
    de::{self, Error, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::md::{Markdown, ToMarkdown};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Speeds {
    walk: Option<isize>,
    run: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Skills {
    acrobatics: Option<isize>,
    animal_handling: Option<isize>,
    arcana: Option<isize>,
    athletics: Option<isize>,
    deception: Option<isize>,
    endurance: Option<isize>,
    history: Option<isize>,
    insight: Option<isize>,
    intimidation: Option<isize>,
    investigation: Option<isize>,
    medicine: Option<isize>,
    nature: Option<isize>,
    perception: Option<isize>,
    performanc: Option<isize>,
    persuasion: Option<isize>,
    religion: Option<isize>,
    sleight_of_hand: Option<isize>,
    stealth: Option<isize>,
    streetwise: Option<isize>,
    survival: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Action {
    name: String,
    desc: String,
    attack_bonus: Option<isize>,
    damage_dice: Option<String>,
    damage_bonus: Option<isize>,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name != "" {
            let mut s = String::new();
            s.push_str(format!("1. **{}**\n", self.name).as_str());
            s.push_str(format!("{}\n", self.desc).as_str());

            if let Some(ab) = self.attack_bonus {
                s.push_str(format!("*Attack Bonus:* {}  ", ab).as_str());
            }
            if let Some(ref dd) = self.damage_dice {
                s.push_str(format!("*Damage Dice:* {}  ", dd).as_str());
            }
            if let Some(db) = self.damage_bonus {
                s.push_str(format!("*Damage Bonus:* {}", db).as_str());
            }
            write!(f, "{}\n", s)
        } else {
            write!(f, "")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Monster {
    slug: String,
    name: String,
    size: String,
    #[serde(rename = "type")]
    race: String,
    subtype: String,
    group: Option<String>,
    alignment: String,
    armor_class: isize,
    armor_desc: Option<String>,
    hit_points: isize,
    hit_dice: String,
    speed: Speeds,
    strength: isize,
    dexterity: isize,
    constitution: isize,
    intelligence: isize,
    wisdom: isize,
    charisma: isize,
    strength_save: Option<isize>,
    dexterity_save: Option<isize>,
    constitution_save: Option<isize>,
    intelligence_save: Option<isize>,
    wisdom_save: Option<isize>,
    charisma_save: Option<isize>,
    perception: Option<isize>,
    skills: Skills,
    damage_vulnerabilities: String,
    damage_resistances: String,
    damage_immunities: String,
    condition_immunities: String,
    senses: String,
    languages: String,
    challenge_rating: String,
    #[serde(deserialize_with = "deserialize_string_into_vec_of_strings")]
    actions: Vec<Action>,
    #[serde(deserialize_with = "deserialize_string_into_vec_of_strings")]
    reactions: Vec<Action>,
    legendary_desc: String,
    #[serde(deserialize_with = "deserialize_string_into_vec_of_strings")]
    legendary_actions: Vec<Action>,
    #[serde(deserialize_with = "deserialize_string_into_vec_of_strings")]
    special_abilities: Vec<Action>,
    spell_list: Vec<String>,
    img_main: Option<String>,
    document__slug: String,
    document__title: String,
    document__license_url: String,
}

struct DeserializeStringIntoVecOfStringVisitor;

impl<'de> Visitor<'de> for DeserializeStringIntoVecOfStringVisitor {
    type Value = Vec<Action>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a vector of strings")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(vec![Action {
            name: v,
            desc: "".to_owned(),
            ..Default::default()
        }])
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
    }
}

fn deserialize_string_into_vec_of_strings<'de, D>(deserializer: D) -> Result<Vec<Action>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeStringIntoVecOfStringVisitor)
}

impl ToMarkdown for Monster {
    fn to_md(&self) -> Markdown {
        Markdown(format!(
            "
###Name:  {}
**Race:** {}
- **Armor Class** {}
- **Hit Points** {}
- **Hit Dice** {}

|STR|DEX|CON|INT|WIS|CHA|
|:---:|:---:|:---:|:---:|:---:|:---:|
|{}|{}|{}|{}|{}|{}|
|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|
___
- **Condition Immunities** {}
- **Passive Perception** {:?}
- **Languages** {}
- **Challenge** {}

**Spells:**
{}

**Actions:**
{}
",
            self.name,
            self.race,
            self.armor_class,
            self.hit_points,
            self.hit_dice,
            self.strength,
            self.dexterity,
            self.constitution,
            self.intelligence,
            self.wisdom,
            self.charisma,
            self.strength_save,
            self.dexterity_save,
            self.constitution_save,
            self.intelligence_save,
            self.wisdom_save,
            self.charisma_save,
            self.condition_immunities,
            self.perception,
            self.languages,
            self.challenge_rating,
            self.spell_list.join("\n"),
            {
                let mut sa = String::new();
                self.actions
                    .iter()
                    .for_each(|f| sa.push_str(f.to_string().as_str()));
                sa
            }
        ))
    }
}
