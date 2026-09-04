# tilt-demo-loja

Loja de exemplo com **backend em Rust** (`backend/`, axum) e **frontend em TypeScript/Node**
(`frontend/`), cada um com seu Dockerfile. O `tilt.toml` na raiz é a receita de CI/CD que a Tilt
lê: as duas etapas de build começam juntas, em máquinas diferentes; os testes de cada lado rodam
dentro da imagem construída. Ver a spec S-0260 do Tilt Studio.
