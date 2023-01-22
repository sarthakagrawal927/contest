use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.pwd.as_path());
        let mut paths = vec![];
        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }
        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }
        out
    }

    pub fn get_value(&self, key: &String) -> Option<&String> {
        let mut curr = Some(self.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(val) = dir.get(key) {
                    out = Some(val);
                    break;
                }
            }
            curr = p.parent()
        }
        return out;
    }

    pub fn set_value(&mut self, key: String, val: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, val);
    }

    pub fn remove_value(&mut self, key: &String) {
        self.data
            .projector
            .get_mut(&self.pwd)
            .map(|x| x.remove(key));
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(&p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }
        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;
        return Ok(());
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or("{\"projector\":{}}".to_string());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data {
                projector: HashMap::new(),
            });

            return Projector { config, data, pwd };
        }
        return Projector {
            config,
            pwd,
            data: default_data(),
        };
    }
}

#[cfg(test)]
mod test {

    use collection_macros::hashmap;
    use std::{collections::HashMap, path::PathBuf};

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
            PathBuf::from("/")=> hashmap! {
                "foo".into()=>"bar1".into(),
                "disco".into()=> "dancer".into(),
            },
            PathBuf::from("/foo")=> hashmap! {
                "foo".into()=> "bar2".into()
            },
            PathBuf::from("/foo/bar")=> hashmap! {
                "foo".into()=> "bar3".into(),
            },
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: PathBuf::from(""),
            data: Data {
                projector: get_data(),
            },
            pwd,
        };
    }

    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(
            proj.get_value(&String::from("foo")),
            Some(&String::from("bar3"))
        );
        assert_eq!(
            proj.get_value(&String::from("disco")),
            Some(&String::from("dancer"))
        );
    }

    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.set_value(String::from("foo"), String::from("bar"));
        assert_eq!(
            proj.get_value(&String::from("foo")),
            Some(&String::from("bar"))
        );
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.remove_value(&String::from("foo"));
        proj.remove_value(&String::from("discover"));
        assert_eq!(
            proj.get_value(&String::from("foo")),
            Some(&String::from("bar2"))
        );
        assert_eq!(
            proj.get_value(&String::from("disco")),
            Some(&String::from("dancer"))
        );
    }
}
