#[cfg(feature = "generate")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "generate")]
mod inner {
    extern crate glob;
    extern crate serde;
    extern crate serde_json;
    extern crate tempdir;
    extern crate tera;
    extern crate yaml_rust;
    use self::serde_json::value::{from_value, to_value, Value};
    use self::tera::{Context, Result, Tera};
    use self::yaml_rust::YamlLoader;
    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};
    use std::process::Command;

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
        pub fn new(
            label: &str,
            name: &str,
            category: &str,
            os: &str,
            os_version: &str,
            browser_type: &str,
            version: &str,
            vendor: &str,
            target: &str,
        ) -> Data {
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

    fn is_skip_test_for_version(args: &HashMap<String, Value>) -> Result<Value> {
        let name = match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => v,
                Err(_) => {
                    return Err("uasge: is_skip_test_for_version(name='your test name')".into());
                }
            },
            None => "".to_string(),
        };
        let mut is_skip = false;
        for test_name in vec![
            r#"Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)"#,
            r#"Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Trident/6.0; Xbox; Xbox One)"#,
            r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.48 Safari/537.36 Edg/74.1.96.24"#,
            r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.10240"#,
        ] {
            if test_name == name {
                is_skip = true;
                break;
            }
        }

        Ok(to_value(is_skip).unwrap())
    }

    fn is_skip_test_for_os_version(args: &HashMap<String, Value>) -> Result<Value> {
        let name = match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => v,
                Err(_) => {
                    return Err("uasge: is_skip_test_for_os_version(name='your test name')".into());
                }
            },
            None => "".to_string(),
        };
        let mut is_skip = false;
        for test_name in vec![
            r#"Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US; aggregator VocusBot 0.4; +http://www.vocus.com/vnhs.html)"#,
            r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Instapaper/4.0 (+http://www.instapaper.com/)"#,
            r#"Mozilla/5.0 (X11; FreeBSD amd64; rv:8.0) Gecko/20100101 Firefox/8.0"#,
            r#"Mozilla/5.0 (Mobile; rv:18.0) Gecko/18.0 Firefox/18.0"#,
            r#"Mozilla/4.0 (compatible; MSIE 6.0; Windows CE; IEMobile 7.7) S12HT"#,
        ] {
            if test_name == name {
                is_skip = true;
                break;
            }
        }

        Ok(to_value(is_skip).unwrap())
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
            woothee_results.push(Data::new(
                label,
                name,
                category,
                os,
                os_version,
                browser_type,
                version,
                vendor,
                "",
            ));
        }

        context.insert("results", &woothee_results);
        let output = match template_engine.render("dataset.tmpl", &context) {
            Ok(ret) => ret,
            Err(e) => panic!("tera.render() error. {}", e),
        };

        f.write_all(output.as_bytes()).unwrap();
    }

    fn gen_testsets(woothee_dir: PathBuf, root: PathBuf) {
        let woothee_tests_glob = woothee_dir.join("testsets/*.yaml");
        let template_dir = root.join("templates/*.tmpl");
        let mut template_engine = Tera::new(template_dir.to_str().unwrap()).unwrap();
        let mut context = Context::new();

        for entry in glob::glob(woothee_tests_glob.to_str().unwrap()).unwrap() {
            let filename = match entry {
                Ok(path) => path.file_name().unwrap().to_str().unwrap().to_owned(),
                Err(e) => panic!("glob error. {:?}", e),
            };

            let testname = filename.split('.').collect::<Vec<&str>>()[0];
            context.insert("test_fnname", &testname);

            let yaml_file = woothee_dir.join(Path::new(format!("testsets/{}", filename.as_str()).as_str()));
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
                tests.push(Data::new(
                    "",
                    name,
                    category,
                    os,
                    os_version,
                    browser_type,
                    version,
                    vendor,
                    target,
                ));
                context.insert("tests", &tests);
            }

            template_engine.register_function("is_skip_test_for_version", is_skip_test_for_version);
            template_engine.register_function("is_skip_test_for_os_version", is_skip_test_for_os_version);
            let output = match template_engine.render("tests.tmpl", &context) {
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
