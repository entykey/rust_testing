#![allow(unused)]
// retrieving available resources:
use num_cpus;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};

fn main() {

    // available resources
    let total_cores = num_cpus::get_physical();
    let total_threads = if total_cores > 0 {
        total_cores * 2
    } else {
        num_cpus::get()
    };

    println!("Total Cores: {}", total_cores);
    println!("Total Threads: {}", total_threads);

    // Create a new System instance to get the RAM info
    let mut system = System::new_all();

    // First we update all information of our `System` struct.
    system.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in system.disks() {
        println!("{:?}", disk);
    }

    // Retrieve the total RAM in megabytes
    let total_ram_mb = system.total_memory() / (1024 * 1024);
    println!("Total RAM: {} MB", total_ram_mb);

    println!("=> system:");
    // RAM and swap information:
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
    println!("total memory: {} bytes", total_memory);
    println!("used memory : {} bytes", used_memory);
    println!("memory used : {:.2}%", memory_percentage);
    println!("total swap  : {} bytes", system.total_swap());
    println!("used swap   : {} bytes", system.used_swap());

    // Display system information:
    println!("System name:             {:?}", system.name());
    println!("System kernel version:   {:?}", system.kernel_version());
    println!("System OS version:       {:?}", system.os_version());
    println!("System host name:        {:?}", system.host_name());

    // Number of CPUs (including physical & nonphysical):
    println!("NB CPUs: {}", system.cpus().len());


}
