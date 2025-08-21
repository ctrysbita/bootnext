use efivar::efi::{Variable, VariableFlags};
use std::error::Error;

const TARGET_OS: Option<&'static str> = option_env!("TARGET_OS");

fn main() -> Result<(), Box<dyn Error>> {
    let target_os = TARGET_OS.unwrap_or("ubuntu").to_lowercase();

    println!("Searching for {} boot entries...", target_os);
    let mut manager = efivar::system();

    let mut found = None;
    for (entry_result, var) in manager.get_boot_entries()? {
        if let Ok(entry) = entry_result {
            let desc = &entry.entry.description;
            if desc.to_lowercase().contains(&target_os) {
                println!("Boot entry found: {} ({})", var.name(), desc);
                found = Some((entry.id, desc.clone()));
                break;
            }
        }
    }

    let (target_boot_id, target_desc) = match found {
        Some((id, desc)) => (id, desc),
        None => {
            eprintln!("No {} boot entry found", target_os);
            return Err(format!("No {} boot entry found", target_os).into());
        }
    };

    let boot_next_var = Variable::new("BootNext");
    let attributes = VariableFlags::NON_VOLATILE
        | VariableFlags::BOOTSERVICE_ACCESS
        | VariableFlags::RUNTIME_ACCESS;
    let value_bytes = target_boot_id.to_le_bytes();

    println!(
        "Setting BootNext to {:04X} ({})...",
        target_boot_id, target_desc
    );
    manager.write(&boot_next_var, attributes, &value_bytes)?;
    println!("Successfully set next boot to {}", target_os);
    Ok(())
}
