
use std::error::Error;
use std::path::{Path, PathBuf};
use audiotags2::Tag;
use mpris::PlayerFinder;
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use xdir::{config, home};

#[derive(Debug, Default)]
pub struct OverlayMetadata {
    pub name: String,
    pub artist: String,
    pub album: String,
}

#[derive(Debug)]
pub struct SongMetadata {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub lyrics_location: Option<String>,
}
pub fn get_metadata() -> Result<OverlayMetadata, Box<dyn Error>> {
    let player = PlayerFinder::new()?.find_active()?;
    let metadata = player.get_metadata()?;

    Ok(OverlayMetadata {
        name: metadata.title().unwrap_or("").to_string(),
        artist: metadata.artists().unwrap_or(vec![""])[0].to_string(),
        album: metadata.album_name().unwrap_or("").to_string(),
    })
}

fn write_to_database(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let p = Path::new(path.file_stem().unwrap()).join(".lrc");
    if p.exists() {
        let mut tags = Tag::new().read_from_path(&path).expect(format!("Could not read tags from {:?}", path).as_str());
        get_database().execute(r"
        INSERT INTO songs
(name
album

lyrics)
      values (?1, ?2, ?3, ?4)
        ", (tags.title().unwrap_or_default(),
            tags.album().unwrap().title,

            p.to_str())

        ).expect(format!("Could not write to database: {:?}", path).as_str());}
    Ok(())

}

pub fn read_folder(location: PathBuf) {
    for path in location.read_dir().unwrap() {
        if let Ok(path) = path {
            if path.file_type().unwrap().is_file() {
                write_to_database(path.path());
            }
            else if path.file_type().unwrap().is_dir() {
                read_folder(path.path());
            }
        }
    }
}

fn get_database() -> Connection {
    let path = config().map(|path| path.join("snowstormosd")).unwrap_or_default().join("snowstormosd.sqlite");
    let conn = Connection::open(&path).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS songs (\
        name TEXT NOT NULL
        album TEXT
        lyrics TEXT
    \
    )",()).unwrap();
    return conn;
}

#[cfg(test)]
mod tests {
    use crate::get_metadata;

    #[test]
    fn test_get_metadat() {eprintln!("{:?}", get_metadata());
    assert!(true);}}