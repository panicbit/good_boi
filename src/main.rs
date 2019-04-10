use std::io::Write;
use std::error::Error;
use std::collections::HashSet;
pub use crate::core::Core;

mod instruction;
mod core;

fn main() {
    // let data = include_bytes!("../gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let data = include_bytes!("../gb-test-roms/cpu_instrs/individual/01-special.gb");

    let mut core = Core::new(data.to_vec());

    core.print_state();

    let mut breakpoints = HashSet::<u16>::new();

    loop {
        let mut input = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.split_whitespace().collect::<Vec<&str>>();

        let result = match &*input {
            ["b", addr] => add_breakpoint(&mut breakpoints, addr),
            ["p", addr] => print_mem_u8(&core, addr),
            ["r"] => loop {
                core.step();
                core.print_state();
                if breakpoints.contains(&core.pc()) {
                    println!("Stopping at breakpoint.");
                    break Ok(());
                }
            },
            ["r", addr] => run_until(&mut core, addr),
            [] | ["n"] => {
                core.step();
                core.print_state();
                Ok(())
            },
            ["w", addr, value] => write_mem_u8(&mut core, addr, value),
            _ => Err("Unknown command".into()),
        };

        if let Err(err) = result {
            println!("❌ {}", err);
        }
    }
}

fn add_breakpoint(breakpoints: &mut HashSet<u16>, addr: &str) -> Result<(), Box<Error>> {
    let addr = u16::from_str_radix(addr, 16)?;

    breakpoints.insert(addr);

    Ok(())
}

fn run_until(core: &mut Core, addr: &str) -> Result<(), Box<Error>> {
    let addr = u16::from_str_radix(addr, 16)?;

    loop {
        core.step();

        if addr == core.pc() {
            break;
        }
    }

    core.print_state();
    Ok(())
}

fn print_mem_u8(core: &Core, addr: &str) -> Result<(), Box<Error>> {
    let addr = u16::from_str_radix(addr, 16)?;
    let value = core.peek_mem_u8(addr);

    println!("[{:04X}] = {:02X}", addr, value);

    Ok(())
}

fn write_mem_u8(core: &mut Core, addr: &str, value: &str) -> Result<(), Box<Error>> {
    let addr = u16::from_str_radix(addr, 16)?;
    let value = u8::from_str_radix(value, 16)?;
    core.write_mem_u8(addr, value);

    dbg!(addr);

    println!("[{:04X}] = {:02X}", addr, value);

    Ok(())
}
