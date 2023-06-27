use std::env;
use std::error::Error;

mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers;

    #[test]
    fn slice_long() {
        let result = helpers::helper_h_index::get_h_index(&[3, 0, 6, 1, 5]);
        assert_eq!(result, 3,)
    }

    #[test]
    fn slice_short() {
        let result = helpers::helper_h_index::get_h_index(&[1, 3, 1]);
        assert_eq!(result, 1,)
    }

    #[test]
    fn slice_from_internet() {
        let result = helpers::helper_h_index::get_h_index(&[7, 15, 20, 30, 33, 6, 5, 4]);
        assert_eq!(result, 6)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let vec_of_args = env::args().collect::<Vec<String>>();

    let string_exec_type = match vec_of_args.get(1) {
        Some(result) => result,
        None => {
            panic!("Error: No execution indicator passed from shell.")
        }
    };

    match string_exec_type.as_str() {
        "local" => {
            if vec_of_args.len() < 3 {
                println!("h-index = 0");
            }

            let vec_of_numbers: Vec<u32> = match vec_of_args[2..vec_of_args.len()]
                .iter()
                .map(|item| item.parse::<u32>())
                .collect()
            {
                Ok(result) => result,
                Err(err) => {
                    panic!(
                        "{}",
                        [
                            "Error: Could not parse all shell args into u32.".to_string(),
                            format!("err = {:?}", err,),
                        ]
                        .join("\n")
                    );
                }
            };
            println!(
                "h-index = {:?}",
                helpers::helper_h_index::get_h_index(vec_of_numbers.as_slice())
            );
        }
        "server" => {
            match helpers::helper_api::run_server().await {
                Ok(()) => {}
                Err(_err) => {
                    println!("Responded to client with error.")
                }
            };
        }
        _ => {
            panic!("Error: No execution type provided.")
        }
    }
    Ok(())
}
