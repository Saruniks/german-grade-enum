#![feature(arbitrary_enum_discriminant)]
#![feature(core_intrinsics)]
use std::intrinsics::discriminant_value;
use std::string::ToString;
use strum_macros::Display;

#[derive(Debug, Display)]
enum GradeSuffix {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
} 

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Display)]
enum Grade {
    VeryGood(Option<GradeSuffix>) = 1,
    Good(Option<GradeSuffix>),
    Satisfactory(Option<GradeSuffix>),
    Sufficient(Option<GradeSuffix>),
    Insufficient(Option<GradeSuffix>),
    Failed,
}

impl Grade {
    fn as_detailed_figure(&self) -> String {
        let prefix = match self {
            Grade::VeryGood(Some(suffix)) |
            Grade::Good(Some(suffix)) |
            Grade::Satisfactory(Some(suffix)) |
            Grade::Sufficient(Some(suffix)) |
            Grade::Insufficient(Some(suffix)) => suffix.to_string(),
            _ => "".to_string(),
        };
        format!("{}{}", prefix, discriminant_value(self))
    }
}

fn main() {
    print_grade(Grade::Good(Some(GradeSuffix::Plus)));
    print_grade(Grade::Sufficient(Some(GradeSuffix::Minus)));
    print_grade(Grade::Insufficient(None));
    print_grade(Grade::Failed);
}

fn print_grade(grade: Grade) {
    println!("Grade as symbol         : {:?}", grade);
    println!("Grade as figure         : {}"  , discriminant_value(&grade));
    println!("Grade as detailed figure: {}"  , grade.as_detailed_figure());
    println!("Grade description       : {}"  , grade.to_string()); 
    println!(); 
}
