use sigmaos_craft_voice::{TTSEngine, SpeechSegment};

#[test]
fn tts_engine_integration_aaa() {
    let mut e = TTSEngine::new("TTS Engine");
    e.add_segment(SpeechSegment { id: "s1".into(), text: "Hello".into(), duration_ms: 500 }).unwrap();
    assert_eq!(e.segment_count(), 1);
}
