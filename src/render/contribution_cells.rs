use std::{cmp::Ordering, collections::HashMap, fmt::Write};

use chrono::{Datelike, Duration, NaiveDate};

use crate::{
    api::languages::MostUsedLanguage,
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING, GRID_TOP_PADDING, MASK_CLASS},
};

use super::{objects::Objects, renderable::Renderable};

pub struct ContributionCells {
    year: i32,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
    most_used_languages: Vec<MostUsedLanguage>,
}

impl ContributionCells {
    pub fn new(
        year: i32,
        start_date: NaiveDate,
        weeks: usize,
        commits: HashMap<String, u32>,
        most_used_languages: Vec<MostUsedLanguage>,
    ) -> Self {
        Self {
            year,
            start_date,
            weeks,
            commits,
            most_used_languages,
        }
    }
}

impl Renderable for ContributionCells {
    fn render(&self) -> String {
        let mut cells = String::new();

        let most_used_language = self.most_used_languages.iter().max_by(|a, b| {
            a.percentage
                .partial_cmp(&b.percentage)
                .unwrap_or(Ordering::Equal)
        });

        for week in 0..self.weeks {
            let x_coord = GRID_LEFT_PADDING + week as u32 * (CELL_SIZE + CELL_SPACING);

            for day in 0..7 {
                let current_date = self.start_date + Duration::days((week * 7 + day) as i64);

                if current_date.year() != self.year {
                    continue;
                }

                let y_coord = GRID_TOP_PADDING + day as u32 * (CELL_SIZE + CELL_SPACING);
                let formatted_date = current_date.format("%Y-%m-%d").to_string();
                let commit_level = self.commits.get(&formatted_date).unwrap_or(&0);

                write!(
                    cells,
                    r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                    x_coord,
                    y_coord,
                    Objects::Dirt.to_string()
                )
                .unwrap();

                if *commit_level > 0 {
                    let flower = match *commit_level {
                        1 => Objects::FlowerOne,
                        2 => Objects::FlowerTwo,
                        3 => Objects::FlowerThree,
                        _ => Objects::FlowerFour,
                    };

                    write!(
                        cells,
                        r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                        x_coord,
                        y_coord,
                        flower.to_string()
                    )
                    .unwrap();

                    if most_used_language.is_some() && flower.get_mask_id().is_some() {
                        write!(
                            cells,
                            r##"<rect mask="url(#{})" x="{}" y="{}" width="{}" height="{}" fill="{}" class="{}" />"##,
                            flower.get_mask_id().unwrap(),
                            x_coord,
                            y_coord,
                            CELL_SIZE,
                            CELL_SIZE,
                            most_used_language.unwrap().color,
                            MASK_CLASS
                        ).unwrap();
                    }
                }
            }
        }

        cells
    }
}
