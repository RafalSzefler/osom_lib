const OSOM_RUNNING_ENVS: &[&str] = &["", "github"];

fn main() {
    generate_cfg_directive();
    generate_env_directive();
}

fn generate_cfg_directive() {
    let mut osom_running_envs = OSOM_RUNNING_ENVS.iter().map(|s| format!("\"{}\"", s));
    let first = osom_running_envs.next().unwrap();
    let mut directive = "cargo::rustc-check-cfg=cfg(osom_running_env, values(".to_owned() + &first;
    for item in osom_running_envs {
        directive.push(',');
        directive.push_str(&item);
    }
    directive.push_str("))");
    println!("{directive}");
}

fn generate_env_directive() {
    println!("cargo:rerun-if-env-changed=OSOM_RUNNING_ENV");
    let running_env = std::env::var("OSOM_RUNNING_ENV").unwrap_or_default();
    if !OSOM_RUNNING_ENVS.contains(&&*running_env) {
        let mut osom_running_envs = OSOM_RUNNING_ENVS
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| format!("\"{}\"", s));
        let mut envs = osom_running_envs.next().unwrap();
        for item in osom_running_envs {
            envs.push(',');
            envs.push_str(&item);
        }
        panic!("Invalid OSOM_RUNNING_ENV: {running_env}. Available values: {envs}");
    }

    if running_env.is_empty() {
        return;
    }
    println!("cargo::rustc-cfg=osom_running_env=\"{}\"", &running_env);
}
