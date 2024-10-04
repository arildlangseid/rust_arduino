use bindgen::{Bindings, Builder, CargoCallbacks};
use cc::Build;
use glob::glob;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

//mod build_arduino_helper;
//use crate::build_arduino_helper::*;

const CONFIG_FILE: &str = "clibs_bindings.yaml";

#[derive(Debug, Deserialize)]
struct BindgenLists {
    pub allowlist_function: Vec<String>,
    pub allowlist_type: Vec<String>,
    pub blocklist_function: Vec<String>,
    pub blocklist_type: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct BindgenArduinoHelperLists {
    pub allowlist_function: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    pub arduino_home: String,
    pub external_libraries_home: String,
    pub core_version: String,
    pub variant: String,
    pub avr_gcc_version: String,
    pub arduino_libraries: Vec<String>,
    pub external_libraries: Vec<String>,
    pub definitions: HashMap<String, String>,
    pub flags: Vec<String>,
    pub bindgen_lists: BindgenLists,
    pub bindgen_arduino_helper_lists: BindgenArduinoHelperLists,
}

impl Config {
    fn local_path(&self) -> PathBuf {
        let expanded = "./";
        let arduino_home_path = PathBuf::from(expanded);
        arduino_home_path.join("src_c")
    }

    fn arduino_package_path(&self) -> PathBuf {
        let expanded = envmnt::expand(&self.arduino_home, None);
        let arduino_home_path = PathBuf::from(expanded);
        arduino_home_path.join("packages").join("arduino")
    }

    fn core_path(&self) -> PathBuf {
        self.arduino_package_path()
            .join("hardware")
            .join("avr")
            .join(&self.core_version)
    }

    fn avr_gcc_home(&self) -> PathBuf {
        self.arduino_package_path()
            .join("tools")
            .join("avr-gcc")
            .join(&self.avr_gcc_version)
    }

    fn avg_gcc(&self) -> PathBuf {
        self.avr_gcc_home().join("bin").join("avr-gcc")
    }

    fn arduino_core_path(&self) -> PathBuf {
        self.core_path().join("cores").join("arduino")
    }

    fn arduino_include_dirs(&self) -> Vec<PathBuf> {
        let variant_path = self.core_path().join("variants").join(&self.variant);
        let avr_gcc_include_path = self.avr_gcc_home().join("avr").join("include");
        vec![self.arduino_core_path(), variant_path, avr_gcc_include_path]
    }

    fn arduino_libraries_path(&self) -> Vec<PathBuf> {
        let library_root = self.core_path().join("libraries");
        let mut result = vec![];
        for library in &self.arduino_libraries {
            result.push(library_root.join(library).join("src"))
        }
        result
    }

    fn external_libraries_path(&self) -> Vec<PathBuf> {
        let expanded = envmnt::expand(&self.external_libraries_home, None);
        let external_library_root = PathBuf::from(expanded);
        let mut result = vec![];
        for library in &self.external_libraries {
            result.push(external_library_root.join(library))
        }
        result
    }

    fn include_dirs(&self) -> Vec<PathBuf> {
        let mut result = self.arduino_include_dirs();
        result.extend(self.arduino_libraries_path());
        result.extend(self.external_libraries_path());
        result
    }

    fn include_arduino_helper_dirs(&self) -> Vec<PathBuf> {
        let expanded = envmnt::expand("./src_c", None);
        let arduino_home_path = PathBuf::from(expanded);

        let mut result = self.arduino_include_dirs();
        result.extend(vec![arduino_home_path]);
        result
    }

    fn project_files(&self, patten: &str) -> Vec<PathBuf> {
        let mut result =
            files_in_folder(self.arduino_core_path().to_string_lossy().as_ref(), patten);
        let mut libraries = self.arduino_libraries_path();
        libraries.extend(self.external_libraries_path());

        let pattern = format!("**/{}", patten);
        for library in libraries {
            let lib_sources = files_in_folder(library.to_string_lossy().as_ref(), &pattern);
            if crate::debug_print() { println!("cargo:warning=********************   libraries  ******************** {}", library.to_string_lossy()); }
            result.extend(lib_sources);
        }

        result
    }

    fn project_arduino_helper_files(&self, patten: &str) -> Vec<PathBuf> {
        let result = files_in_folder(self.local_path().to_string_lossy().as_ref(), patten);
        if crate::debug_print_arduino_helper() { println!("cargo:warning=********************   project_arduino_files  ******************** {}", self.local_path().to_string_lossy()); }

//        let libraries = self.arduino_libraries_path();

//        let pattern = format!("**/{}", patten);
//        for library in libraries {
//            let lib_sources = files_in_folder(library.to_string_lossy().as_ref(), &pattern);
//            println!("cargo:warning=********************   libraries  ******************** {}", library.to_string_lossy());
//            result.extend(lib_sources);
//        }

        result
    }

    fn cpp_files(&self) -> Vec<PathBuf> {
        self.project_files("*.cpp")
    }
    fn cpp_arduino_helper_files(&self) -> Vec<PathBuf> {
        self.project_arduino_helper_files("*.cpp")
    }


    fn c_files(&self) -> Vec<PathBuf> {
        self.project_files("*.c")
    }

    // c_arduino_helper_files is not used until we need to compile c sourvce
    //fn c_arduino_helper_files(&self) -> Vec<PathBuf> {
    //    self.project_arduino_helper_files("*.c")
    //}

    fn bindgen_headers(&self) -> Vec<PathBuf> {
        let mut result = vec![];
        for library in self.external_libraries_path() {
            let lib_headers = files_in_folder(library.to_string_lossy().as_ref(), "*.h");
            result.extend(lib_headers);
        }
        result
    }
    fn bindgen_arduino_helper_headers(&self) -> Vec<PathBuf> {
/*
        fn external_libraries_path()
        let expanded = envmnt::expand(&self.external_libraries_home, None);
        let external_library_root = PathBuf::from(expanded);
        let mut result = vec![];
        for library in &self.external_libraries {
            result.push(external_library_root.join(library))
        }
        result
*/
        let mut result = vec![];
/*
        for library in self.external_libraries_path() {
            let lib_headers = files_in_folder(library.to_string_lossy().as_ref(), "*.h");
            result.extend(lib_headers);
        }
*/
        let lib_headers = files_in_folder("./src_c/", "*.h");
        result.extend(lib_headers);

        for header in &result {
            if crate::debug_print_arduino_helper() { println!("cargo:warning=********************   header    ********************* {}", header.to_string_lossy()); }
        }

        result
    }
}

fn files_in_folder(folder: &str, pattern: &str) -> Vec<PathBuf> {
    let cpp_pattern = format!("{}/{}", folder, pattern);
    let mut results = vec![];
    for cpp_file in glob(&cpp_pattern).unwrap() {
        let file = cpp_file.unwrap();
        if !file.ends_with("main.cpp") {
            results.push(file);
        }
    }
    results
}

pub fn add_source_file(builder: &mut Build, files: Vec<PathBuf>) {
    for file in files {
        println!("cargo:rerun-if-changed={}", file.to_string_lossy());
        builder.file(file);
    }
}

fn configure_arduino(config: &Config) -> Build {
    let mut builder = Build::new();
    for (k, v) in &config.definitions {
        builder.define(k, v.as_str());
    }
    for flag in &config.flags {
        builder.flag(flag);
    }
    builder
        .compiler(config.avg_gcc())
        .flag("-Os")
        .cpp_set_stdlib(None)
        .flag("-fno-exceptions")
        .flag("-ffunction-sections")
        .flag("-fdata-sections");

    for include_dir in config.include_dirs() {
        builder.include(include_dir);
    }
    builder
}

fn debug_print() -> bool {
    false
}
fn debug_print_arduino_helper() -> bool {
    false
}
fn compile_arduino(config: &Config) {
    if debug_print() { println!("cargo:warning=***********************   1   ************************"); }
    let mut builder = configure_arduino(&config);
    builder
        .cpp(true)
        .flag("-std=gnu++11")
        .flag("-fpermissive")
        .flag("-fno-threadsafe-statics");

    if debug_print() { println!("cargo:warning=***********************   2   ************************"); }
    add_source_file(&mut builder, config.cpp_files());

    if debug_print() { println!("cargo:warning=***********************   3   ************************"); }
    builder.compile("libarduino_c++.a");
    if debug_print() { println!("cargo:warning=***********************   4   ************************"); }
    println!("cargo:rustc-link-lib=static=arduino_c++");

    if debug_print() { println!("cargo:warning=***********************   5   ************************"); }

    let mut builder = configure_arduino(&config);
    builder.flag("-std=gnu11");
    add_source_file(&mut builder, config.c_files());
    if debug_print() { println!("cargo:warning=***********************   6   ************************"); }
    builder.compile("libarduino_c.a");
    if debug_print() { println!("cargo:warning=***********************   7   ************************"); }
    println!("cargo:rustc-link-lib=static=arduino_c");

    if debug_print() { println!("cargo:warning=***********************   8   ************************"); }
}

fn configure_bindgen_for_arduino(config: &Config) -> Builder {
    let mut builder = Builder::default();
    if debug_print() { println!("cargo:warning=***********************   9   ************************"); }
    for (k, v) in &config.definitions {
        builder = builder.clang_arg(&format!("-D{}={}", k, v));
    }
    for flag in &config.flags {
        builder = builder.clang_arg(flag);
    }
    builder = builder
        .clang_args(&["-x", "c++", "-std=gnu++11"])
        .use_core()
        .layout_tests(false)
        .parse_callbacks(Box::new(CargoCallbacks::new()));

    for item in &config.bindgen_lists.allowlist_function {
        builder = builder.allowlist_function(item);
    }
    for item in &config.bindgen_lists.allowlist_type {
        builder = builder.allowlist_type(item);
    }
    for item in &config.bindgen_lists.blocklist_function {
        builder = builder.blocklist_function(item);
    }
    for item in &config.bindgen_lists.blocklist_type {
        builder = builder.blocklist_type(item);
    }

    for include_dir in config.include_dirs() {
        builder = builder.clang_arg(&format!("-I{}", include_dir.to_string_lossy()));
    }
    for header in config.bindgen_headers() {
        builder = builder.header(header.to_string_lossy());
    }
    builder
}

fn generate_bindings(config: &Config) {
    let bindings: Bindings = configure_bindgen_for_arduino(&config)
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate bindings");
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("clibs_bindings.rs");
    bindings
        .write_to_file(project_root)
        .expect("Couldn't write bindings!");
}

/*

        Build Local C Lib

 */
fn configure_arduino_helper(config: &Config) -> Build {
    let mut builder = Build::new();
    for (k, v) in &config.definitions {
        builder.define(k, v.as_str());
        if debug_print_arduino_helper() { println!("cargo:warning=******************   definitions   ******************* {}: {}", k.as_str(), v.as_str()); }
    }
    for flag in &config.flags {
        builder.flag(flag);
        if debug_print_arduino_helper() { println!("cargo:warning=*********************   flags   ********************** {}", flag.as_str()); }
    }
    builder
        .compiler(config.avg_gcc())
        .flag("-Os")
        .cpp_set_stdlib(None)
        .flag("-fno-exceptions")
        .flag("-ffunction-sections")
        .flag("-fdata-sections");

    for include_dir in config.include_arduino_helper_dirs() {
        builder.include(include_dir.clone());
        if debug_print_arduino_helper() { println!("cargo:warning=******************   include_dir   ******************* {}", include_dir.to_string_lossy().to_string()); }
    }
    builder
}

fn compile_arduino_helper_lib(config: &Config) {
    if debug_print_arduino_helper() { println!("cargo:warning=***********************   A   ************************"); }
    let mut builder = configure_arduino_helper(&config);
    builder
        .cpp(true)
        .flag("-std=gnu++11")
        .flag("-fpermissive")
        .flag("-fno-threadsafe-statics");

    if debug_print_arduino_helper() { println!("cargo:warning=***********************   B   ************************"); }
    add_source_file(&mut builder, config.cpp_arduino_helper_files());

    if debug_print_arduino_helper() { println!("cargo:warning=***********************   C   ************************"); }
    builder.compile("libarduinohelper_c++.a");
    if debug_print_arduino_helper() { println!("cargo:warning=***********************   D   ************************"); }
    println!("cargo:rustc-link-lib=static=arduinohelper_c++");

    if debug_print_arduino_helper() { println!("cargo:warning=***********************   E   ************************"); }

    // uncomment if building c source
    //let mut builder = configure_local_arduino(&config);
    //builder.flag("-std=gnu11");
    //add_source_file(&mut builder, config.c_arduino_helper_files());
    //if debug_print_arduino_helper() { println!("cargo:warning=***********************   F   ************************"); }
    //builder.compile("libarduinohelper_c.a");
    //if debug_print_arduino_helper() { println!("cargo:warning=***********************   G   ************************"); }
    //println!("cargo:rustc-link-lib=static=arduinohelper_c");
    //if debug_print_arduino_helper() { println!("cargo:warning=***********************   H   ************************"); }
}

fn configure_bindgen_for_arduino_helper(config: &Config) -> Builder {
    let mut builder = Builder::default();
    if debug_print_arduino_helper() { println!("cargo:warning=***********************   I   ************************"); }
    for (k, v) in &config.definitions {
        builder = builder.clang_arg(&format!("-D{}={}", k, v));
    }
    for flag in &config.flags {
        builder = builder.clang_arg(flag);
    }
    builder = builder
        .clang_args(&["-x", "c++", "-std=gnu++11"])
        .use_core()
        .layout_tests(false)
        .parse_callbacks(Box::new(CargoCallbacks::new()));

    for item in &config.bindgen_arduino_helper_lists.allowlist_function {
        builder = builder.allowlist_function(item);
        if debug_print_arduino_helper() { println!("cargo:warning=*******************   allowList   ******************** {}", item.to_string()); }
    }
/*
    for item in &config.bindgen_lists.allowlist_type {
        builder = builder.allowlist_type(item);
    }
    for item in &config.bindgen_lists.blocklist_function {
        builder = builder.blocklist_function(item);
    }
    for item in &config.bindgen_lists.blocklist_type {
        builder = builder.blocklist_type(item);
    }
*/
    for include_dir in config.include_arduino_helper_dirs() {
        builder = builder.clang_arg(&format!("-I{}", include_dir.to_string_lossy()));
        if debug_print_arduino_helper() { println!("cargo:warning=******************   include_dir   ******************* {}", include_dir.to_string_lossy().to_string()); }
    }
    for header in config.bindgen_arduino_helper_headers() {
        builder = builder.header(header.to_string_lossy());
    }
    builder
}

fn generate_arduino_helper_bindings(config: &Config) {
    let bindings: Bindings = configure_bindgen_for_arduino_helper(&config)
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate local bindings");
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("clibs_arduino_helper_bindings.rs");
    bindings
        .write_to_file(project_root)
        .expect("Couldn't write local bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed={}", CONFIG_FILE);
    let config_string = std::fs::read_to_string(CONFIG_FILE)
        .unwrap_or_else(|e| panic!("Unable to read {} file: {}", CONFIG_FILE, e));
    let config: Config = serde_yaml::from_str(&config_string)
        .unwrap_or_else(|e| panic!("Unable to parse {} file: {}", CONFIG_FILE, e));

    println!("Arduino configuration: {:#?}", config);

    compile_arduino(&config);
    generate_bindings(&config);

    compile_arduino_helper_lib(&config);
    generate_arduino_helper_bindings(&config);

    //build_arduino_helper();

}
