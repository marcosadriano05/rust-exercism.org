#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String,
}

const DNA: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA: [char; 4] = ['C', 'G', 'A', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        return match dna.chars().position(|char| !DNA.contains(&char)) {
            Some(pos) => Err(pos),
            None => Ok(Dna {
                nucleotides: dna.to_string(),
            }),
        };
    }

    pub fn into_rna(self) -> Rna {
        let mut rna: String = "".to_string();
        self.nucleotides.chars().for_each(|char| {
            let position = DNA.iter().position(|item| item == &char).unwrap();
            rna.push(RNA.get(position).unwrap().clone())
        });
        Rna { nucleotides: rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        return match rna.chars().position(|char| !RNA.contains(&char)) {
            Some(pos) => Err(pos),
            None => Ok(Rna {
                nucleotides: rna.to_string(),
            }),
        };
    }
}
