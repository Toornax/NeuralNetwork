mod data_extractor;
mod data;

use self::data_extractor::DataExtractor;
use self::data::Data;

fn main() {
    let mut extractor = DataExtractor::new();
    let mut training_data: Vec<Data> = Vec::new();

    extractor.set_path_img("./Data/train-images.idx3-ubyte");
    extractor.set_path_val("./Data/train-labels.idx1-ubyte");
    if let Err(e) = extractor.extract(&mut training_data) {
        println!("Error extracting datas : {}", e);
        return;
    }
}
