use tokio;

#[tokio::test]
async fn test_ncbi_blast_full() {
    let blast = ebiotic::tools::Blast::default();
    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
    let test = blast.run(query).await.unwrap();
    println!("{:?}", test);
}

#[tokio::test]
async fn test_ebi_tools_clustalo() {
    let mut clustalo = ebiotic::tools::Clustalo::default();
    clustalo.set_email("harryallsopp8@gmail.com".to_string());

    let seq1 = ebiotic::tools::Record::with_attrs(
        &"seq1".to_string(),
        None,
        "AGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA"
            .to_string()
            .as_ref(),
    );
    let seq2 = ebiotic::tools::Record::with_attrs(
        &"seq2".to_string(),
        None,
        "TACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA"
            .to_string()
            .as_ref(),
    );
    let seq3 = ebiotic::tools::Record::with_attrs(
        &"seq3".to_string(),
        None,
        "GCTAGTCCGATGCGTACGATCGTACGATGCTAGCTAGCTAGCTAGCTAGCTA"
            .to_string()
            .as_ref(),
    );
    let seq4 = ebiotic::tools::Record::with_attrs(
        &"seq4".to_string(),
        None,
        "CGTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA"
            .to_string()
            .as_ref(),
    );

    clustalo.set_sequences(vec![seq1, seq2, seq3, seq4]);

    let test = clustalo.run().await;
    println!("{test}");
}
