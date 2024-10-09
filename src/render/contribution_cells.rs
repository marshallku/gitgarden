use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate};

use crate::constants::render::{
    CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING, GRID_TOP_PADDING, MASK_CLASS,
};

use super::{objects::Objects, renderable::Renderable};

pub struct ContributionCells {
    year: i32,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
}

impl ContributionCells {
    pub fn new(
        year: i32,
        start_date: NaiveDate,
        weeks: usize,
        commits: HashMap<String, u32>,
    ) -> Self {
        Self {
            year,
            start_date,
            weeks,
            commits,
        }
    }
}

impl Renderable for ContributionCells {
    fn render(&self) -> String {
        let mut cells = String::new();

        for week in 0..self.weeks {
            for day in 0..7 {
                let current_date = self.start_date + Duration::days((week * 7 + day) as i64);

                if current_date.year() != self.year {
                    continue;
                }

                let formatted_date = current_date.format("%Y-%m-%d").to_string();

                let x = GRID_LEFT_PADDING + week as u32 * (CELL_SIZE + CELL_SPACING);
                let y = GRID_TOP_PADDING + day as u32 * (CELL_SIZE + CELL_SPACING);

                let commit_level = self.commits.get(&formatted_date).unwrap_or(&0);

                cells.push_str(&format!(
                    r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                    x,
                    y,
                    Objects::Dirt.to_string()
                ));

                if *commit_level > 0 {
                    let flower = match *commit_level {
                        1 => Objects::FlowerOne,
                        2 => Objects::FlowerTwo,
                        3 => Objects::FlowerThree,
                        _ => Objects::FlowerFour,
                    };

                    cells.push_str(&format!(
                        r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                        x,
                        y,
                        flower.to_string()
                    ));

                    if flower.get_mask_id().is_some() {
                        cells.push_str(&format!(
                            r##"<rect mask="url(#{})" x="{}" y="{}" width="{}" height="{}" fill="#132345" class="{}" />"##,
                            flower.get_mask_id().unwrap(),
                            x,
                            y,
                            CELL_SIZE,
                            CELL_SIZE,
                            MASK_CLASS
                        ));
                    }
                }
            }
        }

        cells
    }
}
