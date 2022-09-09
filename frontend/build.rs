use static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    NpmBuild::new(".")
        .install()?
        .run("build")?
        .target("build")
        .change_detection()
        .to_resource_dir()
        .build()
}
