# Taxonomia

Como o conteúdo é classificado. Inspirada na hierarquia curso → capítulo → seção da biblioteca
aberta do WeBWorK e nas habilidades do OATutor ([pesquisa](../09-pesquisa/plataformas-de-referencia.md)).

```
Área            exatas · computacao · arquitetura
└─ Disciplina   calculo, vga, fisica-1, poo, algoritmos, banco-de-dados, estruturas-1, geometria-descritiva …
   └─ Tópico    derivadas, limites, vetores, heranca, sql-joins, forcas-aplicadas …
      └─ Habilidade   regra-da-cadeia, reta-tangente, produto-vetorial, sobrescrita, reacoes-de-apoio …
```

## Regras

- **Identificadores** em `kebab-case`, sem acento, em PT-BR, com o caminho completo:
  `calculo.derivadas.regra-da-cadeia`. O título com acento fica no arquivo de taxonomia.
- **Exercícios** apontam para **habilidades** (1 a 4 por exercício). Disciplina e tópico são
  derivados do caminho.
- **Visualizações** apontam para **tópicos**.
- Uma habilidade pertence a um único tópico. Habilidades usadas por várias disciplinas (vetores em
  VGA, Física e Estruturas) ficam na disciplina de origem e são **referenciadas** pelas outras
  (`requires`).
- Disciplinas não são cursos: "Cálculo A" da UFRGS e "Cálculo I" de outra universidade usam a mesma
  disciplina `calculo`. A instituição fica na `source.citation` da lista.

## Arquivo de taxonomia (`content/taxonomy/*.yaml`)

```yaml
area: exatas
discipline: calculo
title: Cálculo
topics:
  - id: derivadas
    title: Derivadas
    skills:
      - { id: definicao, title: Derivada pela definição }
      - { id: reta-tangente, title: Reta tangente }
      - { id: regra-do-produto, title: Regra do produto }
      - { id: regra-do-quociente, title: Regra do quociente }
      - { id: regra-da-cadeia, title: Regra da cadeia }
      - { id: derivadas-de-ordem-superior, title: Derivadas de ordem superior }
      - { id: pontos-criticos, title: Pontos críticos e tangente horizontal }
```

## Mapa inicial

| Área | Disciplina | Tópicos iniciais |
| ---- | ---------- | ---------------- |
| exatas | `calculo` | funcoes, limites, continuidade, derivadas, integrais |
| exatas | `vga` | vetores, retas-e-planos, matrizes, sistemas-lineares, conicas |
| exatas | `fisica-1` | cinematica, dinamica, trabalho-e-energia |
| exatas | `metodos-numericos` | raizes, edos, erro |
| computacao | `poo` | paradigmas, classes-e-objetos, encapsulamento, relacionamentos, heranca, polimorfismo |
| computacao | `algoritmos` | ordenacao, busca, estruturas-lineares, arvores, complexidade |
| computacao | `banco-de-dados` | modelagem-er, sql-basico, joins, agregacao |
| arquitetura | `estruturas-1` | forcas-aplicadas, equilibrio, reacoes-de-apoio, trelicas, esforcos-internos, sistemas-estruturais |
| arquitetura | `geometria-descritiva` | epura, verdadeira-grandeza, secoes |

O conteúdo que motivou cada disciplina (listas, apostilas e slides de professores) fica em
`content-private/`, fora do git ([ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
