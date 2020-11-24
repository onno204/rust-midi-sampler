extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{MidiInput, MidiInputPort, MidiInputPorts, MidiOutput, MidiOutputConnection, MidiOutputPort, MidiOutputPorts};

const PREFFERED_MIDI_INPUT: &str = "loopMIDI_IN2";
const PREFFERED_MIDI_OUTPUT: &str = "loopMIDI_OUT_2";

#[allow(dead_code)]
enum PadColor {
    Off = 00,
    Green = 01,
    GreenBlink = 02,
    Red = 03,
    RedBlink = 04,
    Yellow = 05,
    YellowBlink = 06,
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum PadState {
    PadPressed = 144,
    PadReleased = 128,
    SliderUsed = 176,
    UNKOWN = -1,
}

impl From<u8> for PadState {
    fn from(n: u8) -> PadState {
        match n {
            144 => PadState::PadPressed,
            128 => PadState::PadReleased,
            176 => PadState::SliderUsed,
            _ => PadState::UNKOWN
        }
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn get_sampler_pads() -> Vec<u8> {
    let mut pads: Vec<u8> = Vec::new();
    for x in 32..48 {
        pads.push(x);
    }
    pads
}

fn get_group_pads() -> Vec<u8> {
    let mut pads: Vec<u8> = Vec::new();
    for x in 84..90 {
        pads.push(x);
    }
    pads
}

fn send_midi_data(conn_out: &mut MidiOutputConnection, pad: &u8, color_code: PadColor) -> bool {
    match conn_out.send(&[0x90, *pad, color_code as u8]) {
        Ok(..) => true,
        Err(..) => false
    }
}

fn incomming_midi_action(conn_out: &mut MidiOutputConnection, message: &[u8]) -> () {
    let action_id: u8 = message[0];
    let pad_id: u8 = message[1];
    let action_value: u8 = message[2];
    println!("action_id: {:?}, pad_id: {:?}, action_value: {:?}", action_id, pad_id, action_value);
    let action: PadState = action_id.into();
    if get_sampler_pads().contains(&pad_id) {
        if action == PadState::PadPressed {
            send_midi_data(conn_out, &pad_id, PadColor::Red);
        } else if action == PadState::PadReleased {
            send_midi_data(conn_out, &pad_id, PadColor::YellowBlink);
        }
    }
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

    println!("Opening connection with input \"{}\" and output \"{}\"", midi_in.port_name(in_port)?, midi_out.port_name(out_port)?);
    let mut conn_out: MidiOutputConnection = midi_out.connect(out_port, "midir-test")?;

    for x in get_sampler_pads() {
        send_midi_data(&mut conn_out, &x, PadColor::YellowBlink);
    }
    for x in get_group_pads() {
        send_midi_data(&mut conn_out, &x, PadColor::GreenBlink);
    }

    // This needs to be called as last because of moving variables
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-test-read-input", move |_stamp, message, _| {
        incomming_midi_action(&mut conn_out, message);
    }, ())?;

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
                    println!("Choosing preferred input port: {}", midi_in.port_name(p).unwrap());
                    return i;
                }
            }
            println!("Available input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
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
                    println!("Choosing preferred output port: {}", midi_out.port_name(p).unwrap());
                    return i;
                }
            }
            println!("Available output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            return input.trim().parse::<usize>().unwrap();
        }
    };
}
