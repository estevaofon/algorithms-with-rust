# data-structures-and-algorithms-with-rust

Projeto open source com implementações educacionais de **estruturas de dados** e **algoritmos** em Rust.  

## Motivação

O objetivo deste projeto é servir como material de estudo e referência para desenvolvedores que desejam aprender mais sobre como funcionam internamente essas estruturas de dados em Rust. As implementações são simples e diretas, facilitando a compreensão dos conceitos e servindo de base para futuras extensões.

## Estrutura do Projeto

A estrutura do projeto segue a convenção do Cargo para múltiplos binários:

```bash
data-structures-and-algorithms-with-rust/
├── Cargo.toml
└── src/
    ├── bin/
    │   ├── linked_lists.rs
    │   └── dynamic_array.rs
    └── lib.rs   # (Opcional - para código compartilhado entre os binários)
```

Cada arquivo em `src/bin/` é compilado como um executável separado.

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão 1.XX ou superior)
- Cargo (geralmente instalado juntamente com o Rust)

## Como Executar

Para executar a implementação de **Linked Lists**, utilize o comando:

```bash
cargo run --bin linked_lists
cargo run --bin dynamic_array
```
## Contribuições

Contribuições são muito bem-vindas! Se você deseja melhorar as implementações, corrigir bugs ou sugerir novas estruturas e algoritmos, sinta-se à vontade para:

- Abrir uma *issue* para discutir a sua ideia.
- Fazer um *fork* do repositório.
- Criar uma branch para a sua feature (`git checkout -b feature/minha-nova-feature`).
- Realizar seus commits (`git commit -m 'Adiciona nova feature'`).
- Enviar para a branch (`git push origin feature/minha-nova-feature`).
- Abrir um *Pull Request*.

## Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).

## Contato

Em caso de dúvidas, sugestões ou feedback, por favor, abra uma *issue* no GitHub.

    