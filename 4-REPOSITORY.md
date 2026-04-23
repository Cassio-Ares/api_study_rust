# Repository
- fala com banco (todo o SQL fica aqui)

## SQLx (query, query_as<>, query_as!)

- query -> retorna PgRow - uma linha crua do banco.
           Você tem que extrair cada campo MANUALMENTE.

````
// query — linha crua, extração manual
let row = query("SELECT * FROM payments")
    .fetch_one(pool).await?;
    let amount: f64 = row.get("amount");
    let currency = row.get("currency");
````

- query_as<> -> query_as já converte a linha para Payment 
                automaticamente, sem verificação no build

  ✅ Erro pego DURANTE execução;

  ✅ Mais flexível para SQL dinâmico

  ✅ Funciona em qualquer ambiente

  ❌ Erro de SQL só aparece em tempo de execução

  ❌ Você mesmo garante que os tipos batem 

````
let payment = query_as::<Postgres, Payment>(
  "SELECT * FROM payment WHERE uuid = $1"
)
.bind(uuid)
.fetch_one(pool)
.await?;  // já é um Payment pronto!
````

- query_as! -> tem tudo que o query_as tem, mais a segurança da verificação em tempo de compilação. (O compilador CONECTA no banco durante o "cargo build" e verifica se o SQL está correto antes de rodar)


 ✅ (com !) — Macro com verificação em tempo de compilação

 ✅  Tipos dos campos verificados automaticamente  

 ✅  Erro aparece antes de rodar o programa

 ❌  Precisa de banco de dados disponível no cargo build

 ❌  Mais difícil em CI/CD e ambientes sem banco

 ````
 // query_as! — converte para Payment + verifica SQL em tempo de compilação

 let payment = query_as!(Payment, "SELECT * FROM payments")
    .fetch_one(pool).await?; // Payment pronto + SQL seguro ✅✅
 ````

### Sintaxe diferença de query_as<> para query_as!

- query_as<>

A função recebe o tipo via turbofish `::<>` e valores via `.bind()`

````
query_as::<Postgress, Payment>(  <- tipagem no turbofish
    "SELECT * FROM payments WHERE uuid = $1"  <- query
)
.bind(uuid) <- valores
.fetch_one(pool)
.await?;
````

- query_as! -> tudo é recebido com argumento, (typagem, query, valore)

````
query_as!(
  Payment, 
  "SELECT * FROM payments WHERE uuid = $1",  <- query
  uuid  <- valores 
)
.fetch_one(pool)
.await?;
````

## Importante Result 

- Result vem da biblioteca padrão do Rust (std) — e é tão fundamental que não precisa importar, o Rust já traz automático.Result é um enum com dois estados possíveis, OK e Error.

````
pub async fn create_pauyment_repository(
  pool: &PgPool, 
  new_payment:NewPayment,
) -> Result<Payment, sqlx::Error>
````

## Repository

````
// PgPool -> pool de conexão
// Postgres -> tipo de banco
// query_as -> função de query
use sqlx::{PgPool, Postgres, query_as};

//uuid -> uuid v4
use uuid::Uuid;

// Chrono -> biblioteca de datas e horários do Rust. Utc é o fuso horário universal.
use chrono::Utc;

// Importa todos os models necessários para este arquivo
use crate::models::payment::{Payment, NewPayment, PaymentMethod, PaymentStatus, UpdatePayment, UpdatePaymentStatus};

pub async fn create_payment(
  pool:&PgPool, 
  new_payment:NewPayment
) -> Result<Payment, sqlx::Error>{
  let payment = query_as::<Postgres, Payment>(
    "INSERT INTO payment(
        uuid, amount, currency, payment_method, payment_reason, status, created_at, updated_at
      ) VALUES(
        $1, $2, $3, $4, $5, $6, $7, $8
      ) RETURNING *"
  )
  .bind(Uuid::new_v4())              // $1 — gera um UUID aleatório 
  .bind(new_payment.amount)          // $2 — valor do pagamento
  .bind(new_payment.currency)        // $3 — moeda (ex: "BRL")
  .bind(new_payment.payment_method as PaymentMethod)  // $4 — converte enum para o tipo do banco
  .bind(new_payment.payment_reason)  // $5 — motivo do pagamento
  .bind(new_payment.status as PaymentStatus)          // $6 — converte enum para o tipo do banco
  .bind(Utc::now().naive_utc())      // $7 — data/hora atual como created_at
  .bind(Utc::now().naive_utc())      // $8 — data/hora atual como updated_at
  .fetch_one(pool) //executa a espera exatamente 1 resultado
  .await?   // aguarda a query assíncrona — `?` propaga erro se falhar

  OK(payment) // retorna o payment criado
}

pub async fn get_all_payment( pool:&PgPool
   ) -> Result<Vec<Payment>, sqlx::Error>{
    let payments = query_as::<Postgres, Payment>(
      "SELECT * FROM payments" // busca todos os pagamentos
    )
    .fetch_all(pool) // retorna TODOS os resultados como Vec<Payment>
    .await?;

    ok(payments)
   }

pub async fn get_payment_by_uuid(pool:&PgPool, uuid:Uuid)->
   Result<Option<Payment>, sqlx::Error>{
     let payment = query_as::<Postgres, Payment>(
        "SELECT * FROM payments WHERE uuid = $1" // busca pelo uuid específico
    )
    .bind(uuid)               // $1 — o uuid que veio como parâmetro
    .fetch_optional(pool)     // retorna Option<Payment> — Some(payment) se achou, None se não achou
    .await?;

    Ok(payment)
   }
````

## Tipo de Fetch e função 

- `.fetch_one(pool).await?` ->
   Usado para buscar por Id/Uuid quando você tem certeza que existe.

  ✅ achou 1 retorn valor

  ❌ achou 0 Error RowNotFound

  ❌ achou 2 ou mais Erro

- `.fetch_optional(pool).await?` ->
  Usado para buscar algo que talvez não exista.

  ✅ achou 1 retorna Some(modelo do valor)

  ✅ achou 0 retorna None (sem erro) 

- `.fetch_all(pool).await` -> 
  Usado para busca de lista de dados

  ✅ achou N → retorna Vec<Payment>

  ✅ achou 0 → retorna Vec vazio [] (sem erro)

- `execute(pool).await?` ->
  Usado em DELETE, UPDATE sem RETURNING
  
  ✅ execute retorna PgQueryResult com linhas afetadas, não retorna os dados do banco — ele retorna apenas uma confirmação de que a operação aconteceu.

  ✅ execute retorna só uma confirmação com um número de linhas afetadas

```
// DELETE
let result = query("DELETE FROM payments WHERE uuid = $1")
    .bind(uuid)
    .execute(pool).await?;

result.rows_affected()
// → 1  = encontrou e deletou ✅
// → 0  = não encontrou nada (uuid não existe) ⚠️


// UPDATE sem RETURNING
let result = query("UPDATE payments SET status = $1 WHERE uuid = $2")
    .bind(status)
    .bind(uuid)
    .execute(pool).await?;

result.rows_affected()
// → 1  = encontrou e atualizou ✅
// → 0  = uuid não existe ⚠️
```

- `.fetch(pool)` ->

Use em: volumes enormes de dados (milhões de linhas)
  IMPORTANTE não retorna Vec, retorna um Stream e é nescessário processar 
  linha por linha com .next()
  while let Some(payment) = stream.next().await?

 ✅ Processa linha por linha conforme chega

```
use futures::StreamExt; // precisa importar isso

// fetch retorna um Stream — não uma Promise
// então você não faz .await direto nele
let mut stream = query_as::<Postgres, Payment>(
    "SELECT * FROM payments"
)
.fetch(pool); // ← sem .await aqui!

// Você processa linha por linha com .next()
while let Some(payment) = stream.next().await? {
    // processa um payment por vez
    // nunca carrega tudo na memória
    println!("{:?}", payment);
}
```


## RESUMO 

Regra geral de qual usar em cada operação:

| Operação      | SQL               | Uso                              |
|--------------|------------------|----------------------------------|
| Criar        | INSERT           | fetch_one ou execute            |
| Buscar um    | SELECT WHERE uuid = $1 | fetch_one ou fetch_optional |
| Buscar todos (lista pequena) | SELECT *          | fetch_all                      |
| Buscar todos (milhões) | SELECT *          | fetch                        |
| Atualizar    | UPDATE           | fetch_one ou execute            |
| Deletar      | DELETE           | execute                        |

Resumo: fetch é uma ferramenta para casos específicos de grande volume — no dia a dia de uma API de pagamentos, fetch_all, fetch_one e fetch_optional resolvem praticamente tudo.
