use stam::*;
use std::collections::BTreeSet;
use std::env;
use std::ops::Deref;
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

fn version_naive(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    // This version takes very long, minutes!

    let mut results = Vec::new();
    let begin = SystemTime::now();

    for phrase in store
        .find_data("features", "otype", stam::DataOperator::Equals("phrase"))
        .annotations()
    {
        let mut words = phrase.annotations_in_targets(false).filter_key_value(
            &store.key("features", "otype").or_fail()?,
            stam::DataOperator::Equals("word"),
        );

        if let (Some(firstword), Some(lastword)) = (words.next(), words.last()) {
            if let Some(data) = firstword
                .annotations()
                .data()
                .filter_key(&store.key("features", "nu").or_fail()?)
                .next()
            {
                if lastword
                    .annotations()
                    .data()
                    .filter_annotationdata(&data)
                    .test()
                {
                    results.push((phrase.clone(), firstword.clone(), lastword.clone()));
                }
            }
        }
    }

    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_cached(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let mut results = Vec::new();
    let begin = SystemTime::now();

    let set_features = store.dataset("features").unwrap();
    let key_nu = set_features.key("nu").unwrap();

    let otype_word = store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("word"))
        .next()
        .unwrap();

    for phrase in store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("phrase"))
        .annotations()
    {
        let mut words = phrase
            .annotations_in_targets(false)
            .filter_annotationdata(&otype_word);

        if let (Some(firstword), Some(lastword)) = (words.next(), words.last()) {
            if let Some(data_nu) = firstword.annotations().data().filter_key(&key_nu).next() {
                if lastword
                    .annotations()
                    .data()
                    .filter_annotationdata(&data_nu)
                    .test()
                {
                    results.push((phrase.clone(), firstword.clone(), lastword.clone()));
                }
            }
        }
    }

    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_parallel(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let begin = SystemTime::now();

    let set_features = store.dataset("features").unwrap();
    let key_nu = set_features.key("nu").unwrap();

    let otype_word = store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("word"))
        .next()
        .unwrap();

    let results: Vec<_> = store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("phrase"))
        .annotations()
        .parallel()
        .filter_map(|phrase| {
            let mut words = phrase
                .annotations_in_targets(false)
                .filter_annotationdata(&otype_word);

            if let (Some(firstword), Some(lastword)) = (words.next(), words.last()) {
                if let Some(data_nu) = firstword.annotations().data().filter_key(&key_nu).next() {
                    if lastword
                        .annotations()
                        .data()
                        .filter_annotationdata(&data_nu)
                        .test()
                    {
                        return Some((phrase.clone(), firstword.clone(), lastword.clone()));
                    }
                }
            }
            None
        })
        .collect();

    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_morecache(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    //this actually performs way worse
    let mut results = Vec::new();
    let begin = SystemTime::now();

    let set_features = store.dataset("features").unwrap();

    let key_nu = set_features.key("nu").unwrap();
    let annotations_nu = key_nu
        .data()
        .annotations()
        .annotations_in_targets(false)
        .to_handles(store);

    let otype_word = store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("word"))
        .next()
        .unwrap();

    for phrase in store
        .find_data(&set_features, "otype", stam::DataOperator::Equals("phrase"))
        .annotations()
    {
        let mut words = phrase
            .annotations_in_targets(false)
            .filter_annotationdata(&otype_word);

        if let (Some(firstword), Some(lastword)) = (words.next(), words.last()) {
            if annotations_nu
                .items()
                .filter_annotations(
                    [firstword.clone(), lastword.clone()]
                        .into_iter()
                        .to_handles(store),
                )
                .test()
            {
                if let Some(data_nu) = firstword.annotations().data().filter_key(&key_nu).next() {
                    if lastword
                        .annotations()
                        .data()
                        .filter_annotationdata(&data_nu)
                        .test()
                    {
                        results.push((phrase.clone(), firstword.clone(), lastword.clone()));
                    }
                }
            }
        }
    }

    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn main() -> Result<(), stam::StamError> {
    let store = stam::AnnotationStore::from_file(
        "bhsa.store.stam.cbor", //BAD hardcoded!
        stam::Config::default(),
    )?;
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 || args[1] == "naive" {
        eprintln!("Naive version:");
        version_naive(&store)?;
    }

    if args.len() <= 1 || args[1] == "cached" {
        eprintln!("Cached version:");
        version_cached(&store)?;
    }
    if args.len() <= 1 || args[1] == "parallel" {
        eprintln!("Parallel version:");
        version_parallel(&store)?;
    }

    if args.len() <= 1 || args[1] == "morecache" {
        eprintln!("Morecache version:");
        version_morecache(&store)?;
    }
    Ok(())
}
