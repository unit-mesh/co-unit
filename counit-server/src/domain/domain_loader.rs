use crate::domain::domain_record::DomainRecord;

pub struct DomainLoader {
    pub domain_records: Vec<DomainRecord>,
}

impl DomainLoader {
    pub fn new() -> Self {
        DomainLoader {
            domain_records: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
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
                    self.load_csv(path_str);
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
}