fn main() {
    println!("{}", num_cpus::get());
    println!(
        "{}:{}",
        sysinfo::System::name().unwrap(),
        sysinfo::System::cpu_arch()
    );
}
