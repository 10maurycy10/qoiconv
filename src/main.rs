use std::path::Path;
use std::io::Write;
use libqoi::decode_qoi;
use libqoi::encode_qoi;
use image;



fn main() {
	let argsv: Vec<_>  = std::env::args().collect();
	let mut args = argsv.iter();
	args.next().unwrap();
	let mut infile = None;
	let mut outs = vec![];
	loop {
		match args.next().map(|x| x.as_str()) {
			Some("in") => infile = Some(args.next().expect("'in' reqires a filename")),
			Some("out") => outs.push(args.next().expect("'out' requres a filename")),
			Some(a) => println!("invalid argument '{}'!", a),
			None => break
		}
	}
	let infile = infile.unwrap();
	let mut img = None;
	let mut res = None;
	if infile.ends_with(".qoi") {
		let file = &std::fs::read(infile).unwrap();
		let out = decode_qoi(file).expect("Invaid file");
		img = Some(out.1);
		res = Some((out.0.height, out.0.width));
	} else {
		let iimg = image::io::Reader::open(infile).unwrap().decode().unwrap().into_rgba8();
		res = Some((iimg.height(), iimg.width()));
		img = Some((*iimg.into_raw()).to_vec());
	}
	let res = res.unwrap();
	let img = img.unwrap();
	for out in outs {
		println!("{} -> {}", infile, out);
		if out.ends_with(".qoi") {
			let qoi = encode_qoi(&img, res.0 as usize, res.1 as usize, 4, 0).unwrap();
			std::fs::File::create(out).unwrap().write_all(&qoi).unwrap();
		} else {
			image::save_buffer(&Path::new(out), &img, res.1, res.0, image::ColorType::Rgba8).unwrap();
		}
	}
	
    //let img = image::io::Reader::open("cats.jpg").unwrap().decode().unwrap();
    //println!("Opened image");
    //let size = (5935, 3898);
    //let qoi = encode_qoi(&img.into_rgba8(), size.0, size.1, 2, 1).unwrap();
    //println!("Compressed");
    //println!("SIZE {}", qoi.len());
//     std::fs::File::create("image.qoi").unwrap().write_all(&qoi).unwrap();;
    //let img2 = decode_qoi(&qoi).unwrap();
    //println!("decoded");
//     image::save_buffer(&Path::new("image.png"), &img2.1, size.0 as u32, size.1 as u32, image::ColorType::Rgba8).unwrap();
}

