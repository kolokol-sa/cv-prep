// A Track has a title (String), an artist (String),
// and a length in seconds (u32).
//
// A Playlist has a name (String) and a Vec<Track>.
//
// Give Track:
//   - a constructor taking title, artist, seconds
//   - a method returning its length in minutes as f64
//
// Give Playlist:
//   - a constructor taking a name, starting with no tracks
//   - a method that adds a track
//   - a method returning the total length in seconds
//   - a method returning the longest track
//   - a method returning all tracks by a given artist
//
// Decide the signatures yourself: what each method takes
// and what it returns.
//
// In main: build a playlist, add four or five tracks,
// print the total length, the longest track, and everything
// by one artist.
//
// "Redbone" — Childish Gambino — 326
// "Nikes" — Frank Ocean — 314
// "Sweet Life" — Frank Ocean — 262
// "Alright" — Kendrick Lamar — 219
// "Los Ageless" — St. Vincent — 245

struct Track {
    title: String,
    artist: String,
    length: u32,
}

impl Track {
    fn new(title: &str, artist: &str, sec: u32) -> Track {
        Track {
            title: String::from(title),
            artist: String::from(artist),
            length: sec,
        }
    }

    fn len_min(&self) -> f64 {
        (self.length as f64) / 60.0
    }
}

struct Playlist {
    name: String,
    tracklist: Vec<Track>,
}

impl Playlist {
    fn new(name: &str) -> Playlist {
        Playlist {
            name: String::from(name),
            tracklist: vec![],
        }
    }

    fn add_track(&mut self, track: Track) {
        self.tracklist.push(track);
    }

    fn total_len(&self) -> u32 {
        let mut total = 0;
        for track in self.tracklist.iter() {
            total += track.length;
        }
        total
    }

    fn longest_track(&self) -> &Track {
        let mut max_len = 0;
        let mut longest_track = &self.tracklist[0];
        for track in self.tracklist.iter() {
            if track.length > max_len {
                max_len = track.length;
                longest_track = track;
            }
        }
        longest_track
    }

    fn tracks_by(&self, artist: &str) -> Vec<&Track> {
        self.tracklist
            .iter()
            .filter(|t| t.artist == artist)
            .collect()
    }
}

const SELECTED_ARTIST: &str = "Frank Ocean";

fn main() {
    // dummy dataset
    let raw = [
        ("Redbone", "Childish Gambino", 326),
        ("Nikes", "Frank Ocean", 314),
        ("Sweet Life", "Frank Ocean", 262),
        ("Alright", "Kendrick Lamar", 219),
        ("Los Ageless", "St. Vincent", 245),
    ];

    // initializing a playlist and populating it with tracks
    let mut playlist1 = Playlist::new("My First Tracks");
    for track in raw.iter() {
        playlist1.add_track(Track::new(track.0, track.1, track.2));
    }

    // printing the playlist
    println!("Playlist \"{}\" - {} tracks:", &playlist1.name, playlist1.tracklist.len());
    for track in playlist1.tracklist.iter() {
        println!(
            "\"{}\" by {} - {} s",
            track.title, track.artist, track.length,
        );
    }

    // printing total length
    println!(
        "\nTotal length of \"{}\" is {} seconds",
        playlist1.name,
        playlist1.total_len()
    );

    // printing the longest track
    let longest = playlist1.longest_track();
    println!(
        "\nThe longest track is \"{}\" by {} - {} seconds ({:.1} minutes)",
        longest.title,
        longest.artist,
        longest.length,
        longest.len_min()
    );

    // printing all tracks by SELECTED_ARTIST
    println!("\nHere are all tracks by {}:", SELECTED_ARTIST);
    for track in playlist1.tracks_by(SELECTED_ARTIST) {
        println!("\"{}\" - {} s", track.title, track.length)
    }
}
