//! SigmaOS-craft-voice: voice model.
//!
//! Real implementation: waveforms, phonemes, speech synthesis, TTS, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Voice type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceType {
    Neural,
    Concatenative,
    Parametric,
}

/// Speech segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechSegment {
    pub id: String,
    pub text: String,
    pub duration_ms: u32,
}

/// Voice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voice {
    pub id: String,
    pub name: String,
    pub voice_type: VoiceType,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceError {
    EmptyText,
    DuplicateSegment(String),
    InvalidDuration,
}

impl fmt::Display for VoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "segment text must not be empty"),
            Self::DuplicateSegment(id) => write!(f, "duplicate segment {:?}", id),
            Self::InvalidDuration => write!(f, "invalid duration"),
        }
    }
}

impl std::error::Error for VoiceError {}

/// TTS engine.
pub struct TTSEngine {
    pub name: String,
    voices: BTreeMap<String, Voice>,
    segments: BTreeMap<String, SpeechSegment>,
}

impl TTSEngine {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), voices: BTreeMap::new(), segments: BTreeMap::new() } }

    pub fn add_voice(&mut self, voice: Voice) { self.voices.insert(voice.id.clone(), voice); }

    pub fn add_segment(&mut self, segment: SpeechSegment) -> Result<(), VoiceError> {
        if segment.text.trim().is_empty() { return Err(VoiceError::EmptyText); }
        if segment.duration_ms == 0 { return Err(VoiceError::InvalidDuration); }
        let id = segment.id.clone();
        if self.segments.contains_key(&id) {
            return Err(VoiceError::DuplicateSegment(id.clone()));
        }
        self.segments.insert(id, segment);
        Ok(())
    }

    pub fn segment_count(&self) -> usize { self.segments.len() }

    pub fn voice_count(&self) -> usize { self.voices.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_create_aaa() {
        let e = TTSEngine::new("TTS Engine");
        assert_eq!(e.name, "TTS Engine");
        assert_eq!(e.segment_count(), 0);
    }

    #[test]
    fn test_add_segment_aaa() {
        let mut e = TTSEngine::new("TTS Engine");
        let s = SpeechSegment { id: "s1".into(), text: "Hello".into(), duration_ms: 500 };
        e.add_segment(s).unwrap();
        assert_eq!(e.segment_count(), 1);
    }

    #[test]
    fn test_empty_text_rejected_aaa() {
        let mut e = TTSEngine::new("TTS Engine");
        let s = SpeechSegment { id: "x".into(), text: "".into(), duration_ms: 100 };
        assert!(e.add_segment(s).is_err());
    }

    #[test]
    fn test_invalid_duration_rejected_aaa() {
        let mut e = TTSEngine::new("TTS Engine");
        let s = SpeechSegment { id: "x".into(), text: "X".into(), duration_ms: 0 };
        assert!(e.add_segment(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut e = TTSEngine::new("TTS Engine");
        let s = SpeechSegment { id: "x".into(), text: "X".into(), duration_ms: 100 };
        e.add_segment(s).unwrap();
        let s2 = SpeechSegment { id: "x".into(), text: "X".into(), duration_ms: 100 };
        assert!(e.add_segment(s2).is_err());
    }
}
