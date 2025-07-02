use efivar::efi::{Variable, VariableFlags};
use std::error::Error;

#[cfg(feature = "ubuntu")]
const TARGET_OS: &str = "ubuntu";
#[cfg(feature = "windows")]
const TARGET_OS: &str = "windows";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Searching for {} boot entries...", TARGET_OS);
    let mut manager = efivar::system();

    let mut found = None;
    for (entry_result, var) in manager.get_boot_entries()? {
        if let Ok(entry) = entry_result {
            let desc = &entry.entry.description;
            if desc.to_lowercase().contains(TARGET_OS) {
                println!("Boot entry found: {} ({})", var.name(), desc);
                found = Some((entry.id, desc.clone()));
                break;
            }
        }
    }

    let (target_boot_id, target_desc) = match found {
        Some((id, desc)) => (id, desc),
        None => {
            eprintln!("No {} boot entry found", TARGET_OS);
            return Err(format!("No {} boot entry found", TARGET_OS).into());
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
    println!("Successfully set next boot to {}", TARGET_OS);
    Ok(())
}
