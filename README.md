# Mixteco Corpus Toolkit

A Rust toolkit for exploring, analyzing, and extracting lexical information from Mixtec-language corpora.

The project was originally developed to work with the parallel Spanish–Mixtec corpus published in:

> *Parallel Corpus for Indigenous Language Translation: Spanish-Mazatec and Spanish-Mixtec*

and has evolved into a corpus-analysis and lexicon-extraction toolkit.

---

## Features

### Orthography

- Orthographic normalization
- Search-form normalization
- Glottal-stop handling

### Corpus Analysis

- Corpus statistics
- Vocabulary statistics
- Frequency analysis
- Top-N word lists

### Corpus Search

- Concordance search
- Normalization-aware search
- Token-aware matching
- KWIC (Key Word In Context)

### Collocations

- Left-context analysis
- Right-context analysis

### Exports

- Vocabulary export (CSV)
- Context export (CSV)
- KWIC export (CSV)

### Dictionary Extraction

- Experimental bilingual lexicon extraction from aligned Spanish–Mixtec corpora

---

## Workspace Layout

```text
crates/
├── mixteco-cli
├── mixteco-core
├── mixteco-corpus
├── mixteco-dictionary
├── mixteco-orthography
├── mixteco-phonology
├── mixteco-search
└── mixteco-xtn
```

---

## Building

Build the workspace:

```bash
cargo build
```

Run formatting:

```bash
cargo fmt
```

Run static analysis:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run tests:

```bash
cargo test
```

---

## Corpora

The repository currently includes:

```text
Spanish-Mazatec and Spanish-Mixtec/

├── es-maq.csv
├── es-xtnNT(mextec) copy.csv
├── maqNT-es(Mazatec).csv
├── xtnNT-es(mextec).csv
└── README.md
```

The toolkit has been tested on the Spanish–Mixtec corpus:

```text
xtnNT-es(mextec).csv
```

containing approximately:

- 15,597 sentence pairs
- 450,000+ Mixtec tokens
- 16,818 unique word forms

---

## Usage

### Corpus Statistics

```bash
cargo run -p mixteco-cli -- stats corpus.csv
```

Example:

```text
Sentence pairs: 15597
Source words : 450239
Target words : 365598
```

---

### Vocabulary Statistics

```bash
cargo run -p mixteco-cli -- vocab corpus.csv
```

Example:

```text
Vocabulary Statistics

Unique words         : 16818
Words occurring once : 8963
Hapax rate           : 53.29%

Most common word     : ni
Frequency            : 24379
```

---

### Top Words

```bash
cargo run -p mixteco-cli -- top corpus.csv
```

or

```bash
cargo run -p mixteco-cli -- top corpus.csv --limit 100
```

Example:

```text
 1. ni      24379
 2. te      16059
 3. ja      15763
```

---

### Concordance Search

```bash
cargo run -p mixteco-cli -- concordance corpus.csv ñayiu
```

Features:

- normalization-aware search
- token-aware matching
- bilingual output

---

### KWIC (Key Word In Context)

```bash
cargo run -p mixteco-cli -- kwic corpus.csv ñayiu
```

Example:

```text
... ndiꞌi [ñayiu] janaꞌa ja ka okuu ...
... nuu [ñayiu] judio chi ni ka ...
```

Custom context width:

```bash
cargo run -p mixteco-cli -- kwic corpus.csv ñayiu --width 10
```

---

### Context Analysis

```bash
cargo run -p mixteco-cli -- contexts corpus.csv ñayiu
```

Example:

```text
Left contexts

nuu                  519
ndiꞌi                455
in                   300

Right contexts

ka                  1024
ja                   379
ñuu                  221
```

---

### Lexicon Extraction

Experimental extraction of translation candidates from aligned sentence pairs.

```bash
cargo run -p mixteco-cli -- lexicon corpus.csv ñayiu
```

Example:

```text
Translation candidates for: ñayiu

hombre               408
hombres              319
pueblo               153
judíos               157
```

Example:

```bash
cargo run -p mixteco-cli -- lexicon corpus.csv tnoo
```

Example output:

```text
palabra              222
```

---

## CSV Exports

### Export Vocabulary

```bash
cargo run -p mixteco-cli -- export-vocab corpus.csv vocab.csv
```

Output:

```csv
word,count
ni,24379
te,16059
ja,15763
```

---

### Export Contexts

```bash
cargo run -p mixteco-cli -- export-contexts corpus.csv ñayiu contexts.csv
```

Output:

```csv
direction,word,count
left,nuu,519
left,ndiꞌi,455
right,ka,1024
```

---

### Export KWIC

```bash
cargo run -p mixteco-cli -- export-kwic corpus.csv ñayiu kwic.csv
```

Output:

```csv
left,keyword,right,translation
"yaꞌa ka kuu sivi ndiꞌi","ñayiu","janaꞌa ja ka okuu","..."
```

---

## Dataset Citation

If you use the Spanish–Mazatec or Spanish–Mixtec corpus, please cite the paper below.

### BibTeX

```bibtex
@article{tonja2023parallel,
  title={Parallel Corpus for Indigenous Language Translation: Spanish-Mazatec and Spanish-Mixtec},
  author={Tonja, Atnafu Lambebo and Maldonado-Sifuentes, Christian and Castillo, David Alejandro Mendoza and Kolesnikova, Olga and Castro-Sánchez, Noé and Sidorov, Grigori and Gelbukh, Alexander},
  journal={arXiv preprint arXiv:2305.17404},
  year={2023}
}
```

### Reference

Tonja, A. L., Maldonado-Sifuentes, C., Castillo, D. A. M., Kolesnikova, O., Castro-Sánchez, N., Sidorov, G., & Gelbukh, A. (2023). *Parallel Corpus for Indigenous Language Translation: Spanish-Mazatec and Spanish-Mixtec.* arXiv preprint arXiv:2305.17404.

### Related resource

- [Machine Translation Resources](https://github.com/atnafuatx/Machine-Translation-Resources) — a collection of machine-translation resources.
