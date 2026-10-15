# Banco público de exercícios

Exercícios, listas e taxonomia **abertos** da Vértice, licenciados em [CC BY-SA 4.0](LICENSE).
Ainda vazio: o compilador e a validação chegam na F1 ([roadmap](../docs/02-produto/roadmap.md)).

```
content/
  taxonomy/<area>/<disciplina>.yaml     áreas, disciplinas, tópicos e habilidades
  exercises/<disciplina>/<topico>/*.ex.md
  lists/<disciplina>/*.yaml
  assets/<disciplina>/...
```

- Formato: [docs/02-produto/formato-de-exercicios.md](../docs/02-produto/formato-de-exercicios.md).
- Taxonomia: [docs/02-produto/taxonomia.md](../docs/02-produto/taxonomia.md).
- Só conteúdo **autoral** ou de fonte aberta compatível. Listas, apostilas e slides de professores
  ficam em `content-private/` (ignorado pelo git) e entram no app como pack privado
  ([ADR 0012](../docs/03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
