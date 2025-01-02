pub struct CharacterClass {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub hit_points: i64,
    pub stamina_expression: String,
}

impl TypeSignature for CharacterClass {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(&self.hit_points.to_be_bytes());
        signature.extend_from_slice(self.stamina_expression.as_bytes());

        Self::as_hashed(signature)
    }
}

impl CharacterClass {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && !self.description.is_empty()
            && self.hit_points > 0
            && !self.stamina_expression.is_empty()
    }
}
