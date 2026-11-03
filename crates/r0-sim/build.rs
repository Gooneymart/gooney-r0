fn main() {
    // Rebuild if C++ bridge source changes
    println!("cargo:rerun-if-changed=cpp/sim_bridge.cpp");

    // Compile the C++ simulation bridge
    cc::Build::new()
        .cpp(true)
        .file("cpp/sim_bridge.cpp")
        .opt_level(2)
        .compile("sim_bridge");
}
