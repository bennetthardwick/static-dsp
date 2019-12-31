use jack::{AudioIn, AudioOut, Client, ClientOptions};
use static_dsp::{Freeverb, Node};

fn main() {
    let client = Client::new("rust_capture", ClientOptions::NO_START_SERVER)
        .unwrap()
        .0;

    let in_spec = AudioIn::default();
    let out_spec = AudioOut::default();

    let audio_in_l_port = client.register_port("in_l", in_spec).unwrap();
    let audio_in_r_port = client.register_port("in_r", in_spec).unwrap();
    let mut audio_out_l_port = client.register_port("out_l", out_spec).unwrap();
    let mut audio_out_r_port = client.register_port("out_r", out_spec).unwrap();

    let mut freeverb: Freeverb<f32> = Freeverb::new();

    let process = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let in_l = audio_in_l_port.as_slice(ps);
            let in_r = audio_in_r_port.as_slice(ps);

            let out_l = audio_out_l_port.as_mut_slice(ps);
            let out_r = audio_out_r_port.as_mut_slice(ps);

            for (input, (out_l, out_r)) in in_l
                .iter()
                .zip(in_r.iter())
                .zip(out_l.iter_mut().zip(out_r.iter_mut()))
            {
                let (in_l, in_r) = freeverb.process((*input.0, *input.1));
                *out_l = in_l;
                *out_r = in_r;
            }

            jack::Control::Continue
        },
    );

    let active = client.activate_async((), process).unwrap();

    // Wait for user input to quit
    println!("Press enter to stop...");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).ok();

    active.deactivate().unwrap();
}
