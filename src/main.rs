use std::collections::HashMap;

fn media(vetor: &Vec<i32>) -> f32
{
    let mut soma: i32 = 0;
    let quantidade_itens: usize = vetor.len().try_into().unwrap();
    let resultado: f32;

    for item in vetor
    {
        soma += item;
    }

    resultado = soma as f32 / quantidade_itens as f32;
    return resultado;
}

fn mediana(vetor: &Vec<i32>) -> f32
{
    let mut vetor_sorted = vetor.clone();
    let mediana: f32;
    let indice: usize;

    vetor_sorted.sort();
    
    indice = vetor_sorted.len() / 2;

    if vetor_sorted.len() % 2 == 0
    {
        mediana = media(&vec![vetor_sorted[indice - 1], vetor_sorted[indice]]);
    }
    else
    {
        mediana = vetor_sorted[indice] as f32;
    }
    return mediana;
}

fn moda(vetor: &Vec<i32>) -> i32
{
    let mut map = HashMap::new();
    for item in vetor
    {
        let contador = map.entry(item).or_insert(0);
        *contador += 1;
    }

    let mut maior_valor: i32 = 0;
    let mut maior_key: i32 = 0;
    for (key, value) in map
    {
        if value > maior_valor
        {
            maior_valor = value;
            maior_key = *key;
        }
    }
    return maior_key;
}

fn soma_itens(vetor: &Vec<i32>) -> i32
{
    let mut soma: i32 = 0;
    for item in vetor
    {
        soma += item;
    }
    return soma;
}

fn main()
{
    {
        let valores: Vec<i32> = vec![8, 7, 3, 1, 5, 7, 4, 9, 2];

        println!("Meu vetor: {:?}", valores);
        println!("Quantidade de itens: {}", valores.len());
        println!("Soma dos itens: {}", soma_itens(&valores));
        println!("Média dos itens: {}", media(&valores));
        println!("Mediana: {}", mediana(&valores));
        println!("Moda: {}", moda(&valores));

        let mut valores_sorted = valores.clone();
        valores_sorted.sort();
        println!("Meu vetor ordenado: {:?}\n", valores_sorted);

    }

    {
        let valores: Vec<i32> = vec![4, 2, 1, 5, 0, 4, 1, 3, 6, 4];

        println!("Meu vetor: {:?}", valores);
        println!("Quantidade de itens: {}", valores.len());
        println!("Soma dos itens: {}", soma_itens(&valores));
        println!("Média dos itens: {}", media(&valores));
        println!("Mediana: {}", mediana(&valores));
        println!("Moda: {}", moda(&valores));
        
        let mut valores_sorted = valores.clone();
        valores_sorted.sort();
        println!("Meu vetor ordenado: {:?}", valores_sorted);
    }
}
