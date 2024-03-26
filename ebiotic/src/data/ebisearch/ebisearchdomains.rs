use std::fmt::Display;

use super::{AvailableReturnFormats, DataReturnFormats};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EbiSearchDomains {
    All,
    Uniprot,
    Ena,
    Embl,
    ArrayExpress,
    ExpressionAtlas,
    BioModels,
    BioSamples,
    ChEMBL,
    ComplexPortal,
    EGA,
    Ensembl,
    EnsemblGenomes,
    EuropePMC,
    GeneExpressionAtlas,
    MetaboLights,
    PDBe,
    PRIDE,
    Reactome,
    SequenceReadArchive,
    UniParc,
    UniProt,
    UniRef,
}

impl Display for EbiSearchDomains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EbiSearchDomains::All => write!(f, "/"),
            EbiSearchDomains::Uniprot => write!(f, "uniprot"),
            EbiSearchDomains::Ena => write!(f, "ena"),
            EbiSearchDomains::Embl => write!(f, "embl"),
            EbiSearchDomains::ArrayExpress => write!(f, "arrayexpress"),
            EbiSearchDomains::ExpressionAtlas => write!(f, "expressionatlas"),
            EbiSearchDomains::BioModels => write!(f, "biomodels"),
            EbiSearchDomains::BioSamples => write!(f, "biosamples"),
            EbiSearchDomains::ChEMBL => write!(f, "chembl"),
            EbiSearchDomains::ComplexPortal => write!(f, "complexportal"),
            EbiSearchDomains::EGA => write!(f, "ega"),
            EbiSearchDomains::Ensembl => write!(f, "ensembl"),
            EbiSearchDomains::EnsemblGenomes => write!(f, "ensemblgenomes"),
            EbiSearchDomains::EuropePMC => write!(f, "europepmc"),
            EbiSearchDomains::GeneExpressionAtlas => write!(f, "geneexpressionatlas"),
            EbiSearchDomains::MetaboLights => write!(f, "metabolights"),
            EbiSearchDomains::PDBe => write!(f, "pdbe"),
            EbiSearchDomains::PRIDE => write!(f, "pride"),
            EbiSearchDomains::Reactome => write!(f, "reactome"),
            EbiSearchDomains::SequenceReadArchive => write!(f, "sequencereadarchive"),
            EbiSearchDomains::UniParc => write!(f, "uniparc"),
            EbiSearchDomains::UniProt => write!(f, "uniprot"),
            EbiSearchDomains::UniRef => write!(f, "uniref"),
        }
    }
}

// TODO - need to check these are correct, i just used copilot LMAO
impl AvailableReturnFormats for EbiSearchDomains {
    fn available_return_formats(&self) -> Vec<DataReturnFormats> {
        match self {
            EbiSearchDomains::All => vec![DataReturnFormats::Json],
            EbiSearchDomains::Uniprot => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Json,
                DataReturnFormats::Xml,
                DataReturnFormats::Csv,
                DataReturnFormats::Tsv,
            ],
            EbiSearchDomains::Ena => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Json,
                DataReturnFormats::Xml,
                DataReturnFormats::Csv,
                DataReturnFormats::Tsv,
            ],
            EbiSearchDomains::Embl => vec![
                DataReturnFormats::Fasta,
                DataReturnFormats::Json,
                DataReturnFormats::Xml,
                DataReturnFormats::Csv,
                DataReturnFormats::Tsv,
            ],
            EbiSearchDomains::ArrayExpress => vec![DataReturnFormats::Json],
            EbiSearchDomains::ExpressionAtlas => vec![DataReturnFormats::Json],
            EbiSearchDomains::BioModels => vec![DataReturnFormats::Json],
            EbiSearchDomains::BioSamples => vec![DataReturnFormats::Json],
            EbiSearchDomains::ChEMBL => vec![DataReturnFormats::Json],
            EbiSearchDomains::ComplexPortal => vec![DataReturnFormats::Json],
            EbiSearchDomains::EGA => vec![DataReturnFormats::Json],
            EbiSearchDomains::Ensembl => vec![DataReturnFormats::Json],
            EbiSearchDomains::EnsemblGenomes => vec![DataReturnFormats::Json],
            EbiSearchDomains::EuropePMC => vec![DataReturnFormats::Json],
            EbiSearchDomains::GeneExpressionAtlas => vec![DataReturnFormats::Json],
            EbiSearchDomains::MetaboLights => vec![DataReturnFormats::Json],
            EbiSearchDomains::PDBe => vec![DataReturnFormats::Json],
            EbiSearchDomains::PRIDE => vec![DataReturnFormats::Json],
            EbiSearchDomains::Reactome => vec![DataReturnFormats::Json],
            EbiSearchDomains::SequenceReadArchive => vec![DataReturnFormats::Json],
            EbiSearchDomains::UniParc => vec![DataReturnFormats::Json],
            EbiSearchDomains::UniProt => vec![DataReturnFormats::Json],
            EbiSearchDomains::UniRef => vec![DataReturnFormats::Json],
        }
    }
}
