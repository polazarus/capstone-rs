extern crate macho;
extern crate capstone;
use std::env;
use std::fs;
use std::io::Read;
use std::process;

fn main() {
    let cs = capstone::Capstone::new(capstone::CsArch::ARCH_X86,
                                     capstone::CsMode::MODE_64).ok().unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let mut fh = fs::File::open(&args[1]).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = fh.read_to_end(&mut buf);

    if let Some(header) = macho::MachHeader::parse(&buf[..]) {
        // Find the text segment
        for segment in header.segments {
            if segment.segname == "__TEXT" {
                for section in segment.sections {
                    if section.sectname == "__text" {
                        let text = &buf[section.offset as usize .. (section.offset as u64 + section.size) as usize];
                        match cs.disasm(text, section.addr, 0) {
                            Ok(insns) => {
                                println!("Got {} instructions", insns.len());

                                for i in insns.iter() {
                                    println!("{}", i);
                                }
                            },
                            Err(err) => {
                                println!("Error: {}", err);
                                process::exit(1);
                            }
                        }
                        return;
                    }
                }
            }
        }
        panic!("No __TEXT segment");
    }
}
