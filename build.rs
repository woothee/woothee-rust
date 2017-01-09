#[cfg_attr(feature = "generate", feature(proc_macro))]
#[cfg(feature = "generate")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "generate")]
mod inner {
    extern crate yaml_rust;
    extern crate tempdir;
    extern crate tera;
    extern crate serde;
    extern crate glob;
    extern crate serde_json;
    use std::env;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use self::yaml_rust::YamlLoader;
    use self::tera::{Tera, Context};

    #[derive(Serialize, Deserialize, Debug)]
    struct Data {
        label: String,
        name: String,
        category: String,
        os: String,
        os_version: String,
        browser_type: String,
        version: String,
        vendor: String,
        target: String,
    }

    impl Data {
        pub fn new(label: &str,
                   name: &str,
                   category: &str,
                   os: &str,
                   os_version: &str,
                   browser_type: &str,
                   version: &str,
                   vendor: &str,
                   target: &str)
                   -> Data {
            Data {
                label: label.to_owned(),
                name: name.to_owned(),
                category: category.to_owned(),
                browser_type: browser_type.to_owned(),
                os: os.to_owned(),
                os_version: os_version.to_owned(),
                version: version.to_owned(),
                vendor: vendor.to_owned(),
                target: target.to_owned(),
            }
        }
    }

    fn gen_dataset(woothee_dir: PathBuf, root: PathBuf) {
        let dest = root.join("./src/dataset.rs");
        let template_dir = root.join("templates/*.tmpl");
        let mut f = File::create(&dest).unwrap();

        // load dataset
        match env::set_current_dir(woothee_dir.as_path()) {
            Err(e) => panic!("set_current_dir() error. {}", e),
            Ok(_) => (),
        };
        let yaml_file = woothee_dir.join(Path::new("dataset.yaml"));
        let path = Path::new(&yaml_file);
        let mut y = File::open(&path).unwrap();
        let mut s = String::new();
        match y.read_to_string(&mut s) {
            Err(e) => panic!("yaml.read_to_string() {}", e),
            Ok(_) => (),
        }

        // render dataset.rs
        let template_engine = Tera::new(template_dir.to_str().unwrap()).unwrap();
        let mut context = Context::new();
        let mut woothee_results = vec![];

        let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
        let doc = &docs[0];
        for vecc in doc.as_vec().unwrap() {
            let mut label = "";
            let mut name = "";
            let mut browser_type = "";
            let mut category = "";
            let mut os = "";
            let mut os_version = "";
            let mut version = "";
            let mut vendor = "";
            for (key, value) in vecc.as_hash().unwrap() {
                match key.as_str().unwrap() {
                    "label" => {
                        label = value.as_str().unwrap();
                    }
                    "name" => {
                        name = value.as_str().unwrap();
                    }
                    "type" => {
                        browser_type = value.as_str().unwrap();
                    }
                    "vendor" => {
                        vendor = value.as_str().unwrap();
                    }
                    "category" => {
                        category = value.as_str().unwrap();
                    }
                    "os" => {
                        os = value.as_str().unwrap();
                    }
                    "os_version" => {
                        os_version = value.as_str().unwrap();
                    }
                    "version" => {
                        version = value.as_str().unwrap();
                    }
                    _ => (),
                }
            }
            woothee_results.push(Data::new(label,
                                           name,
                                           category,
                                           os,
                                           os_version,
                                           browser_type,
                                           version,
                                           vendor,
                                           ""));
        }

        context.add("results", &woothee_results);
        let output = match template_engine.render("dataset.tmpl", context) {
            Ok(ret) => ret,
            Err(e) => panic!("tera.render() error. {}", e),
        };

        f.write_all(output.as_bytes()).unwrap();
    }

    fn gen_testsets(woothee_dir: PathBuf, root: PathBuf) {
        let woothee_tests_glob = woothee_dir.join("testsets/*.yaml");
        let template_dir = root.join("templates/*.tmpl");
        let template_engine = Tera::new(template_dir.to_str().unwrap()).unwrap();
        let mut context = Context::new();

        for entry in glob::glob(woothee_tests_glob.to_str().unwrap()).unwrap() {
            let filename = match entry {
                Ok(path) => path.file_name().unwrap().to_str().unwrap().to_owned(),
                Err(e) => panic!("glob error. {:?}", e),
            };

            let v: Vec<&str> = filename.split('.').collect();
            let testname = v[0];
            context.add("test_fnname", &testname);

            let yaml_file =
                woothee_dir.join(Path::new(format!("testsets/{}", filename.as_str()).as_str()));
            let path = Path::new(&yaml_file);
            let mut y = match File::open(&path) {
                Ok(f) => f,
                Err(e) => panic!("File::open() error. {}", e),
            };
            let mut s = String::new();
            match y.read_to_string(&mut s) {
                Err(e) => panic!("yaml.read_to_string() {}", e),
                Ok(_) => (),
            }

            let dest = root.join(format!("tests/{}.rs", testname));
            let mut tests = vec![];
            let mut f = match File::create(&dest) {
                Ok(f) => f,
                Err(e) => panic!("File::create() error. {}", e),
            };

            let docs = match YamlLoader::load_from_str(s.as_str()) {
                Ok(d) => d,
                Err(e) => panic!("YamlLoader::load_from_str() error. {}", e),
            };
            let doc = &docs[0];
            for vecc in doc.as_vec().unwrap() {
                let mut target = "";
                let mut name = "";
                let mut browser_type = "";
                let mut category = "";
                let mut os = "";
                let mut os_version = "";
                let mut version = "";
                let mut vendor = "";
                for (key, value) in vecc.as_hash().unwrap() {
                    match key.as_str().unwrap() {
                        "target" => {
                            target = match value.as_str() {
                                Some(v) => v,
                                None => "",
                            };
                        }
                        "name" => {
                            name = value.as_str().unwrap();
                        }
                        "type" => {
                            browser_type = value.as_str().unwrap();
                        }
                        "vendor" => {
                            vendor = value.as_str().unwrap();
                        }
                        "category" => {
                            category = value.as_str().unwrap();
                        }
                        "os" => {
                            os = value.as_str().unwrap();
                        }
                        "os_version" => {
                            os_version = value.as_str().unwrap();
                        }
                        "version" => {
                            version = value.as_str().unwrap();
                        }
                        _ => (),
                    }
                }
                tests.push(Data::new("",
                                     name,
                                     category,
                                     os,
                                     os_version,
                                     browser_type,
                                     version,
                                     vendor,
                                     target));
                context.add("tests", &tests);
            }

            let output = match template_engine.render("tests.tmpl", context.to_owned()) {
                Ok(ret) => ret,
                Err(e) => panic!("tera.render() error. {}", e),
            };

            f.write_all(output.as_bytes()).unwrap();
        }
    }

    pub fn gen() {
        let root_path = env::current_dir().unwrap();

        // git clone in tempdir
        let dir = tempdir::TempDir::new("fooo").unwrap();
        let mut curdir = dir.path();
        match env::set_current_dir(&mut curdir) {
            Err(e) => panic!("set_current_dir() error. {}", e),
            Ok(_) => (),
        }

        let _ = Command::new("git")
            .arg("clone")
            .arg("-q")
            .arg("https://github.com/woothee/woothee.git")
            .output()
            .unwrap();

        let woothee_dir = curdir.join(Path::new("woothee"));
        gen_dataset(woothee_dir.to_owned(), root_path.to_owned());
        gen_testsets(woothee_dir, root_path);
    }
}


#[cfg(not(feature = "generate"))]
mod inner {
    pub fn gen() {}
}

fn main() {
    inner::gen();
}
