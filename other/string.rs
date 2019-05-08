fn main(){
    let phrase1 = String::from("I like dogs!");

    let phrase2 = phrase1.replace("dogs", "Cats");

    println!("{}\n{}",phrase1, phrase2);

}
