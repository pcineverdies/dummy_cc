use edit_distance;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resolution {
    matrix: Vec<Vec<String>>,
}

impl Resolution {
    pub fn new() -> Resolution {
        let matrix: Vec<Vec<String>> = Vec::new();
        Resolution { matrix }
    }

    pub fn add_scope(&mut self) {
        self.matrix.push(Vec::new());
    }

    pub fn remove_scope(&mut self) {
        if self.matrix.len() != 0 {
            self.matrix.pop();
        } else {
            panic!("No more scopes to remove");
        }
    }

    pub fn add_identifier(&mut self, id: &String) {
        let index = self.matrix.len() - 1;
        self.matrix[index].push(id.clone());
    }

    pub fn search_identifier(&self, id: &String) -> Result<(), String> {
        for v in &self.matrix {
            for i in v {
                if i.eq(id) {
                    return Ok(());
                }
            }
        }

        let mut closer_string = "";
        let mut closer_distance = 1000;
        for v in &self.matrix {
            for i in v {
                let d = edit_distance::edit_distance(i.as_str(), id.as_str());
                if d < closer_distance {
                    closer_string = i;
                    closer_distance = d;
                }
            }
        }
        return Err(closer_string.to_string());
    }
}
