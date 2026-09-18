pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut accum = Vec::new();
    let mut remaining: &str = rna;
    loop {
        let result = remaining.get(..3);
        if let Some(next) = result {
            remaining = &remaining[3..];
            match next {
                "AUG" => accum.push("Methionine"),
                "UUU" | "UUC" => accum.push("Phenylalanine"),
                "UUA" | "UUG" => accum.push("Leucine"),
                "UCU" | "UCC" | "UCA" | "UCG" => accum.push("Serine"),
                "UAU" | "UAC" => accum.push("Tyrosine"),
                "UGU" | "UGC" => accum.push("Cysteine"),
                "UGG" => accum.push("Tryptophan"),
                "UAA" | "UAG" | "UGA" => break,
                _ => return None,
            }
        } else {
            break;
        }
    }

    match remaining.len() {
        1 | 2 => None,
        _ => Some(accum),
    }
}
