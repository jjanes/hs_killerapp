use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::fs::{File};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct InputObject {
    users: Vec<UserObject>,
    playlists: Vec<PlaylistObject>,
    songs: Vec<SongObject>
}

#[derive(Serialize, Deserialize, Debug)]
struct SongObject {
    id: String,
    artist: String,
    title: String
}

#[derive(Serialize, Deserialize, Debug)]
struct UserObject {
    id: String,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistObject {
    id: String,
    user_id: String,
    song_ids: Vec<String>
}

impl InputObject {
    fn playlist_del(&mut self, id: String) {
        self.playlists.retain(|i| i.id != id);
    }

    fn playlists_song_add(&mut self, playlist_id: String, song_id: String) {
    

    }

    fn find_user(&mut self, user_id: String) -> UserObject {


    }

    fn user_playlist_add(&mut self, user_id: String, song_ids: Vec<String>) {


        self.playlists.push(PlaylistObject {
            id: self.get_next_playlist_id(),
            user_id: "1".to_string(),
            song_ids: [].to_vec()
        });
    }

    fn get_next_playlist_id(&self) -> String {
        let mut next_id: u32 = 0;
        for song in self.songs.iter() {
            let id: u32 = song.id.clone().parse().unwrap();
            if id > next_id {
                next_id = id
            }
        }
        (next_id + 1).to_string()
    }
}

fn main() {
    let _input_file = std::env::args().nth(1).expect("No input file given");
    let _change_file = std::env::args().nth(2).expect("No change file found");
    
    let input_contents = import_data_file(_input_file);
    let json: serde_json::Value = string_to_json(input_contents);

    let mut input: InputObject = json_to_rust(json);

    input.users[0].name = "test".to_string();

    let j = serde_json::to_string_pretty(&input).unwrap();

    write_output(j);
    
    println!("[x] Wrote changes.");

}


fn import_data_file(file_name: String) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}

fn string_to_json(json_string: String) -> Value {
    serde_json::from_str(&json_string)
        .expect("JSON was not well-formatted")
}


fn import_playlist(json: serde_json::Value) {
    println!("TEST:\n{}", json)
}


fn json_to_rust(json: Value) -> InputObject {
    serde_json::from_value(json).unwrap()
}

fn write_output(json_string: String) {
    let mut file = File::create("foo.txt")
        .expect("unable to create file");
    file.write_all(json_string.as_bytes());
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_json() -> String {
        r#"
        {
            "users": [
                {
                    "id": "1", "name": "John Janes"
                }
            ],
            "playlists": [
                {
                    "id": "1", "user_id": "1", "song_ids": ["1","2"]
                },
                {
                    "id": "2", "user_id": "1", "song_ids": ["2"]
                }
            ],
            "songs": [
                {
                  "id" : "1", "artist": "Camila Cabello", "title": "Never Be the Same"
                },
                {
                  "id" : "2", "artist": "Zedd", "title": "The Middle" 
                }
            ]
        }
        "#.to_string()
    }


    #[test] 
    fn test_string_to_json() {
        let json: Value = string_to_json(test_json());

        assert_eq!(json["users"][0]["name"], "John Janes".to_string());
    }

    #[test]
    fn test_remove_playlists() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        input.playlist_del("1".to_string());

        assert_eq!(input.playlists[0].id,"2".to_string());
    }

    #[test]
    fn test_next_playlist_id() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        assert_eq!(input.get_next_playlist_id(),"3".to_string());
    }
}


