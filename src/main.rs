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
fn print_table(table: &[[u32; 3]; 3])
{
    for i in 0..3{
        for j in 0..3{
            if table[i][j] == 0{
                print!("   ");
            }
            else if table[i][j] == 1{
                print!(" X ");
            } 
            else{
                print!(" O ");
            }
            if j < 2 {
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
fn convert_input(og_input: u32) -> (usize, usize){
    let input = og_input-1;
    return ((input/3) as usize, (input%3) as usize);

}

//se jogador == 1 é X
//se jogador == 2 é O
fn checar_vitoria(table: &[[u32; 3]; 3], last_play: (usize, usize), jogador: u32) -> bool{
    //checar vertical
    fn checar_vertical(table: &[[u32; 3]; 3], last_play: (usize, usize), jogador: u32) -> bool{
        for i in 1..3{
            // print!("checando vertical: ({}, {}) {}\n", (last_play.0+i)%3 , last_play.1 , table[(last_play.0+i)%3][last_play.1]);
            if table[(last_play.0+i)%3][last_play.1] != jogador{
                return false;
            }
        }
        return true;
    }
    fn checar_horizontal(table: &[[u32; 3]; 3], last_play: (usize, usize), jogador: u32) -> bool{
        for i in 1..3{
            // print!("checando horizontal: ({}, {}) {}\n", last_play.0 , (last_play.1+i)%3 , table[last_play.0][(last_play.1+i)%3]);
            if table[last_play.0][(last_play.1+i)%3] != jogador{
                return false;
            }
        }
        return true;
    }
    
    fn checar_diagonal_direita(table: &[[u32; 3]; 3], last_play: (usize, usize), jogador: u32) -> bool{
        if last_play.0 != last_play.1 {
            return false;
        }
        for i in 1..3{
            // print!("checando diagonal direita: ({}, {}) {}\n", last_play.0 , (last_play.1+i)%3 , table[last_play.0][(last_play.1+i)%3]);
            if table[(last_play.0+i)%3][(last_play.1+i)%3] != jogador{
                return false;
            }
        }
        return true;
    }
    fn checar_diagonal_esquerda(table: &[[u32; 3]; 3], last_play: (usize, usize), jogador: u32) -> bool{
        if last_play.0 + last_play.1 != 3-1 {
            return false;
        }
        for i in 1..3{
            // print!("checando diagonal direita: ({}, {}) {}\n", last_play.0 , (last_play.1+i)%3 , table[last_play.0][(last_play.1+i)%3]);
            if table[(last_play.0 + (3-i))%3][(last_play.1 + i)%3] != jogador{
                return false;
            }
        }
        return true;
    }

    return checar_vertical(table, last_play, jogador) || checar_horizontal(table, last_play, jogador) || 
    checar_diagonal_direita(table, last_play, jogador) || 
    checar_diagonal_esquerda(table, last_play, jogador);
}

fn main() -> io::Result<()>
{
    //cada valor da tabela tem um sentido
    //0 = está vazio
    //1 = preenchido por X
    //2 = preenchido por O
    let mut table_state: [[u32; 3]; 3] = [[0;3];3];
    print_inicial();
    let mut turno: u32 = 0; 
    let mut buf = String::new();
    let mut turncount: u32 = 1;
    loop{
        print!("É a vez do jogador: ");
        if turno == 0{
            print!("X\n");
        }
        else{
            print!("O\n");
        }
        let mut jogada: u32;
        let indices: (usize, usize);
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
        
        if turncount > 0{
            if checar_vitoria(&table_state, (indices.0, indices.1), turno+1) {
                if turno == 0 {
                    print!("X");
                }
                else{
                    print!("O");
                }
                println!(" é o vencedor!");
                return Ok(());
            }
        }
        if turncount >= 9{
            println!("Deu Velha!");
            return Ok(());
        }
        turncount+=1;
        turno = (turno+1)%2;
        buf.clear();
    }
    // loop{
    //     println!("Turno ")
    // }
}
