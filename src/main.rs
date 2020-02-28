mod dna;
mod fib;
mod hamm;
mod prot;
mod revc;
mod rna;

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
}
