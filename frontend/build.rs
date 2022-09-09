use static_files::NpmBuild;
use std::path::PathBuf;

const PACKAGE_JSON_DIR: &str = ".";
const TARGET_DIR: &str = "build";

fn change_detection() {
    use ::change_detection::{
        path_matchers::{any, equal, func, starts_with},
        ChangeDetection,
    };

    let package_json_dir = PathBuf::from(PACKAGE_JSON_DIR);
    #[allow(unused)]
    let target_dir = package_json_dir.join(TARGET_DIR);
    let exclude_filter = any!(
        equal(package_json_dir.clone()),
        starts_with(package_json_dir.join("node_modules")),
        starts_with(package_json_dir.join(".svelte-kit")),
        equal(package_json_dir.join("package.json")),
        equal(package_json_dir.join("package-lock.json")),
        starts_with(target_dir),
        func(move |p| { p.is_file() && p.parent() != Some(package_json_dir.as_path()) })
    );

    {
        let change_detection = ChangeDetection::exclude(exclude_filter);

        change_detection.path(PACKAGE_JSON_DIR).generate();
    }
}

fn main() -> std::io::Result<()> {
    change_detection();

    NpmBuild::new(PACKAGE_JSON_DIR)
        .install()?
        .run("build")?
        .target(TARGET_DIR)
        .to_resource_dir()
        .build()
}
