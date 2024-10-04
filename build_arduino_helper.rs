use std::path::PathBuf;
// suppress the CargoCallback deprecated warning for now
#[allow(deprecated)]
use bindgen::{Bindings, Builder, CargoCallbacks};
use cc::Build;
use crate::{Config, CONFIG_FILE, add_source_file, files_in_folder};


fn debug_print_arduino_helper() -> bool {
    false
}

impl Config {
    fn local_path(&self) -> PathBuf {
        let expanded = "./";
        let arduino_home_path = PathBuf::from(expanded);
        arduino_home_path.join("src_c")
    }

    fn include_arduino_helper_dirs(&self) -> Vec<PathBuf> {
        let expanded = envmnt::expand("./src_c", None);
        let arduino_home_path = PathBuf::from(expanded);

        let mut result = self.arduino_include_dirs();
        result.extend(vec![arduino_home_path]);
        result
    }

    fn cpp_arduino_helper_files(&self) -> Vec<PathBuf> {
        self.project_arduino_helper_files("*.cpp")
    }

    // c_arduino_helper_files is not used until we need to compile c source
    //fn c_arduino_helper_files(&self) -> Vec<PathBuf> {
    //    self.project_arduino_helper_files("*.c")
    //}

    fn project_arduino_helper_files(&self, patten: &str) -> Vec<PathBuf> {
        let result = files_in_folder(self.local_path().to_string_lossy().as_ref(), patten);
        if debug_print_arduino_helper() { println!("cargo:warning=********************   project_arduino_files  ******************** {}", self.local_path().to_string_lossy()); }

        //        let libraries = self.arduino_libraries_path();

        //        let pattern = format!("**/{}", patten);
        //        for library in libraries {
        //            let lib_sources = files_in_folder(library.to_string_lossy().as_ref(), &pattern);
        //            println!("cargo:warning=********************   libraries  ******************** {}", library.to_string_lossy());
        //            result.extend(lib_sources);
        //        }

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
            if debug_print_arduino_helper() { println!("cargo:warning=********************   header    ********************* {}", header.to_string_lossy()); }
        }

        result
    }

}

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

pub fn build_arduino_helper() {
    println!("cargo:rerun-if-changed={}", CONFIG_FILE);
    let config_string = std::fs::read_to_string(CONFIG_FILE)
        .unwrap_or_else(|e| panic!("Unable to read {} file: {}", CONFIG_FILE, e));
    let config: Config = serde_yaml::from_str(&config_string)
        .unwrap_or_else(|e| panic!("Unable to parse {} file: {}", CONFIG_FILE, e));

    println!("Arduino configuration: {:#?}", config);

    compile_arduino_helper_lib(&config);
    generate_arduino_helper_bindings(&config);

}