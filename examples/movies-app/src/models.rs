use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Page<T: Clone> {
    pub page: u32,
    pub results: im::Vector<T>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MovieDetails {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    // pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_count: u64,
    pub vote_average: f64,
    pub popularity: f64,
    pub budget: u64,
    // pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
    pub production_companies: Vec<ProdCompany>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProdCompany {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
    // pub crew: Vec<Crew>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Crew {
    pub credit_id: String,
    pub department: String,
    pub gender: Option<u8>,
    pub id: u64,
    pub job: String,
    pub name: String,
    pub profile_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CastMember {
    pub id: u64,
    pub cast_id: u64,
    // pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u8,
}

impl From<MovieDetails> for Movie {
    fn from(movie_details: MovieDetails) -> Self {
        Movie {
            id: movie_details.id,
            title: movie_details.title,
            overview: movie_details.overview,
            release_date: movie_details.release_date,
            homepage: movie_details.homepage,
            poster_path: movie_details.poster_path,
            backdrop_path: movie_details.backdrop_path,
            popularity: movie_details.popularity,
            vote_count: movie_details.vote_count,
            vote_average: movie_details.vote_average,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Movie {
    pub id: u64,
    // pub imdb_id: String,
    pub title: String,
    // pub tagline: String,
    pub overview: Option<String>,
    //TODO: chrono-rs
    pub release_date: String,
    // pub runtime: u32,
    pub homepage: Option<String>,
    // pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    // pub budget: u64,
    // pub videos: Option<Results<Video>>,
    // pub credits: Option<Credits>,
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Movie {}

impl Hash for Movie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TvShow {
    pub id: u64,
    // pub imdb_id: String,
    pub name: String,
    // pub tagline: String,
    // pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    // pub release_date: String,
    // pub runtime: u32,
    pub homepage: Option<String>,
    // pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    // pub budget: u64,
    // pub videos: Option<Results<Video>>,
    // pub credits: Option<Credits>,
}

impl PartialEq for TvShow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TvShow {}

impl Hash for TvShow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

//
// #[derive(Debug, PartialEq, Deserialize, Serialize)]
// pub struct Episode {
//     pub air_date: String,
//     pub episode_number: u32,
//     pub id: u64,
//     pub name: String,
//     pub overview: String,
//     pub production_code: Option<String>,
//     pub season_number: u32,
//     pub still_path: Option<String>,
//     pub vote_average: f64,
//     pub vote_count: u64,
// }
//
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TvShowDetails {
    pub id: u64,
    pub backdrop_path: Option<String>,
    // pub created_by: Vec<TVCreator>,
    // pub episode_run_time: Vec<u64>,
    // pub first_air_date: String,
    // pub genres: Vec<Genre>,
    // pub homepage: Option<String>,
    // pub in_production: bool,
    // pub languages: Vec<String>,
    // pub last_air_date: String,
    // pub last_episode_to_air: Option<Episode>,
    // pub name: String,
    // pub networks: Vec<Network>,
    // pub number_of_episodes: u32,
    // pub number_of_seasons: u32,
    // pub origin_country: Vec<String>,
    // pub original_language: String,
    // pub original_name: String,
    // pub overview: String,
    // pub popularity: f64,
    // pub poster_path: Option<String>,
    // pub production_companies: Vec<ProductionCompany>,
    // pub seasons: Vec<Season>,
    // pub status: String,
    // #[serde(rename = "type")]
    // pub show_type: String,
    // pub vote_average: f64,
    // pub vote_count: u64,
    // // pub videos: Option<Results<Video>>,
    // pub credits: Option<TVCredits>,
}
