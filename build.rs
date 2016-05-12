#[cfg(feature = "code-update")]
mod inner {
    extern crate yaml_rust;
    extern crate tempdir;
    use std::env;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use self::tempdir::TempDir;
    use self::yaml_rust::YamlLoader;

    fn gen_dataset(woothee_dir: PathBuf, dest: PathBuf) {
        let mut f = File::create(&dest).unwrap();

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

        let header = "// This file is auto-generated! Any changes to this file will be lost!
use \
                      std::collections::HashMap;
use parser::WootheeResult;

pub fn \
                      get_default_dataset() -> HashMap<String, WootheeResult> {
    let mut \
                      dataset: HashMap<String, WootheeResult> = HashMap::new();
";
        let footer = "    dataset\n}";
        f.write_all(header.as_bytes()).unwrap();

        let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
        let doc = &docs[0];
        for vecc in doc.as_vec().unwrap() {
            for (key, value) in vecc.as_hash().unwrap() {
                match key.as_str().unwrap() {
                    "label" => {
                        f.write_all(format!("    dataset.insert(\"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                        f.write_all(b"        WootheeResult {\n").unwrap();
                    }
                    "name" => {
                        f.write_all(format!("      name: \"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                    }
                    "type" => {
                        f.write_all(format!("      browser_type: \"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                    }
                    "vendor" => {
                        f.write_all(format!("      vendor: \"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                    }
                    "category" => {
                        f.write_all(format!("      category: \"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                    }
                    "os" => {
                        f.write_all(format!("      os: \"{}\".to_string(),\n",
                                            value.as_str().unwrap())
                                        .as_bytes())
                         .unwrap();
                    }
                    _ => (),
                }
            }
            f.write_all(b"        ..WootheeResult::default()\n    });\n").unwrap();
        }

        // f.write_all(output.as_bytes()).unwrap();

        f.write_all(footer.as_bytes()).unwrap();
    }

    pub fn gen() {
        let dest_path = env::current_dir().unwrap().join("./src/dataset.rs");

        // git clone in tempdir
        let dir = TempDir::new("fooo").unwrap();
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
        gen_dataset(woothee_dir, dest_path);
    }
}


#[cfg(not(feature = "code-update"))]
mod inner {
    pub fn gen() {}
}

fn main() {
    inner::gen();
}
