use haybale::*;
use std::path::Path;

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_project() -> Project {
    let modname = "tests/bcfiles/linkedlist.bc";
    Project::from_bc_path(&Path::new(modname))
        .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e))
}

#[test]
fn simple_linked_list() {
    let funcname = "simple_linked_list";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}


#[test]
fn indirectly_recursive_type() {
    let funcname = "indirectly_recursive_type";
    init_logging();
    let proj = get_project();
    let ctx = z3::Context::new(&z3::Config::new());
    let args = find_zero_of_func(&ctx, funcname, &proj, Config::default()).expect("Failed to find zero of the function");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], SolutionValue::I32(3));
}
