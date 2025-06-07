use copy_glob::{CopyGlobOptionsBuilder, copy_glob_with, get_root_path, get_target_folder};

fn main() {
    let output = get_target_folder();
    let resources = get_root_path().join("resources");
    let copy_options = CopyGlobOptionsBuilder::new()
        .set_root_path(resources)
        .build();
    copy_glob_with("*", output, &copy_options);
}