# 💳 Payment Rust — Um Projeto de Estudo

> **Um guia prático para aprender Rust e Actix-web do zero**, construindo uma API de pagamentos simples com explicações em cada passo.

---

## 📚 O que é este projeto?

Este é um **projeto educacional** desenhado para iniciantes em Rust que desejam aprender:

- ✅ **Fundamentos de Rust**: sintaxe, tipos, traits, tratamento de erros
- ✅ **Actix-web**: framework web assíncrono de alta performance
- ✅ **Arquitetura limpa**: separação em camadas (Model → Repository → Service → Handler)
- ✅ **Banco de dados**: integração com PostgreSQL usando SQLx
- ✅ **Validação**: regras de negócio com a biblioteca `validator`
- ✅ **Testes**: testes de integração com `tokio::test`

**O código é simples, mas cada decisão é explicada**. Não é um projeto "production-ready", é um projeto **didático**. Cada README complementa o anterior com conceitos, justificativas e melhorias possíveis.

---

## 🗂️ Arquitetura dos READMEs

Os READMEs são progressivos — cada um constrói sobre o anterior. Comece do 1 e vá até o 6.1:

### **Fase 1: Setup do Projeto**

| README                                                                               | Foco              | O que você aprende                                                                                                           |
| ------------------------------------------------------------------------------------ | ----------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **[1-INIT_PROJECT.md](1-INIT_PROJECT.md)**                                           | 🐳 Docker + Cargo | Como iniciar um projeto Rust com Docker, estrutura básica do Cargo.toml, primeiro `main.rs` com Actix-web e a macro `#[get]` |
| **[2-DOCKERCOMPOSE_E_CONEXÂO_COM_BANCO.md](2-DOCKERCOMPOSE_E_CONEXÂO_COM_BANCO.md)** | 🔗 Banco de dados | Docker Compose para PostgreSQL, pool de conexões, variáveis de ambiente (.env), conexão com SQLx                             |

### **Fase 2: Arquitetura em Camadas**

| README                                 | Foco         | O que você aprende                                                                                                  |
| -------------------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------- |
| **[3-MODEL.md](3-MODEL.md)**           | 📦 Models    | Structs de dados, derives (`Debug`, `Serialize`, `Deserialize`, `FromRow`), DTOs (NewPayment, UpdatePayment), enums |
| **[3.1-VALIDATE.md](3.1-VALIDATE.md)** | ✔️ Validação | Como implementar o trait `Validate`, regras condicionais, erros por campo, por que validar na entrada               |

### **Fase 3: Banco de Dados (Repository Pattern)**

| README                                 | Foco          | O que você aprende                                                                                                     |
| -------------------------------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **[4-REPOSITORY.md](4-REPOSITORY.md)** | 🗄️ Repository | Camada de acesso a dados, funções CRUD (`create`, `get_all`, `get_by_uuid`), SQLx queries, `query_as<>` vs `query_as!` |

### **Fase 4: Lógica de Negócio (Service Layer)**

| README                               | Foco       | O que você aprende                                                                                                   |
| ------------------------------------ | ---------- | -------------------------------------------------------------------------------------------------------------------- |
| **[4.1-SERVICE.md](4.1-SERVICE.md)** | ⚙️ Service | Por que existem services, orquestração (validação → repository), tratamento de erros com `Result<T, Box<dyn Error>>` |

### **Fase 5: HTTP (Handler Layer) + Rotas**

| README                           | Foco        | O que você aprende                                                                  |
| -------------------------------- | ----------- | ----------------------------------------------------------------------------------- |
| **[5-HANDLER.md](5-HANDLER.md)** | 🌐 Handlers | Rotas HTTP, extractors do Actix-web, respostas HTTP, tratamento de erros no handler |
| **[5.1-ROUTE.md](5.1-ROUTE.md)** | 🛣️ Rotas    | Registro de rotas, aninhamento de recursos, mapeamento URLs para handlers           |

### **Fase 6: Main + Testes**

| README                           | Foco      | O que você aprende                                                                                  |
| -------------------------------- | --------- | --------------------------------------------------------------------------------------------------- |
| **[6-MAIN.md](6-MAIN.md)**       | 🚀 Main   | Entrada da aplicação, setup do servidor, registro de módulos                                        |
| **[6.1-TESTS.md](6.1-TESTS.md)** | 🧪 Testes | Testes de integração com `#[tokio::test]`, setup de banco para testes, asserts, estrutura de testes |

---

## 🏗️ Estrutura do Projeto

```
api_study_rust/
├── README.md                    ← Você está aqui!
├── 1-INIT_PROJECT.md            ← Comece daqui
├── 2-DOCKERCOMPOSE_E_CONEXÂO_COM_BANCO.md
├── 3-MODEL.md
├── 3.1-VALIDATE.md
├── 4-REPOSITORY.md
├── 4.1-SERVICE.md
├── 5-HANDLER.md
├── 5.1-ROUTE.md
├── 6-MAIN.md
├── 6.1-TESTS.md
│
├── Cargo.toml                   ← Dependências e configuração
├── .env.example                 ← Template de variáveis de ambiente
├── .env                         ← Seu .env local (não versionar)
│
├── docker/
│   ├── Dockerfile              ← Build da aplicação
│   ├── docker-compose.yml       ← PostgreSQL + App
│   └── init.sql                 ← Schema do banco
│
├── src/
│   ├── main.rs                  ← Entrada da aplicação, setup do servidor
│   │
│   ├── model/                   ← Structs de dados (o "o quê")
│   │   ├── mod.rs
│   │   └── payment_model.rs     ← Payment, NewPayment, PaymentStatus...
│   │
│   ├── validate/                ← Regras de validação
│   │   ├── mod.rs
│   │   ├── new_payment_validate.rs ← Validação de NewPayment
│   │   ├── updatePayment.rs
│   │   └── updatePaymentStatus.rs
│   │
│   ├── repository/              ← Banco de dados (o "como armazenar")
│   │   ├── mod.rs
│   │   └── payment_repository.rs ← CRUD com SQL
│   │
│   ├── service/                 ← Lógica de negócio (o "por que")
│   │   ├── mod.rs
│   │   └── payment_service.rs   ← Orquestra validação + repository
│   │
│   ├── handler/                 ← Rotas HTTP (o "como expor")
│   │   ├── mod.rs
│   │   └── payment_handler.rs   ← Endpoints GET/POST/PUT/PATCH
│   │
│   ├── routes/                  ← Registro de rotas
│   │   ├── mod.rs
│   │   └── payment_routes.rs    ← Mapeia URLs para handlers
│   │
│   └── tests/                   ← Testes de integração
│       ├── mod.rs
│       ├── common.rs            ← Setup compartilhado entre testes
│       ├── repository_test.rs   ← Testa camada de banco
│       ├── service_test.rs      ← Testa lógica de negócio
│       └── validation_test.rs   ← Testa validações
```

---

## 🎯 Como Usar Este Projeto para Aprender

### **Passo 1: Leia os READMEs em Ordem**

Cada README é auto-contido mas progressivo. **Não pule etapas**:

1. Comece com [1-INIT_PROJECT.md](1-INIT_PROJECT.md) — entenda o setup
2. Avance até [6.1-TESTS.md](6.1-TESTS.md) — finalize com testes

### **Passo 2: Entenda o Fluxo de uma Requisição**

Uma requisição POST `/payment` passa por:

```
Cliente HTTP
    ↓
[Handler] payment_handler.rs
    ↓ (extrai JSON e passa para Service)
[Service] payment_service.rs
    ↓ (valida com Validator + chama Repository)
[Validator] new_payment_validate.rs
    ↓ (regras de negócio)
[Repository] payment_repository.rs
    ↓ (executa SQL)
PostgreSQL
    ↓
Response JSON
```

**Por que essa separação?** Cada camada tem responsabilidade única:

- **Model**: define a forma dos dados
- **Validator**: garante que dados a entrada estão corretas
- **Repository**: acessa o banco, sem lógica de negócio
- **Service**: orquestra e implementa regras (ex: "não refundar mais que pagou")
- **Handler**: conversa com HTTP, sem conhecer SQL

### **Passo 3: Modifique e Experimente**

- Tente adicionar um novo campo a `Payment`
- Crie uma nova validação
- Escreva um novo teste

Cada README tem seções "✨ Melhorias Possíveis" para desafiá-lo.

---

## 🚀 Quick Start

### Pré-requisitos

- Docker e Docker Compose instalados
- Rust (opcional, se rodar via Docker)

### Rodar o Projeto

```bash
# 1. Clone este repositório
git clone https://github.com/Cassio-Ares/api_study_rust.git
cd api_study_rust

# 2. Crie seu .env
cp .env.example .env
# Edite .env com suas credenciais

# 3. Suba o banco de dados
docker compose -f docker/docker-compose.yml up -d

# 4. Rode a aplicação
cargo build
cargo run

# 5. Teste uma requisição
curl -X POST http://127.0.0.1:8080/payment \
  -H "Content-Type: application/json" \
  -d '{"amount": 100.0, "currency": "USD", "payment_method": "CreditCard", "payment_reason": "Test", "status": "Completed"}'
```

### Rodar Testes

```bash
# Configure a variável de ambiente para testes
export DATABASE_TEST_URL="postgresql://user:password@localhost:5432/payment_test_db"

# Execute os testes
cargo test
```

---

## 📖 Recursos Educacionais em Cada README

Cada README segue este padrão:

1. **Conceito**: O que você vai aprender e por que
2. **Código de Exemplo**: Exemplos práticos com comentários
3. **Explicação Linha por Linha**: Se é complexo, explicamos cada linha
4. **Justificativa**: Por que fazemos assim (não apenas "o como")
5. **✨ Melhorias Possíveis**: Próximos passos para aprofundar

Exemplo do [3.1-VALIDATE.md](3.1-VALIDATE.md):

```markdown
## Por que Validar na Entrada?

Se não validar, o banco recebe dados inválidos, histórico fica sujo,
e outros sistemas que consomem a API recebem lixo.

Validar NO HANDLER garante que nunca chega lixo no banco.
```

---

## 🎓 Conceitos-chave Explicados

Todos esses conceitos estão nos READMEs, mas em resumo:

| Conceito         | Onde Aprender                                                                    | Por que Importa                                                  |
| ---------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **Traits**       | [3-MODEL.md](3-MODEL.md)                                                         | Reutilização de comportamento (Serialize, Deserialize, Validate) |
| **Result<T, E>** | [4.1-SERVICE.md](4.1-SERVICE.md)                                                 | Tratamento de erros sem exceções                                 |
| **Async/Await**  | [2-DOCKERCOMPOSE_E_CONEXÂO_COM_BANCO.md](2-DOCKERCOMPOSE_E_CONEXÂO_COM_BANCO.md) | APIs rápidas que usam threads eficientemente                     |
| **Extractors**   | [5-HANDLER.md](5-HANDLER.md)                                                     | Actix injeta dados (JSON, Path, Pool) automaticamente            |
| **Camadas**      | Vários                                                                           | Código testável, manutenível, escalável                          |

---

## 🤝 Contribuições & Sugestões

Este projeto é **para aprender**, então se quiser:

- ✅ Sugerir melhorias didáticas
- ✅ Avisar sobre erros nos READMEs
- ✅ Indicar tópicos faltando

Abra uma issue ou um PR. Vamos melhorar juntos!

---

## 📝 Licença

Este projeto é fornecido como material educacional. Sinta-se livre para usar, modificar e estudar.

---

## 🎬 Comece Agora!

👉 **[Vá para 1-INIT_PROJECT.md](1-INIT_PROJECT.md)** e comece sua jornada em Rust!

---

**Última atualização**: Abril 2026  
**Status**: Projeto de estudo básico concluído até testes de integração
