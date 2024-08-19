use crate::{
    api::{contributions::get_daily_commits, stats::get_stats},
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING},
    env::state::AppState,
    render::{
        contribution_cells::ContributionCells, farm::Farm, grasses::Grasses, home::Home,
        trees::Trees,
    },
    utils::date::{calculate_weeks, get_year_range},
};

pub async fn render_farm_service(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<String, Box<dyn std::error::Error>> {
    let commits = get_daily_commits(&user_name, year).await.unwrap();
    let (start_date, end_date) = get_year_range(year).unwrap();
    let weeks = calculate_weeks(start_date, end_date);

    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    const HEIGHT: u32 = 465;

    let stats = get_stats(
        user_name,
        format!("{}-01-01T00:00:00Z", year),
        format!("{}-12-31T23:59:59Z", year),
        state.token,
    )
    .await;

    if stats.is_err() {
        let error = &stats.err().unwrap()[0];
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error.message.clone(),
        )));
    }

    let stats = stats.unwrap();

    let mut farm = Farm::new(width, HEIGHT);
    // length of key of commits / (365 / 2) * 100, upper bound 100
    let progress = (commits.len() as f32 / 182.5 * 100.0).min(100.0);

    farm.set_progress(progress);
    farm.add_object(ContributionCells::new(year, start_date, weeks, commits));
    farm.add_object(Trees::new(
        user_name,
        width,
        stats
            .contributions_collection
            .total_repositories_with_contributed_commits,
    ));
    farm.add_object(Grasses::new(
        user_name,
        width,
        &stats.contributions_collection,
    ));
    farm.add_object(Home::new(user_name));

    Ok(farm.render())
}
