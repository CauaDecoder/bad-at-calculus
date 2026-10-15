# ADR 0002 — Clean Architecture

- **Status:** aceito
- **Data:** 2026-09-28

## Contexto
O projeto terá muitos módulos (cálculo, vetores, física...) e contribuidores humanos e agentes de
IA. Sem fronteiras claras, a lógica matemática tende a vazar para componentes de UI e comandos
Tauri, tornando tudo difícil de testar e de reutilizar entre desktop e web.

## Decisão
Adotar Clean Architecture com quatro camadas (domain, application, interface adapters,
frameworks & drivers), cada camada Rust em uma crate separada, e a **regra de dependência
garantida pelo compilador** via `Cargo.toml`. O front espelha a ideia com `domain/ports/adapters/
application/ui`.

## Alternativas consideradas
- **Crate única com módulos** — mais simples, mas nada impede imports proibidos.
- **Arquitetura hexagonal "pura"** — praticamente equivalente; usamos o vocabulário de Clean
  Architecture por ser o das referências adotadas pelo projeto.

## Consequências
- ✅ Domínio 100% testável sem mocks; casos de uso testáveis com fakes simples.
- ✅ Trocar parser, renderer ou plataforma não afeta as regras matemáticas.
- ⚠️ Mais arquivos e alguma cerimônia (DTOs, traits) — aceitável pelo ganho em clareza.
