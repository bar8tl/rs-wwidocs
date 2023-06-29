//**********************************************************************************
// typeof.rs : Display the data type of one object (2017-05-24 bar8tl)
//**********************************************************************************
pub fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}
