mod dna;
mod fib;
mod hamm;
mod revc;
mod rna;

fn main() {
    dna::count("data/rosalind_dna.txt");
    rna::transcribe("data/rosalind_rna.txt");
    revc::complement("data/rosalind_revc.txt");
    fib::run("data/rosalind_fib.txt");
    println!("\nHAMM:");
    hamm::run("data/rosalind_hamm.txt");
}
