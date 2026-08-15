use crate::science::crucible::Gene;

pub fn decode_genes_to_language(genes: &[Gene]) -> String {
    let mut output = String::new();
    let consonants = ["K", "T", "P", "S", "M", "N", "L", "R", "F", "V"];
    let vowels = ["A", "E", "I", "O", "U", "AE", "OU", "EI"];
    
    for (i, gene) in genes.iter().enumerate() {
        let v = gene.current_value;
        // Use the value to pick a consonant and a vowel
        let c_idx = ((v * 100.0) as usize) % consonants.len();
        // Use the difference from the previous gene to pick the vowel (capturing the energy delta)
        let delta = if i > 0 { (gene.current_value - genes[i-1].current_value).abs() } else { v };
        let v_idx = ((delta * 100.0) as usize) % vowels.len();
        
        output.push_str(consonants[c_idx]);
        output.push_str(vowels[v_idx]);
        
        if (v * 10.0) as usize % 3 == 0 {
            output.push_str(" "); // Random spaces for words
        }
    }
    output.trim().to_string()
}
