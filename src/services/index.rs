use crate::api::contributions::get_daily_commits;

pub async fn index_service(user_name: String, year: i32) -> String {
    let response = get_daily_commits(&user_name, year).await.unwrap();
    let svg = r##"
    <svg
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        viewBox="0 0 930 465"
        fill="none"
    >
    </svg>
    "##;

    println!("{:?}", response);

    svg.to_string()
}
