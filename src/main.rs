use std::process;
mod db_data_modification;

///From  https://colab.research.google.com/github/fastai/fastbook/blob/master/08_collab.ipynb#scrollTo=DaLIxvUZjy2d

fn main() {
    if let Err(err) = db_data_modification::init_train_data() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
