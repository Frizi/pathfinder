use std::fs::File;
use std::env;
use std::path::{PathBuf, Path};
use phf_codegen::Map;

fn add_dir(map: &mut Map<String>, root: &Path, dir: &Path) {
    println!("{:?}", dir);
    let abs_dir = root.join(dir);
    for entry in abs_dir.read_dir().expect("not a directory") {
        let entry = entry.unwrap();
        let typ = entry.file_type().unwrap();
        let path = dir.join(entry.file_name());
        if typ.is_file() {
            let file_path = root.join(&path);
            map.entry(path.to_str().expect("non-utf8 filename").into(), &format!("include_bytes!({:?})", file_path));
        } else if typ.is_dir() {
            add_dir(map, root, &path)
        }
    }
}

fn main() {
    let resources = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent().unwrap()
        .join("resources");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", resources.to_str().expect("no-utf8 path"));
    let file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("resources_data.rs");
    
    let mut file = File::create(file_path).unwrap();
    let mut map = Map::new();

    #[cfg(feature="gl3_shaders")]
    add_dir(&mut map, &resources, Path::new("shaders/gl3"));

    #[cfg(feature="metal_shaders")]
    add_dir(&mut map, &resources, Path::new("shaders/metal"));

    #[cfg(feature="fonts")]
    add_dir(&mut map, &resources, Path::new("fonts"));

    #[cfg(feature="debug-fonts")]
    add_dir(&mut map, &resources, Path::new("debug-fonts"));

    #[cfg(feature="svg")]
    add_dir(&mut map, &resources, Path::new("svg"));

    #[cfg(feature="textures_lut")]
    add_dir(&mut map, &resources, Path::new("textures/lut"));

    #[cfg(feature="textures_demo")]
    add_dir(&mut map, &resources, Path::new("textures/demo"));

    #[cfg(feature="textures_debug")]
    add_dir(&mut map, &resources, Path::new("textures/debug"));

    map.build(&mut file).unwrap();
}
