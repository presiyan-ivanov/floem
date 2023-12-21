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
    CurrentlyAiring,
    AiringToday,
    Trending,
    Popular,
    TopRated,
    // Upcoming
}

impl DataProvider {

    fn get_movies(&self) {

    }

    fn get_tv_shows(&self) {

    }
}
