extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput, MidiInputPort, MidiInputPorts, MidiOutput, MidiOutputPort, MidiOutputPorts};

const PREFFERED_MIDI_INPUT: &str = "loopMIDI_IN2";
const PREFFERED_MIDI_OUTPUT: &str = "loopMIDI_OUT_2";

enum PadColor {
    Off = 00,
    Green = 01,
    GreenBlink = 02,
    Red = 03,
    RedBlink = 04,
    Yellow = 05,
    YellowBlink = 06,
}

enum PadState {
    PadPressed = 144,
    PadReleased = 128,
    SliderStart = 0,
    SliderEnd = 127,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn incomming_midi_action(message: &[u8]) -> () {
    println!("Hey {:?}", message)
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_in: midir::MidiInput = MidiInput::new("midir reading input")?;
    let midi_out: midir::MidiOutput = MidiOutput::new("My Test Output")?;

    let input_port_num: usize = midi_select_in_port(&midi_in);
    let in_ports: MidiInputPorts = midi_in.ports();
    let in_port: &MidiInputPort = in_ports.get(input_port_num).ok_or("Error on sellecting input")?;

    let output_port_num: usize = midi_select_out_port(&midi_out);
    let out_ports: MidiOutputPorts = midi_out.ports();
    let out_port: &MidiOutputPort = out_ports.get(output_port_num).ok_or("Error on sellecting input")?;

    println!("Opening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-test-read-input", move |_stamp, message, _| {
        incomming_midi_action(message);
    }, ())?;

    println!("Opened connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;

    let mut send_midi_data = |pad: u8, color_code: PadColor| {
        let _ = conn_out.send(&[0x90, pad, color_code as u8]);
    };

    for x in 32..48 {
        send_midi_data(x, PadColor::Off);
    }
    for x in 84..90 {
        send_midi_data(x, PadColor::Off);
    }

    println!("Press [Enter] to exit.");
    let mut input = String::new();
    input.clear();
    stdin().read_line(&mut input)?;
    Ok(())
}

fn midi_select_in_port(midi_in: &midir::MidiInput) -> usize {
    let in_ports: MidiInputPorts = midi_in.ports();
    return match in_ports.len() {
        0 => 0,
        1 => {
            println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
            0
        }
        _ => {
            for (i, p) in in_ports.iter().enumerate() {
                if midi_in.port_name(p).unwrap() == PREFFERED_MIDI_INPUT.to_string() {
                    println!("Choosing the same output as the input port: {}", midi_in.port_name(p).unwrap());
                    return i;
                }
            }
            println!("Available input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input);
            return input.trim().parse::<usize>().unwrap();
        }
    };
}

fn midi_select_out_port(midi_out: &midir::MidiOutput) -> usize {
    let out_ports: MidiOutputPorts = midi_out.ports();
    return match out_ports.len() {
        0 => 0,
        1 => {
            println!("Choosing the only available output port: {}", midi_out.port_name(&out_ports[0]).unwrap());
            0
        }
        _ => {
            for (i, p) in out_ports.iter().enumerate() {
                if midi_out.port_name(p).unwrap() == PREFFERED_MIDI_OUTPUT.to_string() {
                    println!("Choosing the same output as the output port: {}", midi_out.port_name(p).unwrap());
                    return i;
                }
            }
            println!("Available output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input);
            return input.trim().parse::<usize>().unwrap();
        }
    };
}
