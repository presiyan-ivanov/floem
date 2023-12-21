#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    //TODO: chrono-rs
    pub release_date: String,
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
    pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    pub air_date: String,
    pub episode_number: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub season_number: u32,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TvShow {
    pub id: u64,
    pub backdrop_path: Option<String>,
    pub created_by: Vec<TVCreator>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub last_episode_to_air: Option<Episode>,
    pub name: String,
    pub networks: Vec<Network>,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub seasons: Vec<Season>,
    pub status: String,
    #[serde(rename = "type")]
    pub show_type: String,
    pub vote_average: f64,
    pub vote_count: u64,
    // pub videos: Option<Results<Video>>,
    pub credits: Option<TVCredits>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCompany {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}
