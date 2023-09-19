use stam;
use std::collections::BTreeSet;
use std::env;
use std::ops::Deref;
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

fn version_a(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let mut results = Vec::new();
    let begin = SystemTime::now();
    for (phrase, _data) in
        store.annotations_by_data("features", "otype", &stam::DataOperator::Equals("phrase"))
    {
        //eprintln!("phrase: {:?}", phrase);
        let words: Vec<_> = phrase
            .find_data_in_targets(
                "features",
                "otype",
                &stam::DataOperator::Equals("word"),
                false,
            )
            .into_iter()
            .flatten()
            .map(|(_data, annotation)| annotation)
            .collect();
        if words.len() < 2 {
            continue;
        }
        let firstword = words.first().unwrap();
        let lastword = words.last().unwrap();
        if let Some((data, _annotation)) = firstword
            .find_data_about("features", "nu", &stam::DataOperator::Any)
            .into_iter()
            .flatten()
            .next()
        {
            if lastword.has_data_about(data) {
                results.push((phrase.clone(), firstword.clone(), lastword.clone()));
            }
        }
    }
    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_a2(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let dataset_features = store.dataset("features").unwrap();
    let key_otype = dataset_features.key("otype").unwrap();
    let key_nu = dataset_features.key("nu").unwrap();

    let mut results = Vec::new();
    let begin = SystemTime::now();
    for (phrase, _data) in store.annotations_by_data(
        dataset_features.handle(),
        key_otype.handle(),
        &stam::DataOperator::Equals("phrase"),
    ) {
        //eprintln!("phrase: {:?}", phrase);
        let words: Vec<_> = phrase
            .find_data_in_targets(
                dataset_features.clone(),
                key_otype.clone(),
                &stam::DataOperator::Equals("word"),
                false,
            )
            .into_iter()
            .flatten()
            .map(|(_data, annotation)| annotation)
            .collect();
        if words.len() < 2 {
            continue;
        }
        let firstword = words.first().unwrap();
        let lastword = words.last().unwrap();
        if let Some((data, _annotation)) = firstword
            .find_data_about(
                dataset_features.handle(),
                key_nu.handle(),
                &stam::DataOperator::Any,
            )
            .into_iter()
            .flatten()
            .next()
        {
            if lastword.has_data_about(data) {
                results.push((phrase.clone(), firstword.clone(), lastword.clone()));
            }
        }
    }
    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_a3(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let dataset_features = store.dataset("features").unwrap();
    let key_otype = dataset_features.key("otype").unwrap();
    let key_nu = dataset_features.key("nu").unwrap();
    let data_otype_word = dataset_features
        .find_data(key_otype.handle(), &stam::DataOperator::Equals("word"))
        .into_iter()
        .flatten()
        .next()
        .unwrap();

    let mut results = Vec::new();
    let begin = SystemTime::now();
    for (phrase, _data) in store.annotations_by_data(
        dataset_features.handle(),
        key_otype.handle(),
        &stam::DataOperator::Equals("phrase"),
    ) {
        let mut firstword = None;
        let mut lastword = None;
        for word in phrase
            .annotations_in_targets(false, false)
            .filter(|annotation| annotation.deref().has_data(&data_otype_word))
        {
            if firstword.is_none() {
                firstword = Some(word);
            } else {
                lastword = Some(word);
            }
        }
        if lastword.is_none() {
            continue;
        }
        let firstword = firstword.unwrap();
        let lastword = lastword.unwrap();
        if let Some((data, _annotation)) = firstword
            .find_data_about(
                dataset_features.handle(),
                key_nu.handle(),
                &stam::DataOperator::Any,
            )
            .into_iter()
            .flatten()
            .next()
        {
            if lastword.has_data_about(data) {
                results.push((phrase.clone(), firstword.clone(), lastword.clone()));
            }
        }
    }
    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{} {:?}", results.len(), duration);
    Ok(())
}

fn version_a4(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    //parallel version
    let dataset_features = store.dataset("features").unwrap();
    let key_otype = dataset_features.key("otype").unwrap();
    let key_nu = dataset_features.key("nu").unwrap();
    let data_otype_word = dataset_features
        .find_data(key_otype.handle(), &stam::DataOperator::Equals("word"))
        .into_iter()
        .flatten()
        .next()
        .unwrap();

    let begin = SystemTime::now();
    let phrases: Vec<_> = store
        .annotations_by_data(
            dataset_features.handle(),
            key_otype.handle(),
            &stam::DataOperator::Equals("phrase"),
        )
        .map(|x| x.0)
        .collect();

    let results: Vec<_> = phrases
        .into_par_iter()
        .filter_map(|phrase| {
            let mut firstword = None;
            let mut lastword = None;
            for word in phrase
                .annotations_in_targets(false, false)
                .filter(|annotation| annotation.deref().has_data(&data_otype_word))
            {
                if firstword.is_none() {
                    firstword = Some(word);
                } else {
                    lastword = Some(word);
                }
            }
            if lastword.is_some() {
                let firstword = firstword.unwrap();
                let lastword = lastword.unwrap();
                if let Some((data, _annotation)) = firstword
                    .find_data_about(
                        dataset_features.handle(),
                        key_nu.handle(),
                        &stam::DataOperator::Any,
                    )
                    .into_iter()
                    .flatten()
                    .next()
                {
                    if lastword.has_data_about(data) {
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

fn version_b(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    let mut results = Vec::new();
    let begin = SystemTime::now();
    let dataset_features = store.dataset("features").unwrap();
    let key_otype = dataset_features.key("otype").unwrap();
    let key_nu = dataset_features.key("nu").unwrap();
    let mut phrases = 0;
    let mut time_find_data_in_targets = Duration::new(0, 0);
    let mut time_find_data_about = Duration::new(0, 0);
    let mut time_test_data_about = Duration::new(0, 0);
    let mut time_annotations_by_data = Duration::new(0, 0);
    let mut time_results = Duration::new(0, 0);
    let mut time_intermezzo = Duration::new(0, 0);
    let mut time_inner = Duration::new(0, 0);
    let mut time_inner2 = Duration::new(0, 0);
    let mut begin2: SystemTime;
    let mut begin3: SystemTime = SystemTime::now();
    let mut begin4: SystemTime;
    let mut begin5: SystemTime;
    for (phrase, _data) in store.annotations_by_data(
        dataset_features.handle(),
        key_otype.handle(),
        &stam::DataOperator::Equals("phrase"),
    ) {
        time_annotations_by_data += SystemTime::now().duration_since(begin3).unwrap();
        phrases += 1;
        begin2 = SystemTime::now();
        begin4 = begin2;
        let words: Vec<_> = phrase
            .find_data_in_targets(
                dataset_features.handle(),
                key_otype.handle(),
                &stam::DataOperator::Equals("word"),
                false,
            )
            .into_iter()
            .flatten()
            .map(|(_data, annotation)| annotation)
            .collect();
        time_find_data_in_targets += SystemTime::now().duration_since(begin2).unwrap();
        begin2 = SystemTime::now();
        if words.len() < 2 {
            begin3 = begin2;
            time_inner += SystemTime::now().duration_since(begin4).unwrap();
            continue;
        }
        let firstword = words.first().unwrap();
        let lastword = words.last().unwrap();
        time_intermezzo += SystemTime::now().duration_since(begin2).unwrap();
        begin2 = SystemTime::now();
        begin5 = begin2;
        if let Some((data, _annotation)) = firstword
            .find_data_about(
                dataset_features.handle(),
                key_nu.handle(),
                &stam::DataOperator::Any,
            )
            .into_iter()
            .flatten()
            .next()
        {
            time_find_data_about += SystemTime::now().duration_since(begin2).unwrap();
            begin2 = SystemTime::now();
            if lastword.test_data_about(
                dataset_features.handle(),
                key_nu.handle(),
                &data.value().into(),
            ) {
                time_test_data_about += SystemTime::now().duration_since(begin2).unwrap();
                begin2 = SystemTime::now();
                results.push((phrase.clone(), firstword.clone(), lastword.clone()));
                time_results += SystemTime::now().duration_since(begin2).unwrap();
            }
            begin2 = SystemTime::now();
        }
        time_inner += SystemTime::now().duration_since(begin4).unwrap();
        time_inner2 += SystemTime::now().duration_since(begin5).unwrap();
        begin3 = SystemTime::now();
    }
    let duration = SystemTime::now().duration_since(begin).unwrap();
    eprintln!("{:?}", duration);
    eprintln!("results: {}", results.len());
    eprintln!("phrases considered: {}", phrases);
    eprintln!(
        "find_find_data_in_targets cumulative: {:?}",
        time_find_data_in_targets
    );
    eprintln!("find_data_about cumulative: {:?}", time_find_data_about);
    eprintln!("test_data_about cumulative: {:?}", time_test_data_about);
    eprintln!(
        "annotations_by_data cumulative: {:?}",
        time_annotations_by_data
    );
    eprintln!("intermezzo cumulative: {:?}", time_intermezzo);
    eprintln!("result adding cumulative: {:?}", time_results);
    eprintln!("inner cumulative: {:?}", time_inner);
    eprintln!("inner2 cumulative: {:?}", time_inner2);
    Ok(())
}

fn version_c(store: &stam::AnnotationStore) -> Result<(), stam::StamError> {
    //using sets

    let mut results = Vec::new();
    let begin = SystemTime::now();
    for (phrase, _data) in
        store.annotations_by_data("features", "otype", &stam::DataOperator::Equals("phrase"))
    {
        let words: Vec<_> = phrase
            .find_data_in_targets(
                "features",
                "otype",
                &stam::DataOperator::Equals("word"),
                false,
            )
            .into_iter()
            .flatten()
            .map(|(_data, annotation)| annotation)
            .collect();
        if words.len() < 2 {
            continue;
        }
        let firstword = words.first().unwrap();
        let lastword = words.last().unwrap();
        let lastword_data: BTreeSet<_> = lastword
            .find_data_about("features", "nu", &stam::DataOperator::Any)
            .into_iter()
            .flatten()
            .map(|(data, _annotation)| data)
            .collect();
        for (data, _) in firstword
            .find_data_about("features", "nu", &stam::DataOperator::Any)
            .into_iter()
            .flatten()
        {
            if lastword_data.contains(&data) {
                results.push((phrase.clone(), firstword.clone(), lastword.clone()));
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

    if args.is_empty() || args[0] == "a" {
        eprintln!("A (naive, no variables):");
        version_a(&store)?;
    }
    if args.is_empty() || args[0] == "a2" {
        eprintln!("A2 (collect all words):");
        version_a2(&store)?;
    }
    if args.is_empty() || args[0] == "a3" {
        eprintln!("A3 (normal):");
        version_a3(&store)?;
    }
    if args.is_empty() || args[0] == "a4" {
        eprintln!("A4 (parallel):");
        version_a4(&store)?;
    }
    if args.is_empty() || args[0] == "b" {
        eprintln!("B (measured details):");
        version_b(&store)?;
    }
    if args.is_empty() || args[0] == "c" {
        eprintln!("C (sets):");
        version_c(&store)?;
    }
    Ok(())
}
