use std::{borrow::Cow, fs, path::Path};

use home::home_dir;
use keyvalues_parser::{Value, Vdf};
use rust_search::SearchBuilder;

pub struct Steam {
    pub localconfig: String,
}

impl Steam {
    pub fn new() -> Self {
        Steam {
            localconfig: Steam::discover_localconfig(),
        }
    }

    fn discover_localconfig() -> String {
        let search: Vec<String> = SearchBuilder::default()
            .location(home_dir().unwrap().to_str().unwrap())
            .search_input("localconfig")
            .limit(1)
            .ext("vdf")
            .strict()
            .hidden()
            .build()
            .collect();

        if search.len() == 0 {
            panic!("Failed to find localconfig.vdf!");
        }

        search.first().unwrap().to_owned()
    }

    pub fn get_hash(&self) -> String {
        let bytes = std::fs::read(Path::new(&self.localconfig)).unwrap();
        return sha256::digest(&bytes);
    }

    pub fn app_ids(&self) -> Option<Vec<String>> {
        let contents = fs::read_to_string(&self.localconfig).unwrap();
        let vdf = Vdf::parse(&contents).expect("Failed to parse VDF file");

        let apps = vdf
            .value
            .get_obj()?
            .get("Software")?
            .get(0)?
            .get_obj()?
            .get("Valve")?
            .get(0)?
            .get_obj()?
            .get("Steam")?
            .get(0)?
            .get_obj()?
            .get("apps")?
            .get(0)?
            .get_obj()?
            .iter();

        let mut app_ids: Vec<String> = vec![];
        for (app_id, _) in apps {
            app_ids.push(app_id.to_string());
        }

        return Some(app_ids);
    }

    pub fn update_all_launch_options<F: Fn(Option<String>) -> String>(
        &self,
        app_ids: Vec<String>,
        get_launch_options: F,
    ) -> Option<()> {
        let contents = fs::read_to_string(&self.localconfig).unwrap();
        let mut vdf = Vdf::parse(&contents).expect("Failed to parse VDF file");

        let apps = vdf
            .value
            .get_mut_obj()?
            .get_mut("Software")?
            .get_mut(0)?
            .get_mut_obj()?
            .get_mut("Valve")?
            .get_mut(0)?
            .get_mut_obj()?
            .get_mut("Steam")?
            .get_mut(0)?
            .get_mut_obj()?
            .get_mut("apps")?
            .get_mut(0)?
            .get_mut_obj()?;

        for app_id in app_ids {
            let app = apps.get_mut(app_id.as_str())?.get_mut(0)?.get_mut_obj()?;

            let launch_options = if app.contains_key("LaunchOptions") {
                app.get_mut("LaunchOptions")?
                    .get_mut(0)?
                    .get_mut_str()
                    .map(|value| value.to_string())
            } else {
                None
            };

            app.insert(
                Cow::from("LaunchOptions"),
                vec![Value::Str(Cow::from(get_launch_options(launch_options)))],
            );
        }

        fs::write(&self.localconfig, vdf.to_string()).expect("Failed to write file");

        Some(())
    }

    pub fn update_launch_options<F: Fn(Option<String>) -> String>(
        &self,
        app_id: String,
        get_launch_options: F,
    ) -> Option<()> {
        self.update_all_launch_options(vec![app_id], get_launch_options)
    }
}
