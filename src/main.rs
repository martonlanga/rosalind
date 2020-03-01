mod cons;
mod dna;
mod fib;
mod fibd;
mod gc;
mod hamm;
mod iprb;
mod prot;
mod revc;
mod rna;
mod subs;

fn main() {
    println!("DNA:");
    dna::count("data/rosalind_dna.txt");
    println!("\nRNA:");
    rna::transcribe("data/rosalind_rna.txt");
    println!("\nREVC:");
    revc::complement("data/rosalind_revc.txt");
    println!("\nFIB:");
    fib::run("data/rosalind_fib.txt");
    println!("\nHAMM:");
    hamm::run("data/rosalind_hamm.txt");
    println!("\nPROT:");
    prot::run("data/rosalind_prot.txt");
    println!("\nFIBD:");
    fibd::run("data/rosalind_fibd.txt");
    println!("\nGC:");
    gc::run("data/rosalind_gc.txt");
    println!("\nIPRB:");
    iprb::run("data/rosalind_iprb.txt");
    println!("\nSUBS:");
    subs::run("data/rosalind_subs.txt");
    println!("\nCONS:");
    cons::run("data/rosalind_cons.txt");
}
