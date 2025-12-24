use tracing::{error, info};

use crate::{
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING},
    env::state::AppState,
    render::{
        contribution_cells::ContributionCells, farm::Farm, grasses::Grasses, home::Home,
        trees::Trees,
    },
    services::farm::fetcher::{get_data, FetchedData, UserData},
    utils::date::{calculate_day_passed_in_year, calculate_weeks, get_year_range},
};

pub async fn render_farm(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let UserData {
        data: FetchedData {
            commits,
            stats,
            languages,
        },
        is_cached,
    } = get_data(user_name, year, state).await?;
    info!("user = {}, cache = {}", user_name, is_cached);

    let (start_date, end_date) = match get_year_range(year) {
        Some((start_date, end_date)) => (start_date, end_date),
        None => {
            error!("Failed to get year range");
            return Err("Failed to get year range".into());
        }
    };
    let weeks = calculate_weeks(start_date, end_date);

    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    let height = 465;

    let mut farm = Farm::new(width, height);
    let progress =
        (commits.len() as f32 / (calculate_day_passed_in_year(year) / 2.0) * 100.0).min(100.0);

    let home = Home::new(user_name);

    farm.set_progress(progress);
    farm.add_object(ContributionCells::new(
        year, start_date, weeks, commits, languages,
    ));

    if stats.is_ok() {
        let stats = stats.unwrap();

        farm.add_object(Grasses::new(
            user_name,
            width,
            &stats.contributions_collection,
            &home.dead_zone,
        ));
        farm.add_object(Trees::new(
            user_name,
            width,
            stats
                .contributions_collection
                .total_repositories_with_contributed_commits,
            &home.dead_zone,
        ));
    }

    farm.add_object(home);

    Ok(farm.render())
}
