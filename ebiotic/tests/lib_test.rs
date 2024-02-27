use ebiotic::data::*;
use ebiotic::tools::*;
use tokio;

#[tokio::test]
async fn blast_run_with_valid_query_returns_expected_result() {
    let blast = Blast::default();
    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
    let result = blast.run(query.to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn blast_run_with_empty_query_returns_error() {
    let blast = Blast::default();
    let query = "";
    let result = blast.run(query.to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn clustalo_run_with_valid_sequences_returns_expected_result() {
    let mut clustalo = Clustalo::default();
    clustalo.set_email("harryallsopp8@gmail.com".to_string());

    let seq1 = Record::with_attrs(
        &"seq1".to_string(),
        None,
        "AGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA"
            .to_string()
            .as_ref(),
    );
    let seq2 = Record::with_attrs(
        &"seq2".to_string(),
        None,
        "TACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA"
            .to_string()
            .as_ref(),
    );
    let seq3 = Record::with_attrs(
        &"seq3".to_string(),
        None,
        "GCTAGTCCGATGCGTACGATCGTACGATGCTAGCTAGCTAGCTAGCTAGCTA"
            .to_string()
            .as_ref(),
    );
    let seq4 = Record::with_attrs(
        &"seq4".to_string(),
        None,
        "CGTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA"
            .to_string()
            .as_ref(),
    );

    let result = clustalo.run(vec![seq1, seq2, seq3, seq4]).await;

    println!("{:?}", result);

    assert!(result.is_ok());
}

#[tokio::test]
async fn clustalo_run_with_empty_sequences_returns_error() {
    let mut clustalo = Clustalo::default();
    clustalo.set_email("harryallsopp8@gmail.com".to_string());

    let result = clustalo.run(vec![]).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_dbfetch() {
    let mut dbfetch = Dbfetch::default();
    let ids = DbfetchIds::new(vec![
        "M10051".to_string(),
        "K00650".to_string(),
        "D87894".to_string(),
        "AJ242600".to_string(),
    ]);

    let result = dbfetch.run(ids).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dbfetch_default_fasta() {
    let mut dbfetch = Dbfetch::default();
    let ids = DbfetchIds::new(vec![
        "M10051".to_string(),
        "K00650".to_string(),
        "D87894".to_string(),
        "AJ242600".to_string(),
    ]);

    let result = dbfetch.run(ids).await;

    assert!(result.is_ok());

    println!("{:?}", result.unwrap().into_records());
}
