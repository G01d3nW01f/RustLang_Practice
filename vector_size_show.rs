fn print_info(mem: &[u8]) {
    let addr = (&mem[0] as *const u8) as u64;
    let mut bound: u64 = 1;
    while addr & bound == 0 { bound <<= 1; }
    println!("size: {:>10}  addr: 0x{:>012x}  bound: {:>7}", mem.len(), addr, bound);
}

fn main () {
    const ONEGB:usize = 1024*1024*1024;
    let mut size:usize = 2;
    while size <= ONEGB {
        let mut mem0: Vec<u8> = Vec::with_capacity(size);
        unsafe { mem0.set_len(size); }
        print_info(&mem0);
        let mut mem1: Vec<u8> = Vec::with_capacity(size);
        unsafe { mem1.set_len(size); }
        print_info(&mem1);
        size *= 2;
    }
}
