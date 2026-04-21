# Model 

-  O model é a planta baixa do seu dado - define a forma, os campos
e os tipos antes de qualquer coisa acontecer.

O model responde só uma pergunta:

> **"Como esse dado é?"** — e nada mais.

````
#[derive(Debug, Deserialize, Serialize)]
pub struct User{
    pub name: string,
    pub age: u32,
    pub email: string
} 
````

- derive -> macro que implementa funcionalidades automaticamente para suas struct

````
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment{ ... }
````

- Debug -> vem da biblioteca padrão do rust (std) e permite imprimir a struct no terminal

````
let payment_data = Payment{...}
println!("{:?}", payment_data)

// Sem Debug isso daria erro de compilação!
````

- Serialize / Deserialize -> vem da biblioteca ´Serde´ e servem 
para conversão de dados

  Serialize   = Rust -> JSON (para enviar dados) {converte um valor em Rust para um formato externo (como JSON, YAML, TOML, BSON, etc.)}

  ### Serialize - quando sua API retorna um pagamento:
  ````
  Payment { id: 1, amount: 99.90, ... }
        ↓  (Serialize)
  { "id": 1, "amount": 99.90, ... }   // JSON enviado ao cliente
  ````

  Deserialize = JSON → Rust (para receber dados) {converte de um formato externo (como JSON) para um tipo Rust.}

  ### Deserialize — quando sua API recebe um pagamento:
  ````
  { "id": 1, "amount": 99.90, ... }   // JSON recebido do cliente
        ↓  (Deserialize)
  Payment { id: 1, amount: 99.90, ... }
  ````

- FromRow -> Vem do **SQLx** — a biblioteca de banco de dados e converter uma linha retornada do banco → em uma struct Rust

````
ex sem FromRow
#[derive(FromRow)]
struct Payment {
    id: i32,
    amount: f64,
    currency: String,
}

let row = sqlx::query("SELECT id, amount, currency FROM payments")
    .fetch_one(&pool)
    .await?;

let payment = Payment {
    id: row.get("id"),
    amount: row.get("amount"),
    currency: row.get("currency"),
};

_________________________________________________________________________
ex com FromRow

#[derive(FromRow)]
struct Payment {
    id: i32,
    amount: f64,
    currency: String,
}

let payment: Payment = sqlx::query_as("SELECT id, amount, currency FROM payments")
    .fetch_one(&pool)
    .await?;

````

NaiveDateTime, Utc -> vem do "chrono" trabalham com manipulação de data e hora

- NaiveDateTime ->  data + hora SEM fuso horário

````
ex:
   2024-03-15 14:30:00
pub created_at: NaiveDateTime,  // quando o pagamento foi criado
pub updated_at: NaiveDateTime,  // quando foi atualizado por último
````

- Utc -> let agora = Utc::now();  // pega a data/hora atual em UTC

---
## Resumo visual

```
#[derive(
  Debug,        // consigo imprimir no terminal
  Serialize,    // consigo converter de Rust -> JSON
  Deserialize,  // consigo converter de JSON -> Rust
  FromRow,      // consigo converter linha do banco → Rust 
)]
pub struct Payment{
    pub uuid: Uuid,
    pub created_at: NaiveDateTime, 
    pub updated_at: NaiveDateTime, 
}
```
---