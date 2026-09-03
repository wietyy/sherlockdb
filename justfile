dev:
    @rustc demo.rs
    @./demo

test:
    @cargo test

ship commit:
    @git add .
    @git commit -m "{{commit}}"
    @git push