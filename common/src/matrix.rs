pub mod matriz {
    pub fn arrays() {
        let notas: [f32; 4] = [10f32, 8.7, 9.6, 7f32];

        for i in notas.iter().enumerate() {
            let (k, v): (usize, &f32) = i;
            println!("array[{k}], = {v}");
        }

        for i in notas {
            let x: f32 = i;
            println!("valeu = {x}");
        }
    }

    pub fn matrix() {
        let matriz: [[f32; 3]; 2] = [
            [0.0, 1.3, 0.1],
            [2.1; 3]
        ];

        for _x in matriz {
            for _y in _x {
                println!("v = {_y}");
            }
        }
    }
}