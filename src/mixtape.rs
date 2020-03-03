use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::fs::{File};
use std::io::Write;

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

impl PlaylistObject {
    fn song_add(&mut self,song_ids: Vec<&String>) {
        for song_id in song_ids {
            &self.song_ids.push(song_id.to_string());
        }
    }
}

impl InputObject {
    fn playlist_del(&mut self, id: String) -> Result<(),&str> {
        self.playlists.retain(|i| i.id != id);

        Ok(())
    }

    fn playlist_song_add(&mut self, playlist_id: String, song_id: String) -> Result<(),&str> {
        if self.songs_find(vec![&song_id]).len() != 1 {
            return Err("could not find exisiting song");
        }

        let playlist_index = self.playlists.iter().position(|x| x.id == playlist_id);

        match playlist_index {
            None => return Err("could not find playlist"),
            Some(index) => {
                &self.playlists[index].song_add(vec![&song_id]);
            }
        };

        Ok(())
    }
    
    fn playlist_create(&mut self, user_id: String, song_ids: Vec<String>) -> Result<(),&str> { 
        let uid = user_id.to_string(); 

        let current_songs = self.get_song_ids();

        for song_id in &song_ids {
            if current_songs.contains(&song_id.as_str()) == false {
                panic!("Song id {} does not exists.",&song_id);
            }
        }

        if self.users_find(vec![uid]).len() != 1 {
            return Err("could not find user");
        }

        self.playlists.push(PlaylistObject {
            id: self.playlist_next_id(),
            user_id: user_id,
            song_ids: song_ids
        });

        Ok(())
    }

    fn songs_find(&mut self, song_ids: Vec<&String>) -> Vec<&SongObject> {
        let mut songs: Vec<&SongObject> = Vec::new();
        for song in self.songs.iter() {
            if song_ids.contains(&&song.id) {
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

    fn playlist_next_id(&self) -> String {
        let next_id = match self.playlists.iter().max_by_key(|p| &p.id) {
            None => 1,
            Some(x) => { 
                let id: u32 = x.id.as_str().parse::<u32>().unwrap();
                (id + 1)
            }
        };

        next_id.to_string()
    }

    fn get_song_ids(&self) -> Vec<&str> {
        self.songs.iter().map(|x| x.id.as_str() ).collect::<Vec<&str>>()
    }
}


pub fn update(params: MixtapeParams) { 
    let input_contents = import_data_file(params.input_file);
    let input_json: serde_json::Value = string_to_json(input_contents);

    // convert json string to mutable rust struct
    let input: &mut InputObject = &mut json_to_rust(input_json);

    let change_contents = import_data_file(params.change_file);
    let change_json: serde_json::Value = string_to_json(change_contents);

    // serialize changes to rust 
    let changes: ChangeObject = serde_json::from_value(change_json).unwrap();

    // apply changes to the serailized rust struct
    apply_changes(input,changes); 
    
    // deserialize rust struct
    let j = serde_json::to_string_pretty(&input).unwrap();

    // write output to json file
    write_output(j,params.output_file);
}

fn import_data_file(file_name: String) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}

fn string_to_json(json_string: String) -> Value {
    serde_json::from_str(&json_string)
        .expect("JSON was not well-formatted")
}

fn json_to_rust(json: Value) -> InputObject {
    serde_json::from_value(json).unwrap()
}

fn write_output(json_string: String, output_file: String) {
    let mut file = File::create(output_file)
        .expect("unable to create ouput file");

    if let Err(e) = file.write_all(json_string.as_bytes()) {
        panic!("Could not write to output file {} ",e);
    }
}

fn apply_changes(input: &mut InputObject, changes: ChangeObject) {
    for change in changes.changes.iter() {
        match change.operation.as_str() {
            "playlist_add_song" => {
                let playlist_id = match change.playlist_id {
                    None => panic!("Did provide playlist_id in playlist_add_song operation."),
                    _ => change.playlist_id.as_deref().unwrap().to_string()
                };

                let song_id = match change.song_id {
                    None => panic!("Did provide song_id in playlist_add_song operation."),
                    _ =>  change.song_id.as_deref().unwrap().to_string()
                };
                if let Err(e) = input.playlist_song_add(playlist_id, song_id) {
                    panic!("Could not add song to playlist: {}", e)
                }
            },
            "playlist_delete" => {
                let playlist_id = match change.playlist_id {
                    None => panic!("Did provide playlist_id in playlist_delete operation."),
                    _ =>  change.playlist_id.as_deref().unwrap().to_string()
                };

                if let Err(e) = input.playlist_del(playlist_id) {
                    panic!("{}", e)
                }
            },
            "playlist_create" => {
                let song_id = match change.song_id {
                    None => panic!("Did provide song_id in playlist_create operation."),
                    _ =>  change.song_id.as_deref().unwrap().to_string()
                };
            
                let user_id = match change.user_id {
                    None => panic!("Did provide song_id in playlist_create operation."),
                    _ => change.user_id.as_deref().unwrap().to_string()
                };
              
                if let Err(e) = input.playlist_create(user_id,vec![song_id]) {
                    panic!("{}",e);
                }
            },
            _ => panic!("Unknown change operation: {} ", change.operation.as_str())
        }
    }
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
        let input: InputObject = json_to_rust(json);

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

        let songs = input.songs_find(vec![&"1".to_string()]);

        assert_eq!(songs.len(),1);
    }

    #[test]
    fn test_find_songs_none() {
        let json: serde_json::Value = string_to_json(test_json());
        let mut input: InputObject = json_to_rust(json);

        let songs = input.songs_find(vec![&"5".to_string()]);

        assert_eq!(songs.len(),0);
    }
}


