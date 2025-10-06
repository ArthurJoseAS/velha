use std::io;

fn read_input(buf: &mut String) -> io::Result<usize>{
    match io::stdin().read_line(buf){
            Ok(n) => {
                return Ok(n);
            }
            Err(e) => {
                println!("Erro no read_line");
                return Err(e);
            }
        }
}
fn print_table(table: &[[u8; 3]; 3])
{
    for j in 0..3{
        for i in 0..3{
            if table[i][j] == 0{
                print!("   ");
            }
            else if table[i][j] == 1{
                print!(" X ");
            } 
            else{
                print!(" O ");
            }
            if i < 2 {
                print!("|");
            }
        }
        print!("\n");
    }
}
fn print_inicial()
{
    for j in 0..3{
        for i in 0..3{
            print!(" {} ", 3*j+i+1);
            if i < 2 {
                print!("|");
            }
        }
        print!("\n");
    }
}
/*
 0 | 1 | 2
 3 | 4 | 5
 6 | 7 | 8
*/
//returns in order rows, columns
//(i, j)
fn convert_input(og_input: u8) -> (usize, usize){
    let input = og_input-1;
    return ((input%3).into(), (input/3).into());

}

fn main() -> io::Result<()>
{
    //cada valor da tabela tem um sentido
    //0 = está vazio
    //1 = preenchido por X
    //2 = preenchido por O
    let mut table_state: [[u8; 3]; 3] = [[0; 3]; 3];
    print_inicial();
    let mut turno: u8 = 0; 
    let mut buf = String::new();
    loop{
        print!("É a vez do jogador: ");
        if(turno == 0){
            print!("X\n");
        }
        else{
            print!("O\n");
        }
        let mut jogada: u8;
        let mut indices: (usize, usize);
        println!("Digite o numero em que você quer jogar");
        loop{
            read_input(&mut buf).expect("Erro no stdin");
            match buf.trim().parse(){
                Ok(n) => {
                    jogada = n;
                }
                Err(_) => {
                    println!("Insira uma entrada válida");
                    buf.clear();
                    continue;
                }
            }
            if jogada > 9 || jogada < 1{
                println!("Insira um número de 1 a 9");
                buf.clear();
                continue;
            }
            let position = convert_input(jogada);
            if table_state[position.0][position.1] != 0{
                println!("Essa posição já foi utilizada, escolha outra posição");
                buf.clear();
                continue;
            }
            indices = position;
            break;
        }
        table_state[indices.0][indices.1] = turno + 1;
        print_table(&table_state);
        
        

        turno = (turno+1)%2;
        buf.clear();
    }
    // loop{
    //     println!("Turno ")
    // }
    Ok(())
}
