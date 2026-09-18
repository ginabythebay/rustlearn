pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut accum = Vec::new();
    let mut remaining: &str = rna;
    let mut stop = false;
    while remaining.len() > 2 && !stop {
        let next = &remaining[..3];
        remaining = &remaining[3..];
        match next {
            "AUG" => accum.push("Methionine"),
            "UUU" | "UUC" => accum.push("Phenylalanine"),
            "UUA" | "UUG" => accum.push("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => accum.push("Serine"),
            "UAU" | "UAC" => accum.push("Tyrosine"),
            "UGU" | "UGC" => accum.push("Cysteine"),
            "UGG" => accum.push("Tryptophan"),
            "UAA" | "UAG" | "UGA" => stop = true,
            _ => return None,
        }
    }

    match remaining.len() {
        1 | 2 => None,
        _ => Some(accum),
    }
}
