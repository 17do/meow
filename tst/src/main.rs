use meow;

fn main() {
    let s = meow::MeowMusics::new("/home/inado/Music/Kan Saete Kuyashiiwa.mp3")
        .unwrap()
        .play()
        .unwrap();

    std::thread::sleep(s);
}
