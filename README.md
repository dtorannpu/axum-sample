# axum-sample

## DBマイグレーションの追加

```bash
cargo make sqlx migrate add -r sample
```

## DBマイグレーションの実行

```bash
cargo make sqlx migrate run
```

## DBマイグレーションの戻し

```bash
cargo make sqlx migrate revert
```
