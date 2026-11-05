#[derive(Debug, Clone)]
pub struct VoiceMemo { pub id: String, pub duration_sec: u32 }
impl VoiceMemo {
    pub fn new(id: impl Into<String>, dur: u32) -> Self { Self { id: id.into(), duration_sec: dur } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_memo() {
        let m = VoiceMemo::new("m1", 30);
        assert_eq!(m.duration_sec, 30);
    }
}
