pub fn vect() {
    vectors()
}

fn vectors() {
    let mut notas: Vec<f32> = Vec::new();
    notas.push(7f32);
    notas.push(8.5);
    notas.push(6.2);

    println!("{:?}", notas);

    let mut medias: Vec<u8> = vec![2, 4, 6];
    println!("Capacity: {}", medias.capacity());
    medias.push(10);
    println!("Capacity: {}", notas.capacity());

    for m in &medias {
        // let (k, v) : (usize, &u8) = m;
        let v: &u8 = m;
        println!("value: {}", v)
    }
    println!("Medias: {:?}", medias);

    println!("Nota i[6]: {}", match notas.get(6) {
        Some(n) => *n,
        None => 0.0
    });

    // while let Some(n) = notas.pop() {
    //     println!("Nota: {}", n);
    // }

    let mut medianas: Vec<u8> = Vec::with_capacity(4);
    medianas.push(34);
    medianas.push(13);

    for m in &medianas {
        println!("mediana: {}", m);
    }
    println!("Capacity medianas: {}", medianas.capacity());

    let mut buf = vec![0; 10];
    buf.fill(1);
    for b in buf {
        println!("buf: {}", b);
    }
}
