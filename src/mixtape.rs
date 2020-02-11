use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::fs::{File};
use std::io::Write;

pub fn update(params: MixtapeParams) { 
    let input_contents = import_data_file(params.input_file);
    let input_json: serde_json::Value = string_to_json(input_contents);

    let mut input: InputObject = json_to_rust(input_json);

    input.users[0].name = "test".to_string();

    
    let change_contents = import_data_file(params.change_file);
    let change_json: serde_json::Value = string_to_json(change_contents);
    
    let mut changes: ChangeObject = serde_json::from_value(change_json).unwrap();

    changes.apply(&input);



    let j = serde_json::to_string_pretty(&input).unwrap();

    write_output(j,params.output_file);
}

pub struct MixtapeParams {
    pub input_file: String,
    pub change_file: String,
    pub output_file: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ChangeObject {
    changes: Vec<ChangeOperationObject>
}

#[derive(Serialize, Deserialize, Debug)]
struct ChangeOperationObject {
    operation: String,
    user_id: Option<String>,
    song_id: Option<String>,
    playlist_id: Option<String>
}

impl ChangeObject {
    fn apply(&self, input: &InputObject) {
        for change in &self.changes { 
            change.apply(input);
        }
    }
}

impl ChangeOperationObject {
    fn apply(&self, input: &InputObject) {
        match self.operation.as_str() {
            "playlist_add_song" => {
            },
            "playlist_delete" => {
            },
            "playlist_create" => {
            },
            _ => panic!("Unknown change operation: {} ", self.operation.as_str())
        }
    }
}


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

    fn playlist_song_add(&mut self, playlist_id: String, song_id: String) {


    }
    
    fn playlist_create(&mut self, user_id: String, song_ids: Vec<String>) 
        -> Option<&PlaylistObject> {
        let uid = user_id.to_string(); 

        if self.users_find(vec![uid]).len() != 1 {
            return None;
        }

        self.playlists.push(PlaylistObject {
            id: self.playlist_next_id(),
            user_id: user_id,
            song_ids: song_ids
        });

        self.playlists.last()
    }

    fn songs_find(&mut self, song_ids: Vec<String>) -> Vec<&SongObject> {
        let mut songs: Vec<&SongObject> = Vec::new();
        for song in self.songs.iter() {
            if song_ids.contains(&song.id) {
                songs.push(&song);
            }
        }
        songs
    }

    fn users_find(&mut self, user_ids: Vec<String>) -> Vec<&UserObject> {
        let mut users: Vec<&UserObject> = Vec::new();
        for user in self.users.iter() {
            if user_ids.contains(&user.id) {
                users.push(&user);
            }
        }
        users
    }

    fn plalist_find(&mut self, playlist_id: String, song_id: String) -> Option<&PlaylistObject> {

        None
    }

    fn playlist_next_id(&self) -> String {
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

fn write_output(json_string: String, output_file: String) {
    let mut file = File::create(output_file)
        .expect("unable to create ouput file");

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

        assert_eq!(input.playlist_next_id(),"3".to_string());
    }

    #[test]
    fn test_find_users_one() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        let users = input.users_find(vec!["1".to_string()]);

        assert_eq!(users.len(),1);
    }

    #[test]
    fn test_find_users_none() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        let users = input.users_find(vec!["2".to_string()]);

        assert_eq!(users.len(),0);
    }

    #[test]
    fn test_find_songs_one() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        let songs = input.songs_find(vec!["1".to_string()]);

        assert_eq!(songs.len(),1);
    }

    #[test]
    fn test_find_songs_none() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        let songs = input.songs_find(vec!["5".to_string()]);

        assert_eq!(songs.len(),0);
    }
}


