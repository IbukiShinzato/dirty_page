use memmap::Mmap;
use std::thread;
use std::time::{Duration, Instant};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let one_mb = 1024 * 1024; 

    if let Some(file_name) = args.get(1) {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_name)?;

        let mut mmap = unsafe { Mmap::map(&file)?.make_mut()? };
        let total_size_mb = mmap.len() / one_mb;
        println!("Mmap Success: Address={:?}, Size={} MB", mmap.as_ptr(), total_size_mb);

        let start_time = Instant::now();

        for (i, byte) in mmap.iter_mut().enumerate() {
            *byte = 1; 

            if i > 0 && i % one_mb == 0 {
                let current_mb = i / one_mb;
                let elapsed = start_time.elapsed().as_secs_f32();
                
                println!("Dirtying: {:>4} / {} MB (Elapsed: {:.1}s)", current_mb, total_size_mb, elapsed);
                
                thread::sleep(Duration::from_millis(10));
            }
        }
        println!("Done! All pages are now Dirty.");

    } else {
        eprintln!("Usage: cargo run -- <file_name>");
    }

    Ok(())
}
