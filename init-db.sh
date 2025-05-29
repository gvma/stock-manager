#!/bin/bash
cargo install sqlx-cli

for dir in stock auth; do
  echo "Configurando banco de dados em $dir"
  cd "$dir"
  sqlx database setup
  cd ..
done

echo "Setup completo."
