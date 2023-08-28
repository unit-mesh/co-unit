use std::path::Path;
use crate::domain::domain_record::DomainRecord;

pub struct DomainTranspiler {
    pub domain_records: Vec<DomainRecord>,
}

impl DomainTranspiler {
    pub fn new() -> Self {
        DomainTranspiler {
            domain_records: Vec::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) {
        use walkdir::WalkDir;
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let path_str = entry.path().to_str();
                if path_str.is_none() {
                    continue;
                }

                let path_str = path_str.unwrap();
                if path_str.ends_with(".csv") {
                    let _ = self.load_csv(path_str);
                } else if path_str.ends_with(".json") {
                    self.load_json(path_str);
                }
            }
        }
    }

    fn load_csv(&mut self, path: &str) -> anyhow::Result<()> {
        csv::Reader::from_path(path)?
            .into_deserialize()
            .for_each(|result| {
                let record: DomainRecord = result.unwrap();
                self.domain_records.push(record);
            });

        Ok(())
    }

    fn load_json(&mut self, path: &str) {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let records: Vec<DomainRecord> = serde_json::from_reader(reader).unwrap();
        records.into_iter().for_each(|record| {
            self.domain_records.push(record);
        });
    }

    // replace the domain words in the source, for example 金融 -> 金融(finance)
    pub fn transpile(&self, source: &str) -> String {
        let mut result = source.to_string();
        for record in &self.domain_records {
            let native = record.native.as_str();
            let english = record.english.as_str();
            let abbreviation = record.abbreviation.as_str();
            let description = record.description.as_str();

            result = result.replace(native, &format!("{}({})", native, english));
            result = result.replace(abbreviation, &format!("{}({})", abbreviation, english));
            result = result.replace(description, &format!("{}({})", description, english));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use super::*;

    #[test]
    fn load_csv() {
        let model_dir = domain_dir();


        let mut loader = DomainTranspiler::new();
        loader.load(model_dir);
        assert_eq!(loader.domain_records.len(), 29);
    }

    fn domain_dir() -> PathBuf {
        let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent()
            .unwrap()
            .join("_fixtures")
            .join("domain");
        model_dir
    }

    #[test]
    fn transpile() {
        let model_dir = domain_dir();

        let mut loader = DomainTranspiler::new();
        loader.load(model_dir);
        assert_eq!(loader.domain_records.len(), 29);

        assert_eq!(loader.transpile("本币"), "本币(Domestic Currency)");
        assert_eq!(loader.transpile("DCY"), "DCY(Domestic Currency)");
    }
}