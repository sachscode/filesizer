use std::env;

#[derive(Debug)]
struct FormattedSize { 
    bytes: String, 
    kilobytes: String, 
    megabytes: String, 
    gigabytes: String 
}
struct FileSizeSpec {
    size: f64,
    suffix: String,
}

fn parse_size(size: &str) -> FileSizeSpec{
    let len = size.len();
    let (num, suffix) = size.split_at(len - 2);
    let num = num.parse::<f64>().unwrap();
    FileSizeSpec{ size: num, suffix: suffix.to_string()  }
}

fn build_output(sz: f64)-> FormattedSize {
    FormattedSize {
        bytes: format!("{} bytes",sz ),
        kilobytes: format!("{} kilobytes", sz/1024.0),
        megabytes: format!("{} megabytes", sz/(1024.0*1024.0)),
        gigabytes: format!("{} gigabytes", sz/(1024.0*1024.0*1024.0)),
    }
}

fn build_unknown_output()-> FormattedSize {
    FormattedSize {
        bytes: "Unknown".to_string(),
        kilobytes: "Unknown".to_string(),
        megabytes: "Unknown".to_string(),
        gigabytes: "Unknown".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <size>", args[0]);
        return;
    }
    let size_spec: String = args[1].clone();
    let fsize: FileSizeSpec = parse_size(&size_spec);

    let filesize = match fsize.suffix.as_str() {
        "b" => build_output(fsize.size),
        "kb" =>build_output(fsize.size*1024.0),
        "mb" => build_output(fsize.size*1024.0*1024.0),
        "gb" => build_output(fsize.size*1024.0*1024.0*1024.0),
        _ => build_unknown_output(),
    };

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("{:?}.",filesize);

}