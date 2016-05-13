#[cfg(feature = "generate")]
mod inner {
    extern crate yaml_rust;
    extern crate tempdir;
    extern crate tera;
    extern crate serde;
    use std::env;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use self::yaml_rust::YamlLoader;

    #[derive(Debug)]
    struct Data {
        label: String,
        name: String,
        category: String,
        os: String,
        os_version: String,
        browser_type: String,
        version: String,
        vendor: String,
    }
    impl Data {
        pub fn new(label: &str,
                   name: &str,
                   category: &str,
                   os: &str,
                   os_version: &str,
                   browser_type: &str,
                   version: &str,
                   vendor: &str)
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
            }
        }
    }

    impl serde::Serialize for Data {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer
        {
            serializer.serialize_struct("Data",
                                        DataMapVisitor {
                                            value: self,
                                            state: 0,
                                        })
        }
    }

    struct DataMapVisitor<'a> {
        value: &'a Data,
        state: u8,
    }

    impl<'a> serde::ser::MapVisitor for DataMapVisitor<'a> {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
            where S: serde::Serializer
        {
            match self.state {
                0 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("label", &self.value.label))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("name", &self.value.name))))
                }
                2 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("category",
                                                                 &self.value.category))))
                }
                3 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("os", &self.value.os))))
                }
                4 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("os_version",
                                                                 &self.value.os_version))))
                }
                5 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("browser_type",
                                                                 &self.value.browser_type))))
                }
                6 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("version", &self.value.version))))
                }
                7 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("vendor", &self.value.vendor))))
                }
                _ => Ok(None),
            }
        }
    }

    fn gen_dataset(woothee_dir: PathBuf, root: PathBuf) {
        let dest = root.join("./src/dataset.rs");
        let template_dir = root.join("templates/*.tmpl");
        let mut f = File::create(&dest).unwrap();

        // load dataset
        match env::set_current_dir(woothee_dir.as_path()) {
            Err(why) => panic!("set_current_dir() error. {}", why),
            Ok(_) => (),
        };
        let yaml_file = woothee_dir.join(Path::new("dataset.yaml"));
        let path = Path::new(&yaml_file);
        let mut y = File::open(&path).unwrap();
        let mut s = String::new();
        match y.read_to_string(&mut s) {
            Err(why) => panic!("yaml.read_to_string() {}", why),
            Ok(_) => (),
        }

        // render dataset.rs
        let template_engine = tera::Tera::new(template_dir.to_str().unwrap());
        let mut context = tera::Context::new();
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
                                           vendor));
        }

        context.add("results", &woothee_results);
        let output = match template_engine.render("dataset.tmpl", context) {
            Ok(ret) => ret,
            Err(why) => panic!("tera.render() error. {}", why),
        };

        f.write_all(output.as_bytes()).unwrap();
    }

    pub fn gen() {
        let root_path = env::current_dir().unwrap();

        // git clone in tempdir
        let dir = tempdir::TempDir::new("fooo").unwrap();
        let mut curdir = dir.path();
        match env::set_current_dir(&mut curdir) {
            Err(why) => panic!("set_current_dir() error. {}", why),
            Ok(_) => (),
        }

        let _ = Command::new("git")
                    .arg("clone")
                    .arg("-q")
                    .arg("https://github.com/woothee/woothee.git")
                    .output()
                    .unwrap();

        let woothee_dir = curdir.join(Path::new("woothee"));
        gen_dataset(woothee_dir, root_path);
    }
}


#[cfg(not(feature = "generate"))]
mod inner {
    pub fn gen() {}
}

fn main() {
    inner::gen();
}
