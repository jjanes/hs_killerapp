# About
This application was built with the rust programming language. 

It uses only one non standard rust library:

Sere JSON
This library handles the serialization and deserialization of JSON objects.
https://docs.serde.rs/serde_json/

For instructions on installing rust and cargo please visit: 
https://www.rust-lang.org/tools/install

# Installing dependencies & Running the app
in your favorite terminal goto to the application directory and type: 

The following command will install all dependencies
```
cargo update
```

The following command will run the app
```
cargo run <input file> <change file> <output file>
```

An example
```
cargo run mixtape-data.json changset-data.json output-data.json
```

# Testing
``` 
cargo test
```
# How we scale
The first thing I would do is move this to a database. I would separate the app into 3 seperate commands. A command that would load the initial data, a command to apply the changesets, and a command to export data to the output file. 

