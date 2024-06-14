// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{thread, time};
use wavetable::WaveTable;
use vector::VectorOscillator;
use envelope::{BreakPoints, Envelope};
use trig::{Dust, Impulse};
use interpolation::interpolation::{Linear, Cubic};

fn main() -> anyhow::Result<()> {
  const SIZE: usize = 256;
  let (mut trig_tx, trig_rx) = std::sync::mpsc::channel::<bool>();
  let (mut table_tx, table_rx) = std::sync::mpsc::channel::<sailor_lib::TableValue>();
  let mut wavetable = [0.0f32; SIZE];
  let mut wt = WaveTable::new(&wavetable, 48000.0);
  // audio stream

  std::thread::spawn(move || {
    // List all audio devices
    let host = cpal::default_host();
    let devices = host.devices().expect("Devices not found");
    for device in devices {
        println!("{}", device.name().expect("No name?"));
    }

    // List default input and output devices
    let input_device = match host.default_input_device() {
      Some(device) => {println!("Default input: {}", device.name().unwrap()); device},
      None => panic!("no default input device available")
    };

    let output_device = match host.default_output_device() {
      Some(device) => {println!("Default output: {}", device.name().unwrap()); device},
      None => panic!("no default output device available")
    };

    // Use default config from input device
    let config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();
    println!("{:#?}", config);

    let f_sample_rate = config.sample_rate.0 as f32;

    // Calculate size of ringbuffer
    // let latency_frames = (150.0 / 1000.0) * f_sample_rate;
    // let latency_samples = latency_frames as usize * config.channels as usize;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    let mut wt = WaveTable::new(&wavetable, f_sample_rate);
    let time_at_start = std::time::Instant::now();
    
    // Create a channel to send and receive samples
    let (tx, rx) = std::sync::mpsc::channel::<f32>();

    // Callbacks
    let input_callback = move | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
        let mut output_fell_behind = false;
        for &sample in data {
          // Send input data to the output callback, or do any processing
          match tx.send(sample) {
            Err(_) => output_fell_behind = true,
            _ => ()
          }
        }

        if output_fell_behind { eprintln!("Output fell behind"); }
    };

    let output_callback = move | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
        // Process output data
        let mut ch = 0;
        let mut input_fell_behind = false;
        let mut out = 0.0;
        for sample in data {
          // if let Ok(b) = trig_rx.try_recv() {
          //   if b { println!("yes!"); }
          // }
          if let Ok(t) = table_rx.try_recv() {
            wt.update_table((t.value * 2.0) - 1.0, t.index);
          }
          if (ch == 0) {
            out = wt.play::<Cubic>(200.0, 1.0);
          }
          ch = (ch + 1) % 2;
          *sample = out;
        }

        if input_fell_behind { eprintln!("Input fell behind"); }
    };

    let err_callback = |err: cpal::StreamError| {
        eprintln!("{}", err);
    };

    let input_stream = input_device.build_input_stream(
        &config, 
        input_callback,
        err_callback,
        None
    ).expect("unable to build input stream");

    let output_stream = output_device.build_output_stream(
        &config,
        output_callback,
        err_callback,
        None
    ).expect("unable to build output stream");

    input_stream.play().expect("unable to init play input stream");
    output_stream.play().expect("unable to init play output stream");

    thread::sleep(time::Duration::from_secs(1000));

  });

  sailor_lib::run(trig_tx, table_tx)
}
