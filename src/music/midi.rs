use midly::{num::*, Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};
use crate::music::orchestrator::BarScore;

pub struct MidiExporter;

impl MidiExporter {
    pub fn export_to_midi(track_score: &[BarScore], bpm: f32) -> Vec<u8> {
        let ticks_per_beat = 480; // Standard PPQ
        
        let header = Header {
            format: Format::Parallel,
            timing: Timing::Metrical(u15::from_int_lossy(ticks_per_beat)),
        };

        // Track 0: Tempo and Meta
        let mut meta_track = Track::new();
        let tempo_micros = (60_000_000.0 / bpm) as u32;
        meta_track.push(TrackEvent {
            delta: u28::from_int_lossy(0),
            kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::from_int_lossy(tempo_micros))),
        });
        meta_track.push(TrackEvent {
            delta: u28::from_int_lossy(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        // Track 1: Melody
        let mut melody_track = Track::new();
        // Track 2: Kick Drum
        let mut drum_track = Track::new();

        let ticks_per_step = ticks_per_beat / 4; // 16th notes = 1/4 beat

        for bar in track_score {
            let mut last_melody_tick = 0;
            let mut last_drum_tick = 0;

            for step in 0..16 {
                let absolute_tick = step as u32 * ticks_per_step as u32;

                // Kick Drum (Channel 9 is drums, but we'll use Ch 0 and let DAW route)
                let kick = &bar.kick[step];
                if kick.velocity > 0.0 {
                    let vel = (kick.velocity * 127.0) as u8;
                    // Note on
                    let delta_on = absolute_tick - last_drum_tick;
                    drum_track.push(TrackEvent {
                        delta: u28::from_int_lossy(delta_on),
                        kind: TrackEventKind::Midi { channel: u4::new(0), message: MidiMessage::NoteOn { key: u7::new(36), vel: u7::new(vel) } },
                    });
                    
                    // Note off
                    let delta_off = ticks_per_step as u32 / 2;
                    drum_track.push(TrackEvent {
                        delta: u28::from_int_lossy(delta_off),
                        kind: TrackEventKind::Midi { channel: u4::new(0), message: MidiMessage::NoteOff { key: u7::new(36), vel: u7::new(0) } },
                    });
                    last_drum_tick = absolute_tick + delta_off;
                }

                // Melody
                let lead = &bar.lead[step];
                if lead.velocity > 0.0 {
                    let vel = (lead.velocity * 127.0) as u8;
                    let mut pitch = lead.pitch.round() as u8;
                    pitch = pitch.clamp(0, 127);
                    
                    let delta_on = absolute_tick - last_melody_tick;
                    melody_track.push(TrackEvent {
                        delta: u28::from_int_lossy(delta_on),
                        kind: TrackEventKind::Midi { channel: u4::new(1), message: MidiMessage::NoteOn { key: u7::new(pitch), vel: u7::new(vel) } },
                    });
                    
                    // Note off
                    let len_ticks = (lead.length as f32 * ticks_per_step as f32) as u32;
                    let delta_off = len_ticks.max(1);
                    melody_track.push(TrackEvent {
                        delta: u28::from_int_lossy(delta_off),
                        kind: TrackEventKind::Midi { channel: u4::new(1), message: MidiMessage::NoteOff { key: u7::new(pitch), vel: u7::new(0) } },
                    });
                    last_melody_tick = absolute_tick + delta_off;
                }
            }
        }
        
        melody_track.push(TrackEvent {
            delta: u28::from_int_lossy(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        drum_track.push(TrackEvent {
            delta: u28::from_int_lossy(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        let smf = Smf {
            header,
            tracks: vec![meta_track, melody_track, drum_track],
        };

        let mut buffer = Vec::new();
        smf.write(&mut buffer).unwrap();
        buffer
    }
}
