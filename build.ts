import { $ } from "jsr:@david/dax@0.43";

await $`cd read && cargo build --release`;
await $`cd write && cargo run`;
await $`./write/output.exe`;