use super::{AvailableReturnFormats, DataReturnFormats};
use std::fmt::{Display, Formatter};

// WILL HOPEFULLY REPLACE ALL OF THIS ONCE THE MACROS ARE WORKING

/// The `DbfetchDbs` enum is used to specify the databases to be fetched from the `Dbfetch` service.
#[derive(Debug, Clone)]
pub enum DbfetchDbs {
    AlphaFoldDb,
    Cdp,
    ChemblTargets,
    Edam,
    Emdb,
    EnaCoding,
    EnaGeospatial,
    EnaNonCoding,
    EnaRrna,
    EnaSequence,
    EnaSequenceConstructed,
    EnaSequenceConstructedExpanded,
    EnaSva,
    EnsemblGene,
    EnsemblGenomesGene,
    EnsemblGenomesTranscript,
    EnsemblTranscript,
    EpoProteins,
    Hgnc,
    ImgtHlaNucleotideCds,
    ImgtHlaNucleotideGenomic,
    ImgtHlaProtein,
    ImgtLigmDb,
    InterPro,
    IpdKirNucleotideCds,
    IpdKirNucleotideGenomic,
    IpdKirProtein,
    IpdMhcNucleotideCds,
    IpdMhcNucleotideGenomic,
    IpdMhcProtein,
    IpdNhkirNucleotideCds,
    IpdNhkirNucleotideGenomic,
    IpdNhkirProtein,
    Iprmc,
    IprmcUniParc,
    JpoProteins,
    KipoProteins,
    Medline,
    MeropsMp,
    MeropsMpep,
    MeropsMpro,
    PatentDnaNrl1,
    PatentDnaNrl2,
    PatentProteinNrl1,
    PatentProteinNrl2,
    PatentEquivalents,
    Pdb,
    PdbeKb,
    RefSeqNucleotide,
    RefSeqProtein,
    Taxonomy,
    UniParc,
    UniProtKB,
    UniRef100,
    UniRef50,
    UniRef90,
    UniSave,
    UsptoProteins,
}

impl AvailableReturnFormats for DbfetchDbs {
    fn available_return_formats(&self) -> Vec<DataReturnFormats> {
        match self {
            DbfetchDbs::AlphaFoldDb => vec![
                DataReturnFormats::Json,
                DataReturnFormats::Fasta,
                DataReturnFormats::Pdb,
                DataReturnFormats::Mmcif,
            ],
            DbfetchDbs::Cdp => vec![DataReturnFormats::Xml, DataReturnFormats::Fasta],
            DbfetchDbs::ChemblTargets => vec![DataReturnFormats::Fasta],
            DbfetchDbs::Edam => vec![DataReturnFormats::Obo],
            DbfetchDbs::Emdb => vec![DataReturnFormats::Xml],
            DbfetchDbs::EnaCoding => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaGeospatial => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaNonCoding => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaRrna => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaSequence => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaSequenceConstructed => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaSequenceConstructedExpanded => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnaSva => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnsemblGene => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Csv,
                DataReturnFormats::Gff3,
                DataReturnFormats::Gff2,
            ],
            DbfetchDbs::EnsemblGenomesGene => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Csv,
                DataReturnFormats::Gff3,
                DataReturnFormats::Gff2,
            ],
            DbfetchDbs::EnsemblGenomesTranscript => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EnsemblTranscript => vec![DataReturnFormats::Fasta],
            DbfetchDbs::EpoProteins => vec![DataReturnFormats::Fasta],
            DbfetchDbs::Hgnc => vec![DataReturnFormats::Tsv],
            DbfetchDbs::ImgtHlaNucleotideCds => vec![DataReturnFormats::Fasta],
            DbfetchDbs::ImgtHlaNucleotideGenomic => vec![DataReturnFormats::Fasta],
            DbfetchDbs::ImgtHlaProtein => vec![DataReturnFormats::Fasta],
            DbfetchDbs::ImgtLigmDb => vec![DataReturnFormats::Fasta],
            DbfetchDbs::InterPro => vec![DataReturnFormats::Tsv],
            DbfetchDbs::IpdKirNucleotideCds => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdKirNucleotideGenomic => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdKirProtein => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdMhcNucleotideCds => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdMhcNucleotideGenomic => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdMhcProtein => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdNhkirNucleotideCds => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdNhkirNucleotideGenomic => vec![DataReturnFormats::Fasta],
            DbfetchDbs::IpdNhkirProtein => vec![DataReturnFormats::Fasta],
            DbfetchDbs::Iprmc => vec![DataReturnFormats::Gff2],
            DbfetchDbs::IprmcUniParc => vec![DataReturnFormats::Gff2],
            DbfetchDbs::JpoProteins => vec![DataReturnFormats::Fasta],
            DbfetchDbs::KipoProteins => vec![DataReturnFormats::Fasta],
            DbfetchDbs::Medline => vec![DataReturnFormats::Xml],
            DbfetchDbs::MeropsMp => vec![DataReturnFormats::Fasta],
            DbfetchDbs::MeropsMpep => vec![DataReturnFormats::Fasta],
            DbfetchDbs::MeropsMpro => vec![DataReturnFormats::Fasta],
            DbfetchDbs::PatentDnaNrl1 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::PatentDnaNrl2 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::PatentProteinNrl1 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::PatentProteinNrl2 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::PatentEquivalents => vec![DataReturnFormats::PatentEquivalents],
            DbfetchDbs::Pdb => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Pdb,
                DataReturnFormats::Mmcif,
            ],
            DbfetchDbs::PdbeKb => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Pdb,
                DataReturnFormats::Mmcif,
            ],
            DbfetchDbs::RefSeqNucleotide => {
                vec![DataReturnFormats::Json, DataReturnFormats::Fasta]
            }
            DbfetchDbs::RefSeqProtein => vec![DataReturnFormats::Fasta],
            DbfetchDbs::Taxonomy => vec![DataReturnFormats::Xml],
            DbfetchDbs::UniParc => vec![DataReturnFormats::Fasta],
            DbfetchDbs::UniProtKB => vec![DataReturnFormats::Gff3, DataReturnFormats::Fasta],
            DbfetchDbs::UniRef100 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::UniRef50 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::UniRef90 => vec![DataReturnFormats::Fasta],
            DbfetchDbs::UniSave => vec![DataReturnFormats::Fasta],
            DbfetchDbs::UsptoProteins => vec![DataReturnFormats::Fasta],
        }
    }
}

impl Display for DbfetchDbs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbfetchDbs::AlphaFoldDb => write!(f, "afdb"),
            DbfetchDbs::Cdp => write!(f, "cdp"),
            DbfetchDbs::ChemblTargets => write!(f, "chembl"),
            DbfetchDbs::Edam => write!(f, "edam"),
            DbfetchDbs::Emdb => write!(f, "emdb"),
            DbfetchDbs::EnaCoding => write!(f, "ena_coding"),
            DbfetchDbs::EnaGeospatial => write!(f, "ena_geospatial"),
            DbfetchDbs::EnaNonCoding => write!(f, "ena_noncoding"),
            DbfetchDbs::EnaRrna => write!(f, "ena_rrna"),
            DbfetchDbs::EnaSequence => write!(f, "ena_sequence"),
            DbfetchDbs::EnaSequenceConstructed => write!(f, "ena_sequence_con"),
            DbfetchDbs::EnaSequenceConstructedExpanded => write!(f, "ena_sequence_conexp"),
            DbfetchDbs::EnaSva => write!(f, "ena_sva"),
            DbfetchDbs::EnsemblGene => write!(f, "ensemblgene"),
            DbfetchDbs::EnsemblGenomesGene => write!(f, "ensemblgenomesgene"),
            DbfetchDbs::EnsemblGenomesTranscript => write!(f, "ensemblgenomestranscript"),
            DbfetchDbs::EnsemblTranscript => write!(f, "ensembltranscript"),
            DbfetchDbs::EpoProteins => write!(f, "epo_prt"),
            DbfetchDbs::Hgnc => write!(f, "hgnc"),
            DbfetchDbs::ImgtHlaNucleotideCds => write!(f, "imgthlacds"),
            DbfetchDbs::ImgtHlaNucleotideGenomic => write!(f, "imgthlagen"),
            DbfetchDbs::ImgtHlaProtein => write!(f, "imgthlapro"),
            DbfetchDbs::ImgtLigmDb => write!(f, "imgtligm"),
            DbfetchDbs::InterPro => write!(f, "interpro"),
            DbfetchDbs::IpdKirNucleotideCds => write!(f, "ipdkircds"),
            DbfetchDbs::IpdKirNucleotideGenomic => write!(f, "ipdkirgen"),
            DbfetchDbs::IpdKirProtein => write!(f, "ipdkirpro"),
            DbfetchDbs::IpdMhcNucleotideCds => write!(f, "ipdmhccds"),
            DbfetchDbs::IpdMhcNucleotideGenomic => write!(f, "ipdmhcgen"),
            DbfetchDbs::IpdMhcProtein => write!(f, "ipdmhcpro"),
            DbfetchDbs::IpdNhkirNucleotideCds => write!(f, "ipdnhkircds"),
            DbfetchDbs::IpdNhkirNucleotideGenomic => write!(f, "ipdnhkirgen"),
            DbfetchDbs::IpdNhkirProtein => write!(f, "ipdnhkirpro"),
            DbfetchDbs::Iprmc => write!(f, "iprmc"),
            DbfetchDbs::IprmcUniParc => write!(f, "iprmcuniparc"),
            DbfetchDbs::JpoProteins => write!(f, "jpo_prt"),
            DbfetchDbs::KipoProteins => write!(f, "kipo_prt"),
            DbfetchDbs::Medline => write!(f, "medline"),
            DbfetchDbs::MeropsMp => write!(f, "mp"),
            DbfetchDbs::MeropsMpep => write!(f, "mpep"),
            DbfetchDbs::MeropsMpro => write!(f, "mpro"),
            DbfetchDbs::PatentDnaNrl1 => write!(f, "nrnl1"),
            DbfetchDbs::PatentDnaNrl2 => write!(f, "nrnl2"),
            DbfetchDbs::PatentProteinNrl1 => write!(f, "nrpl1"),
            DbfetchDbs::PatentProteinNrl2 => write!(f, "nrpl2"),
            DbfetchDbs::PatentEquivalents => write!(f, "patent_equivalents"),
            DbfetchDbs::Pdb => write!(f, "pdb"),
            DbfetchDbs::PdbeKb => write!(f, "pdbekb"),
            DbfetchDbs::RefSeqNucleotide => write!(f, "refseqn"),
            DbfetchDbs::RefSeqProtein => write!(f, "refseqp"),
            DbfetchDbs::Taxonomy => write!(f, "taxonomy"),
            DbfetchDbs::UniParc => write!(f, "uniparc"),
            DbfetchDbs::UniProtKB => write!(f, "uniprotkb"),
            DbfetchDbs::UniRef100 => write!(f, "uniref100"),
            DbfetchDbs::UniRef50 => write!(f, "uniref50"),
            DbfetchDbs::UniRef90 => write!(f, "uniref90"),
            DbfetchDbs::UniSave => write!(f, "unisave"),
            DbfetchDbs::UsptoProteins => write!(f, "uspto_prt"),
        }
    }
}
