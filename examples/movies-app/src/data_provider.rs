enum DataProvider {
    LocalSnapshot,
    Tmdb
}

enum MovieFilter {
    Trending,
    Popular,
    TopRated,
    NowPlaying,
    // Upcoming
}

enum TvShowFilter {
    Trending,
    Popular,
    TopRated,
    CurrentlyAiring,
    AiringToday
    // Upcoming
}

impl DataProvider {

    fn get_movies(&self) {

    }

    fn get_tv_shows(&self) {

    }
}
