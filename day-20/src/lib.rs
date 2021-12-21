use ndarray::{concatenate, Array2, Axis};
use nom::{
    character::{complete::newline, complete::one_of},
    combinator::opt,
    multi::{fold_many0, many1, separated_list1},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, (Vec<char>, Array2<char>)> {
    let (input, enhancements) = enhancement_list(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, image) = image(input)?;
    Ok((input, (enhancements, image)))
}

fn enhancement_list(input: &str) -> IResult<&str, Vec<char>> {
    let (input, list) = fold_many0(
        one_of("#."),
        || Vec::new(),
        |mut acc, symbol| {
            match symbol {
                '#' => acc.push('1'),
                '.' => acc.push('0'),
                _ => panic!("invalid symbol"),
            }
            acc
        },
    )(input)?;
    Ok((input, list))
}

fn image(input: &str) -> IResult<&str, Array2<char>> {
    let (input, image_vec) = separated_list1(newline, many1(one_of(".#")))(input)?;
    let nrows = image_vec.len();
    let ncols = image_vec.first().unwrap().len();
    let image_vec = image_vec
        .iter()
        .flat_map(|line| {
            line.iter().map(|symbol| match symbol {
                '#' => '1',
                '.' => '0',
                _ => panic!("invalid symbol in image"),
            })
        })
        .collect::<Vec<char>>();
    let image_map = Array2::from_shape_vec((nrows, ncols), image_vec).unwrap();
    Ok((input, image_map))
}

pub fn enhance_image(mut image: Array2<char>, enhancements: &Vec<char>, iterations: u8) -> u32 {
    let mut padding_char = '0';
    for _ in 0..iterations {
        image = pad_image(&image, padding_char);
        image = process_image(image, enhancements, padding_char);
        let padding_index = usize::from_str_radix(&[padding_char; 9].into_iter().collect::<String>(), 2).expect("padding was valid number");
        padding_char = *enhancements.get(padding_index).unwrap();
    }
    image.fold(0u32, |mut count, c | {
        if c == &'1' {
            count += 1
        }
        count
    })
    
}

fn process_image(image: Array2<char>, enhancements: &Vec<char>, padding_char: char) -> Array2<char> {
    let mut enhanced_image = Array2::from_elem((image.nrows(), image.ncols()), '0');
    for ((this_row, this_col), _) in image.indexed_iter() {
        let up = this_row.wrapping_sub(1);
        let down = this_row + 1;
        let left = this_col.wrapping_sub(1);
        let right = this_col + 1;
        let mut binary_string = String::with_capacity(9);
        for (row, col) in [
            (up, left),
            (up, this_col),
            (up, right),
            (this_row, left),
            (this_row, this_col),
            (this_row, right),
            (down, left),
            (down, this_col),
            (down, right),
        ] {
            let val = image.get((row, col)).unwrap_or(&padding_char);
            binary_string.push(*val);
        }
        let binary_number = usize::from_str_radix(&binary_string, 2).unwrap();
        let new_val = enhancements.get(binary_number).unwrap();
        enhanced_image[(this_row, this_col)] = *new_val;
    }

    enhanced_image
}

fn pad_image(image: &Array2<char>, pad_char: char) -> Array2<char> {
    let col_padding = Array2::from_elem((image.nrows(), 3), pad_char);
    let mut padded_image = concatenate(
        Axis(1),
        &[col_padding.view(), image.view(), col_padding.view()],
    )
    .expect("Col padding failed");
    let row_padding = Array2::from_elem((3, padded_image.ncols()), pad_char);
    padded_image = concatenate(
        Axis(0),
        &[row_padding.view(), padded_image.view(), row_padding.view()],
    )
    .expect("row padding failed");
    padded_image
}
